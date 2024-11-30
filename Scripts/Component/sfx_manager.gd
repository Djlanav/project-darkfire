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
	
	if track.get_amplify() or track.get_reverb() or track.get_hard_limit():
		play_ambience_player(track, AudioServer.get_bus_index("Ambience Effects"))
		


func play_ambience_player(audio_track: AudioTrack, audio_bus: int) -> void:
		ambience_player.set_stream(audio_track.get_stream())
		ambience_player.set_bus(AudioServer.get_bus_name(audio_bus))
		ambience_player.play()
		
		var playing_string = "Playing track: {0} on bus {1}"
		print(playing_string.format([audio_track.get_track_name(), ambience_player.get_bus()]))


func _on_ambience_finished() -> void:
	play_random_ambience()
