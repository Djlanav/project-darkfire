extends Node3D
class_name SFXComp

@onready var sfx_manager: SFXManager = $SFXManager
@onready var ambience_player: AudioStreamPlayer = $Ambience


@export var track_array: Array[AudioTrack]


var amb_array: Array[AudioTrack]


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	sfx_manager.get_tracks_from_array(track_array)
	var layout = AudioServer.generate_bus_layout()
	AudioServer.set_bus_layout(layout)
	play_random_ambience()


func play_random_ambience() -> void:
	var track := sfx_manager.retrieve_random_ambience()
	print("SFX ready")
	
	if is_instance_valid(track) and track.get_amplify():
		ambience_player.set_stream(track.get_stream())
		ambience_player.set_bus(AudioServer.get_bus_name(5))
		ambience_player.play()
		
		var my_str = "Playing track: {0} on bus {1}"
		print(my_str.format([track.get_track_name(), ambience_player.get_bus()]))
	elif is_instance_valid(track):
		ambience_player.set_stream(track.get_stream())
		ambience_player.play()
		
		var my_str = "Playing track: {0} on bus {1}"
		print(my_str.format([track.get_track_name(), ambience_player.get_bus()]))


func _on_ambience_finished() -> void:
	play_random_ambience()
