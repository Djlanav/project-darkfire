use std::thread::JoinHandle;
use crossbeam::channel::unbounded;
use crate::player::RustPlayer;
use godot::classes::{INode3D, InputEvent, InputEventMouseMotion, Node3D, Time, Viewport, ViewportTexture};
use godot::classes::input::MouseMode;
use godot::global::deg_to_rad;
use godot::meta::ParamType;
use godot::obj::{Base, WithBaseField};
use godot::prelude::*;
use crate::callable_method;

const MAX_THREADS: u32 = 5;

#[derive(GodotClass)]
#[class(init, rename=RustHealthComponent, base=Node3D)]
pub struct HealthComponent {
    #[export]
    max_health: i64,
    #[var]
    health: i64,
    base: Base<Node3D>
}

#[godot_api]
impl INode3D for HealthComponent {
    fn process(&mut self, delta: f64) {
        if self.health <= 0 {
            self.base_mut().emit_signal("health_zero", &[]);
        }
    }
    
    fn ready(&mut self) {
        self.health = self.max_health;
    }
}

#[godot_api]
impl HealthComponent {
    // Signals
    #[signal]
    pub fn health_zero();
    // End Signals

    #[func]
    fn on_player_take_damage(&mut self, damage: i64) {
        self.health -= damage;
    }
}

#[derive(GodotClass)]
#[class(rename=RustPlayerInputComponent, base=Node3D)]
pub struct PlayerInputComponent {
    input: Gd<Input>,
    threads: Vec<JoinHandle<()>>,

    #[var]
    is_mouse_captured: bool,
    #[var]
    is_moving: bool,
    base: Base<Node3D>
}

#[godot_api]
impl INode3D for PlayerInputComponent {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            input: Input::singleton(),
            threads: Vec::new(),
            is_mouse_captured: false,
            is_moving: false,
            base
        }
    }

    fn process(&mut self, delta: f64) {
        if self.input.is_action_just_pressed("take_screenshot") {
            self.base_mut().call_deferred("take_screenshot", &[]);

        } else if self.input.is_action_just_pressed("debug quit") {
            self.on_exiting_tree();
            self.base().get_tree().unwrap().quit();
        }
    }

    fn ready(&mut self) {
        self.input.set_mouse_mode(MouseMode::CAPTURED);
        self.set_is_mouse_captured(true);

        let self_gd = self.to_gd();
        self.base_mut().connect(
            "tree_exiting",
            &callable_method!(&self_gd, "on_exiting_tree"));
    }
}

#[godot_api]
impl PlayerInputComponent {
    #[inline]
    fn join_threads(&mut self) {
        godot_print!("Joining {} threads...", self.threads.len());

        for thread in self.threads.drain(..) {
            if thread.is_finished() {
                match thread.join() {
                    Ok(_) => godot_print!("Successfully joined thread"),
                    Err(e) => godot_error!("Error joining thread"),
                }
            }
        }
    }

    #[func]
    fn on_exiting_tree(&mut self) {
        self.join_threads();
    }

    #[func]
    fn take_screenshot(&mut self) {
        let viewport_id = self
            .base()
            .get_viewport()
            .expect("Failed to get viewport")
            .get_texture().unwrap()
            .instance_id();

        let thread_handle = std::thread::spawn(move || {
            let viewport_image: Gd<ViewportTexture> = Gd::from_instance_id(viewport_id);
            let time = Time::singleton().get_datetime_string_from_system();
            let path = format!("user://screenshot_{}", time);

            viewport_image
                .get_image()
                .unwrap()
                .save_png(GString::from(path).owned_to_arg());
        });


        if self.threads.len() > MAX_THREADS as usize {
            self.join_threads();
        } else {
            self.threads.push(thread_handle);
        }
    }
}

#[derive(GodotClass)]
#[class(init, base=Camera3D)]
pub struct FirstPersonCamera {
    player: Option<Gd<RustPlayer>>,
    camera_sensitivity: f64,
    base: Base<Camera3D>,
}

#[godot_api]
impl ICamera3D for FirstPersonCamera {
    fn ready(&mut self) {
        let parent = self.base()
            .get_parent()
            .unwrap()
            .cast::<RustPlayer>();
        self.player = Some(parent);

        self.camera_sensitivity = 0.3;
    }

    fn unhandled_input(&mut self, event: Gd<InputEvent>) {
        if let Ok(mouse_motion) = event.try_cast::<InputEventMouseMotion>() {
            let player = self.player.as_mut().unwrap();
            player
                .rotate_y((
                    deg_to_rad(-mouse_motion.get_relative().x as f64)
                        * self.camera_sensitivity) as f32)
        }
    }
}