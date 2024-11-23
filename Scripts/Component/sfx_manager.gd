extends Node3D
class_name SFXComp

@onready var sfx_manager: SFXManager = $SFXManager
@onready var ambience_player: AudioStreamPlayer = $Ambience


@export var track_array: Array[AudioTrack]


var amb_array: Array[AudioTrack]


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	sfx_manager.get_tracks_from_array(track_array)
	play_random_ambience()


func play_random_ambience() -> void:
	var track := sfx_manager.retrieve_random_ambience()
	print("SFX ready")
	
	if is_instance_valid(track) and track.get_amplify():
		ambience_player.set_stream(track.get_stream())
		ambience_player.set_bus("Ambience Effects")
		ambience_player.play()
		print("Playing track: ", track.get_track_name())
	elif is_instance_valid(track):
		ambience_player.set_stream(track.get_stream())
		ambience_player.play()
		print("Playing track: ", track.get_track_name())


func _on_ambience_finished() -> void:
	play_random_ambience()
