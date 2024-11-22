extends Node3D
class_name SFXComp

@onready var sfx_manager: SFXManager = $SFXManager
@export var track_array: Array[AudioTrack]


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	sfx_manager.get_tracks_from_array(track_array)
	var track := sfx_manager.retrieve_audio_track("amb_wind1")
	print(track.get_track_name())


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass
