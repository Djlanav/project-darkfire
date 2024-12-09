extends Node


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	var actions = InputMap.get_actions()
	for action in actions:
		var input_event := InputMap.action_get_events(action)
		for event in input_event:
			print(event.as_text())


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass
