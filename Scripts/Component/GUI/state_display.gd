extends Control
class_name StateDisplayComponent


@onready var label: RichTextLabel = $RichTextLabel


var player_ref: Player
var state: Player.State
var og_text: String


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	og_text = label.text
	player_ref = get_parent()


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(_delta: float) -> void:
	state = player_ref.current_state
	label.text = og_text % Player.State.keys()[state]
