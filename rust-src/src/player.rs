use std::ops::{Add, Deref};
use crate::{components::HealthComponent, components::PlayerInputComponent, audio_handling::SFXManager,
            callable_method};
use godot::classes::{CharacterBody3D, ICharacterBody3D, InputMap, RayCast3D, RigidBody2D, RigidBody3D, SpotLight3D, VoxelGi, Window};
use godot::classes::light_3d::Param;
use godot::global::move_toward;
use godot::prelude::*;
use crate::components::FirstPersonCamera;
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
    player_camera: OnReady<Gd<FirstPersonCamera>>,
    health_component: OnReady<Gd<HealthComponent>>,
    input_component: OnReady<Gd<PlayerInputComponent>>,
    sfx_man: OnReady<Gd<SFXManager>>,
    pick_up: OnReady<Gd<RayCast3D>>,
    hold_point: OnReady<Gd<Node3D>>,
    flashlight: OnReady<Gd<SpotLight3D>>,
    console: OnReady<Gd<Window>>,
    // @onready end

    #[export]
    speed: i64,
    #[export]
    jump_velocity: f64,
    #[export]
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
            pick_up: OnReady::node("PlayerCamera/PickUpRange"),
            hold_point: OnReady::node("PlayerCamera/HoldPoint"),
            flashlight: OnReady::node("PlayerCamera/Flashlight"),
            console: OnReady::node("ConsoleWindow"),

            speed: 5,
            jump_velocity: 7.5,
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

        // Movement
        let transform_basis = self.base().get_transform().basis;
        let input_dir = self.input_singleton.get_vector(
            "left",
            "right",
            "forward",
            "backward");

        if !self.base().is_on_floor() {
            current_velocity += self.base().get_gravity() * Vector3::splat(delta as real);
            self.base_mut().set_velocity(current_velocity);
        }

        if !input_dir.is_zero_approx() {
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
            let new_z = move_toward(current_velocity.z as f64, 0.0, self.speed as f64);

            self.base_mut().set_velocity(Vector3::new(new_x as real,
                                                      current_velocity.y,
                                                      new_z as real));
        }

        if self.input_singleton.is_action_just_pressed("jump") && self.base().is_on_floor() {
            let get_jump_velocity = self.get_jump_velocity();
            self.base_mut().set_velocity(Vector3::new(current_velocity.x,
                                                      get_jump_velocity as real,
                                                      current_velocity.z));
        }

        self.base_mut().move_and_slide();

        let collision_count = self.base().get_slide_collision_count();
        for index in 0..collision_count {
            let collision = self.base_mut().get_slide_collision(index);
            match collision {
                Some(kinematic_collision) => {
                    match kinematic_collision.get_collider() {
                        Some(collider) => {
                            if let Ok(mut body) = collider.try_cast::<RigidBody3D>() {
                                body.apply_central_impulse(
                                    -kinematic_collision.get_normal()
                                        * Vector3::splat(self.inertia as real));
                            }
                        }
                        None => continue
                    }
                },
                None => continue
            }
        }
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

        self.input_component.connect(
            "toggle_flashlight",
           &callable_method!(&self_gd, "on_flashlight_toggled"));


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
    pub fn on_flashlight_toggled(&mut self) {
        let light_energy = self.flashlight.get_param(Param::ENERGY);
        if light_energy >= 1.0 {
            self.flashlight.set_param(Param::ENERGY, 0.0);
        } else {
            self.flashlight.set_param(Param::ENERGY, 2.3);
        }
    }

    #[func]
    pub fn on_health_zero(&mut self) {
        self.base_mut().queue_free();
    }
}