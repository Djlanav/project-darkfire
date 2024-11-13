extends RigidBody3D


@onready var sfx_component: SFXComponent = $SFXComponent


func _on_body_entered(body: Node) -> void:
	sfx_component.activate_play_once()
