extends SFXManager
class_name SFXComp


@export var track_array: Array[AudioTrack]


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	get_streams_from_array(track_array)
	var track := get_audio_track("amb_wind1")
	if track:
		print("Found track: ", track.get_track_name())
	else:
		print("Could not find track: ", track.get_track_name())


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass
