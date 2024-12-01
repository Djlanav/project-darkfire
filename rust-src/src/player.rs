use std::ops::{Add, Deref};
use crate::{components::HealthComponent, components::PlayerInputComponent, audio_handling::SFXManager,
            callable_method};
use godot::classes::{CharacterBody3D, ICharacterBody3D, InputMap, RayCast3D, RigidBody3D, SpotLight3D, VoxelGi, Window};
use godot::global::move_toward;
use godot::prelude::*;
// pub enum PlayerState {
//     IDLE,
//     MOVING,
//     FALLING,
// }
//
// impl Default for PlayerState {
//     fn default() -> Self {
//         Self::IDLE
//     }
// }

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
pub struct RustPlayer {
    base: Base<CharacterBody3D>,

    // @onready vars
    player_camera: OnReady<Gd<Camera3D>>,
    health_component: OnReady<Gd<HealthComponent>>,
    input_component: OnReady<Gd<PlayerInputComponent>>,
    sfx_man: OnReady<Gd<SFXManager>>,
    pick_up: OnReady<Gd<RayCast3D>>,
    hold_point: OnReady<Gd<Node3D>>,
    flashlight: OnReady<Gd<SpotLight3D>>,
    console: OnReady<Gd<Window>>,
    // @onready end

    #[var]
    speed: i64,
    #[var]
    jump_velocity: f64,
    #[var]
    inertia: f64,
    #[var]
    is_wasd: bool,
    #[var]
    held_object: Option<Gd<RigidBody3D>>,
    #[var]
    last_held_position: Vector3,
    #[var]
    current_map: Option<Gd<Node>>,
    input_singleton: Gd<Input>,
    input_map: Gd<InputMap>,
}

#[godot_api]
impl ICharacterBody3D for RustPlayer {
    fn init(base: Base<CharacterBody3D>) -> Self {
        Self {
            input_singleton: Input::singleton(),
            input_map: InputMap::singleton(),

            player_camera: OnReady::node("PlayerCamera"),
            health_component: OnReady::node("RustHealthComponent"),
            input_component: OnReady::node("RustPlayerInputComponent"),
            sfx_man: OnReady::node("SFXComp"),
            pick_up: OnReady::node("PickUpRange"),
            hold_point: OnReady::node("HoldPoint"),
            flashlight: OnReady::node("Flashlight"),
            console: OnReady::node("ConsoleWindow"),

            speed: 5,
            jump_velocity: 4.5,
            inertia: 80.0,
            is_wasd: false,
            held_object: None,
            last_held_position: Vector3::ZERO,
            current_map: None,
            base,
        }
    }

    fn process(&mut self, delta: f64) {
        if let Some(held_object) = self.held_object.as_mut() {
            if held_object.is_instance_valid() {
                self.last_held_position = held_object.get_global_position();
                let held_obj_global_position = held_object.get_global_position();
                let hold_point_global_position = self.hold_point.get_global_position();

                held_object.set_global_position(
                    Vector3::lerp(held_obj_global_position,
                                  hold_point_global_position,
                                  0.2)
                );
            }
        }

        if self.input_singleton.is_action_just_pressed("test action") {
            godot_print!("Got test action!");
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let mut current_velocity = self.base().get_velocity();

        if !self.base().is_on_floor() {
            current_velocity += self.base().get_gravity() * Vector3::splat(delta as real);

            self.base_mut().set_velocity(current_velocity);
        }

        if self.input_singleton.is_action_just_pressed("jump") && self.base().is_on_floor() {
            self.base_mut().get_velocity().y = self.jump_velocity as real;
        }

        // Movement
        let transform_basis = self.base().get_transform().basis;
        let input_dir = self.input_singleton.get_vector(
            "left",
            "right",
            "forward",
            "backward");

        if input_dir != Vector2::ZERO {
            let direction = (transform_basis * Vector3::new(
                input_dir.x,
                0.0,
                input_dir.y)).normalized();

            let new_x = direction.x * self.speed as real;
            let new_z = direction.z * self.speed as real;
            self.base_mut().set_velocity(Vector3::new(
                new_x,
                current_velocity.y,
                new_z,
            ));

            self.set_is_wasd(true);
        } else {
            self.set_is_wasd(false);

            let new_x = move_toward(current_velocity.x as f64, 0.0, self.speed as f64);
            let new_y = move_toward(current_velocity.y as f64, 0.0, self.speed as f64);

            self.base_mut().set_velocity(Vector3::new(new_x as real,
                                                      new_y as real,
                                                      current_velocity.z));
        }

        self.base_mut().move_and_slide();
    }

    fn ready(&mut self) {
        self.current_map = self.base().get_parent();
        self.input_singleton = Input::singleton();

        let mut signal_bus = self.base().get_node_as::<Node>("/root/SignalBus");
        let self_gd = self.to_gd();
        let player_damage = callable_method!(&self.health_component, "on_player_damage");

        signal_bus.connect(
            "voxelgi_toggled",
            &callable_method!(&self.to_gd(), "on_voxelgi_toggled"));

        self.health_component.connect(
            "health_zero",
            &callable_method!(&self_gd, "on_health_zero"));


        self.base_mut().connect("take_damage", &player_damage);

        let actions = [ "left", "right", "forward", "backward" ];
        for action in actions {
            if self.input_map.has_action(action) {
                godot_print!("Has action: {}", action);
            }
        }
    }
}

#[godot_api]
impl RustPlayer {
    // Signals
    #[signal]
    fn take_damage(damage: i64);
    // End Signals

    #[func]
    pub fn on_voxelgi_toggled(&self, status: bool) {
        let mut voxel_gi = match self.current_map.as_ref() {
            Some(voxelgi) => {
                voxelgi.get_node_as::<VoxelGi>("VoxelGI")
            }
            None => {
                godot_print!("current_map is None");
                return;
            },
        };
        if status {
            voxel_gi.show();
        } else {
            voxel_gi.hide();
        }
    }

    #[func]
    pub fn on_health_zero(&mut self) {
        self.base_mut().queue_free();
    }
}