extends SFXManager
class_name SFXComp


@export var track_array: Array[AudioTrack]


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	get_streams_from_array(track_array)


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass
