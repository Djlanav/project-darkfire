extends RichTextLabel


var og_text: String


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	og_text = text


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	var formatted := og_text % Engine.get_frames_per_second()
	text = formatted
