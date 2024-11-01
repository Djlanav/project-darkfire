extends Control
class_name VideoSettings


@onready var msaa_option: OptionButton = $"Anti-Aliasing/OptionButton"
@onready var window_mode: OptionButton = $WindowMode/WindowModeOption


var msaa_3d_path := "rendering/anti_aliasing/quality/msaa_3d"
var msaa_3d = ProjectSettings.get_setting("rendering/anti_aliasing/quality/msaa_3d")
var wm = ProjectSettings.get_setting("display/window/size/mode")


func _ready() -> void:
	msaa_option.selected = msaa_3d
	window_mode.selected = wm


func _on_vsync_toggle_toggled(toggled_on: bool) -> void:
	if toggled_on:
		DisplayServer.window_set_vsync_mode(DisplayServer.VSYNC_ENABLED)
	else:
		DisplayServer.window_set_vsync_mode(DisplayServer.VSYNC_DISABLED)


func _on_option_button_item_selected(index: int) -> void:
	var viewport_rid := get_viewport().get_viewport_rid()
	
	match index:
		0:
			RenderingServer.viewport_set_msaa_3d(viewport_rid, 
					RenderingServer.VIEWPORT_MSAA_DISABLED)
		1:
			RenderingServer.viewport_set_msaa_3d(viewport_rid, 
					RenderingServer.VIEWPORT_MSAA_2X)
		2:
			RenderingServer.viewport_set_msaa_3d(viewport_rid, 
					RenderingServer.VIEWPORT_MSAA_4X)
		3:
			RenderingServer.viewport_set_msaa_3d(viewport_rid,
					RenderingServer.VIEWPORT_MSAA_8X)


func _on_window_mode_option_item_selected(index: int) -> void:
	match index:
		0:
			DisplayServer.window_set_mode(DisplayServer.WINDOW_MODE_WINDOWED)
		1:
			DisplayServer.window_set_mode(DisplayServer.WINDOW_MODE_MINIMIZED)
		2:
			DisplayServer.window_set_mode(DisplayServer.WINDOW_MODE_MAXIMIZED)
		3:
			DisplayServer.window_set_mode(DisplayServer.WINDOW_MODE_FULLSCREEN)
		4:
			DisplayServer.window_set_mode(DisplayServer.WINDOW_MODE_EXCLUSIVE_FULLSCREEN)
