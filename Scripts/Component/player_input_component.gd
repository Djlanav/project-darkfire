extends Node3D
class_name PlayerInputComponent


signal movement_begin
signal movement_end


var is_mouse_captured: bool
var is_moving: bool
var action_func_map: Dictionary
var event_action_map: Dictionary
var mapped_action: String


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	Input.set_mouse_mode(Input.MOUSE_MODE_CAPTURED)
	is_mouse_captured = true
	
	var actions = InputMap.get_actions()
	
	for action in actions:
		var es := InputMap.action_get_events(action)
		if es.is_empty():
			continue
		else:
			var event := es[0].as_text()
			if action.containsn("ui"):
				continue
			else:
				action_func_map[action] = action
				event_action_map[event[0]] = action
			


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	if Input.is_action_just_pressed("unfocus") and is_mouse_captured:
		Input.set_mouse_mode(Input.MOUSE_MODE_VISIBLE)
		is_mouse_captured = false
	elif Input.is_action_just_pressed("quit"):
		get_tree().quit()
	elif Input.is_action_just_pressed("take_screenshot"):
		await RenderingServer.frame_post_draw
		var time = Time.get_datetime_string_from_system()
		var path = "user://screenshot_{date}".format({"date": time}).replace("T", "_")
		get_viewport().get_texture().get_image().save_png(path)


func check_begin_movement() -> void:
	if is_moving:
		movement_begin.emit()


func _unhandled_input(event: InputEvent) -> void:
	var event_str := event.as_text()
	
	if event.is_pressed() and event_action_map.has(event_str):
		mapped_action = event_action_map[event_str]
		
		match mapped_action:
			"forward":
				movement_begin.emit()
			"backward":
				movement_begin.emit()
			"left":
				movement_begin.emit()
			"right":
				movement_begin.emit()
