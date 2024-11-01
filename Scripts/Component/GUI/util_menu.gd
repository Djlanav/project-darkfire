extends Control
class_name UtilMenu


@onready var pause_menu: PauseMenu = $PauseMenu
@onready var settings_menu: Panel = $SettingsMenu


func _on_pause_menu_settings_pressed() -> void:
	settings_menu.show()


func _on_player_input_component_paused() -> void:
	show()


func _on_pause_menu_return_pressed() -> void:
	get_tree().paused = false
	hide()
	Input.mouse_mode = Input.MOUSE_MODE_CAPTURED
