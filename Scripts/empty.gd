extends Node3D


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	ProjectSettings.load_resource_pack("res://graphics.pck")
	ProjectSettings.load_resource_pack("res://audio.pck")
	get_tree().change_scene_to_file.call_deferred("res://Areas/testing_world.tscn")
