extends Node3D
class_name HealthComponent


signal health_zero


@export var max_health: int
var health: int


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	health = max_health


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(_delta: float) -> void:
	if health <= 0:
		health_zero.emit()


func _on_player_take_damage(damage: int) -> void:
	health -= damage
