extends Camera3D


var camera_sensitivity := 0.3
var parent_player: Player


func _ready() -> void:
	parent_player = get_parent()


func _unhandled_input(event: InputEvent) -> void:
	if event is InputEventMouseMotion:
		parent_player.rotate_y(deg_to_rad(-event.relative.x) * camera_sensitivity)
		
		rotate_x(deg_to_rad(-event.relative.y) * camera_sensitivity)
		rotation.x = clamp(rotation.x, -deg_to_rad(90), deg_to_rad(90))
