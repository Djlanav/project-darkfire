extends CharacterBody3D
class_name Player


signal take_damage(damage: int)


@onready var health_component: HealthComponent = $HealthComponent
@onready var input_component: PlayerInputComponent = $PlayerInputComponent
@onready var sfx_component: SFXComponent = $SFXComponent


const SPEED = 5.0
const JUMP_VELOCITY = 4.5
const INERTIA = 80.0


func _physics_process(delta: float) -> void:
	# Add the gravity.
	if not is_on_floor():
		velocity += get_gravity() * delta

	# Handle jump.
	if Input.is_action_just_pressed("jump") and is_on_floor():
		velocity.y = JUMP_VELOCITY

	# Get the input direction and handle the movement/deceleration.
	# As good practice, you should replace UI actions with custom gameplay actions.
	var input_dir := Input.get_vector("left", "right", "forward", "backward")
	var direction := (transform.basis * Vector3(input_dir.x, 0, input_dir.y)).normalized()
	if direction:
		velocity.x = direction.x * SPEED
		velocity.z = direction.z * SPEED
	else:
		velocity.x = move_toward(velocity.x, 0, SPEED)
		velocity.z = move_toward(velocity.z, 0, SPEED)

	move_and_slide()
	
	for i in get_slide_collision_count():
		var c = get_slide_collision(i)
		if c.get_collider() is RigidBody3D:
			c.get_collider().apply_central_impulse(-c.get_normal() * INERTIA)


func _on_health_component_health_zero() -> void:
	queue_free()


func _on_player_input_component_movement_begin() -> void:
	sfx_component.play_looping()


func _on_player_input_component_movement_end() -> void:
	sfx_component.stop_looping()
