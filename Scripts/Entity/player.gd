extends CharacterBody3D
class_name Player


signal take_damage(damage: int)


#region Components
@onready var health_component: HealthComponent = $HealthComponent
@onready var input_component: PlayerInputComponent = $PlayerInputComponent
@onready var sfx_man: SFXComp = $SFXComp
#endregion


#region Node References
@onready var pick_up: RayCast3D = %PickUpRange
@onready var hold_point: Node3D = %HoldPoint
@onready var flashlight: SpotLight3D = %Flashlight
@onready var console: Window = $ConsoleWindow
#endregion


enum State {
	IDLE,
	MOVING,
	FALLING
}


const SPEED = 5.0
const JUMP_VELOCITY = 4.5
const INERTIA = 80.0


var current_state: State
var is_wasd: bool
var held_object: RigidBody3D
var last_held_position: Vector3 = Vector3.ZERO
var current_map: Node


func _ready() -> void:
	current_map = get_parent()
	
	SignalBus.voxelgi_toggled.connect(_on_voxelgi_toggled)


func _process(_delta: float) -> void:
	if held_object:
		last_held_position = held_object.global_position
		held_object.global_position = lerp(held_object.global_position, 
				hold_point.global_position, 0.2)


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
	var direction := (transform.basis * Vector3(input_dir.x, 0, 
			input_dir.y)).normalized()
	
	if direction:
		velocity.x = direction.x * SPEED
		velocity.z = direction.z * SPEED
		is_wasd = true
	else:
		is_wasd = false
		velocity.x = move_toward(velocity.x, 0, SPEED)
		velocity.z = move_toward(velocity.z, 0, SPEED)

	move_and_slide()
	
	for i in get_slide_collision_count():
		var c = get_slide_collision(i)
		if c.get_collider() is RigidBody3D:
			c.get_collider().apply_central_impulse(-c.get_normal() * INERTIA)


func _on_health_component_health_zero() -> void:
	queue_free()


func _on_player_input_component_pick_up() -> void:
	if held_object:
		var momentum = (held_object.global_position - last_held_position) / get_process_delta_time()
		held_object.freeze = false
		held_object.linear_velocity = momentum
		held_object = null
		return
	
	if pick_up.is_colliding() and pick_up.get_collider() is RigidBody3D:
		held_object = pick_up.get_collider()
		held_object.freeze = true


func _on_player_input_component_toggle_flashlight() -> void:
	if flashlight.light_energy >= 1:
		flashlight.light_energy = 0
	else:
		flashlight.light_energy = 2.3


func _on_player_input_component_console_toggle() -> void:
	if not console.visible:
		console.show()
	else:
		console.hide()


func _on_voxelgi_toggled(status: bool) -> void:
	var voxel_gi: VoxelGI = current_map.get_node("VoxelGI") as VoxelGI
	if status:
		voxel_gi.show()
	else:
		voxel_gi.hide()
