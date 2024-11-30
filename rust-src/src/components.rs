use godot::classes::{INode3D, Node3D};
use godot::obj::{Base, WithBaseField};
use godot::prelude::{godot_api, GodotClass};

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

}