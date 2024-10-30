extends Node3D
class_name SFXComponent


@onready var looping_sfx: AudioStreamPlayer3D = $LoopingSFX
@onready var music: AudioStreamPlayer = $Music
@onready var ambience: AudioStreamPlayer = $Ambience


enum SFXState {
	PLAYING,
	STOP,
	PAUSED,
	PLAY_UNTIL
}


var music_list: Array
var ambience_list: Array
var current_state := SFXState.STOP


func _ready() -> void:
	init_ambience_list()


func _process(_delta: float) -> void:
	match current_state:
		SFXState.PLAYING:
			if not looping_sfx.playing:
				looping_sfx.play()
		
		SFXState.STOP:
			if looping_sfx.playing:
				call_deferred("stop_looping")
		
		SFXState.PAUSED:
			if looping_sfx.playing:
				call_deferred("pause_looping")


func init_ambience_list() -> void:
	ambience_list.append(preload("res://FX/SFX/wind-ambience-14720.mp3"))


func play_looping() -> void:
	current_state = SFXState.PLAYING


func stop_looping() -> void:
	looping_sfx.stop()


func pause_looping() -> void:
	if looping_sfx.playing:
		looping_sfx.stream_paused = true


func _on_looping_sfx_finished() -> void:
	looping_sfx.play()


func _on_music_finished() -> void:
	music.stream = music_list.pick_random()
	music.play()


func _on_ambience_finished() -> void:
	ambience.play()


func _on_player_movement_begin() -> void:
	play_looping()


func _on_player_movement_end() -> void:
	current_state = SFXState.PAUSED
