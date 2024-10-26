extends Node3D
class_name SFXComponent


@onready var looping_sfx: AudioStreamPlayer3D = $LoopingSFX
@onready var music: AudioStreamPlayer = $Music
@onready var ambience: AudioStreamPlayer = $Ambience


var music_list: Array
var ambience_list: Array


func init_ambience_list() -> void:
	ambience_list.append(preload("res://FX/SFX/wind-ambience-14720.mp3"))


func play_looping() -> void:
	if not looping_sfx.playing:
		looping_sfx.play()


func stop_looping() -> void:
	if looping_sfx.playing and not Input.is_anything_pressed():
		looping_sfx.stop()


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	init_ambience_list()


func _on_looping_sfx_finished() -> void:
	looping_sfx.play()


func _on_music_finished() -> void:
	var next_track = music_list.pick_random()
	
	music.stream = next_track
	music.play()


func _on_ambience_finished() -> void:
	ambience.play()
