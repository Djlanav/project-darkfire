extends Node3D
class_name SFXComponent


enum LoopingState {
	PLAYING,
	STOP,
	PAUSED,
	PLAY_UNTIL
}


#region Exports
@export var sound_tracks: Dictionary

@export_group("Looping Settings")
@export var looping_stream: AudioStream
@export var reverb_loop: bool
@export var amplify_loop: bool

@export_group("Play-Once Settings")
@export var play_once_stream: AudioStream
@export var reverb_for_once: bool

@export_group("Music")
@export var music_stream: AudioStream
@export var music_list: Array[AudioStream]
@export var loop_music_list: bool

@export_group("Ambience")
@export var ambience_stream: AudioStream
@export var ambience_list: Array[AudioStream]
@export var loop_ambience_list: bool
#endregion


#region Onready
@onready var looping_sfx: AudioStreamPlayer3D = $LoopingSFX
@onready var play_once: AudioStreamPlayer3D = $PlayOnce
@onready var music: AudioStreamPlayer = $Music
@onready var ambience: AudioStreamPlayer = $Ambience
#endregion


var current_looping_state := LoopingState.STOP
var bus_names: Array[String]
var bus_effects: Dictionary


func play_random_ambience() -> void:
	var ambience_tracks: Array[SoundTrack] = sound_tracks["Ambience Tracks"]
	var track := ambience_tracks.pick_random() as SoundTrack
	ambience.stream = track.get_audio_stream()
	
	if track.amplify:
		ambience.bus = "Ambience Effects"


func init_streams() -> void:
	looping_sfx.stream = looping_stream
	play_once.stream = play_once_stream
	music.stream = music_stream
	ambience.stream = ambience_stream


func add_to_lists() -> void:
	if ambience_stream != null:
		ambience.play()
		ambience_list.append(ambience_stream)
	else:
		_on_ambience_finished()
	
	if music_stream != null:
		music.play()
		music_list.append(music_stream)
	else:
		_on_music_finished()


func _get_bus_names() -> void:
	for i in range(0, AudioServer.get_bus_count()):
		bus_names.append(AudioServer.get_bus_name(i))


func _get_bus_effects(bus_name: String) -> Array:
	var bus_index := AudioServer.get_bus_index(bus_name)
	var effect_count := AudioServer.get_bus_effect_count(bus_index)
	if effect_count == 0:
		return []
	
	var effect_array := []
	for index in range(0, effect_count):
		var effect := AudioServer.get_bus_effect(bus_index, index)
		effect_array.append(effect)
	
	return effect_array


func _ready() -> void:
	init_streams()
	add_to_lists()
	_get_bus_names()
	
	for bus in bus_names:
		bus_effects[bus] = _get_bus_effects(bus)


func _process(_delta: float) -> void:
	match current_looping_state:
		LoopingState.PLAYING:
			if not looping_sfx.playing:
				looping_sfx.play.call_deferred()
		
		LoopingState.STOP:
			if looping_sfx.playing:
				looping_sfx.stop.call_deferred()
		
		LoopingState.PAUSED:
			if looping_sfx.playing:
				call_deferred("pause_looping")


func play_looping() -> void:
	current_looping_state = LoopingState.PLAYING


func stop_looping() -> void:
	looping_sfx.stop()


func activate_play_once() -> void:
	if not play_once.playing:
		play_once.play()


func pause_looping() -> void:
	if looping_sfx.playing:
		looping_sfx.stream_paused = true


func set_sfx_state(sfx_state: LoopingState) -> void:
	current_looping_state = sfx_state


func _on_looping_sfx_finished() -> void:
	looping_sfx.play()


func _on_music_finished() -> void:
	if loop_music_list:
		music.stream = music_list.pick_random()
		music_stream = music.stream
		music.play()


func _on_ambience_finished() -> void:
	if loop_ambience_list:
		ambience.stream = ambience_list.pick_random()
		ambience_stream = ambience.stream
		ambience.play()
		
