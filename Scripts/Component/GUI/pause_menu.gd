extends Panel
class_name PauseMenu


signal settings_pressed
signal return_pressed


func _on_settings_button_pressed() -> void:
	hide()
	settings_pressed.emit()


func _on_return_button_pressed() -> void:
	return_pressed.emit()


func _on_settings_menu_closed() -> void:
	show()


func _on_exit_button_pressed() -> void:
	get_tree().quit()
