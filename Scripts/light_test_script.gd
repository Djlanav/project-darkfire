extends Node3D


@onready var lights: Node3D = $Lights


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	lights.rotation_degrees.y += 15 * delta
