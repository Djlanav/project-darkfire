extends Node3D
class_name PlayerInputComponent


signal pick_up
signal toggle_flashlight
signal console_toggle
signal paused


var is_mouse_captured: bool
var is_moving := false
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
func _process(_delta: float) -> void:
	if Input.is_action_just_pressed("unfocus") and is_mouse_captured:
		Input.set_mouse_mode(Input.MOUSE_MODE_VISIBLE)
		is_mouse_captured = false
	
	elif Input.is_action_just_pressed("pause"):
		Input.mouse_mode = Input.MOUSE_MODE_VISIBLE
		paused.emit()
		get_tree().paused = true
	
	elif Input.is_action_just_pressed("hold_obj"):
		pick_up.emit()
	
	elif Input.is_action_just_pressed("flashlight"):
		toggle_flashlight.emit()
	
	elif Input.is_action_just_pressed("open_console"):
		open_console()
	
	elif Input.is_action_just_pressed("take_screenshot"):
		await RenderingServer.frame_post_draw
		var time = Time.get_datetime_string_from_system()
		var path = "user://screenshot_{date}".format({"date": time}).replace("T", "_")
		get_viewport().get_texture().get_image().save_png(path)


func open_console() -> void:
	if is_mouse_captured:
		Input.set_mouse_mode(Input.MOUSE_MODE_VISIBLE)
	else:
		Input.set_mouse_mode(Input.MOUSE_MODE_CAPTURED)
	
	console_toggle.emit()
