use crate::player::RustPlayer;
use godot::classes::{INode3D, InputEvent, InputEventMouseMotion, Node3D};
use godot::classes::input::MouseMode;
use godot::global::deg_to_rad;
use godot::obj::{Base, WithBaseField};
use godot::prelude::*;

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
#[class(init, rename=RustPlayerInputComponent, base=Node3D)]
pub struct PlayerInputComponent {
    input: Gd<Input>,

    #[var]
    is_mouse_captured: bool,
    #[var]
    is_moving: bool,
    base: Base<Node3D>
}

#[godot_api]
impl INode3D for PlayerInputComponent {
    fn ready(&mut self) {
        self.input = Input::singleton();
        self.input.set_mouse_mode(MouseMode::CAPTURED);
        self.set_is_mouse_captured(true);
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