extends CharacterBody2D

@export var max_jumps: int = 2
@export_range(0, 4000, 1, "suffix:px/s") var speed: float = 400
@export_range(0, 4000, 1, "suffix:px/s") var terminal_velocity: float = 2000
@export_range(0, 4000, 1, "suffix:px/s") var jump_speed: float = 900
@export_range(0, 4000, 1, "suffix:px/s²") var acceleration: float = 800
@export_range(0, 4000, 1, "suffix:px/s²") var deceleration: float = 1000

@onready var state_machine: AnimationNodeStateMachinePlayback = %AnimationTree["parameters/playback"]
@onready var jumps: int = max_jumps
@onready var gravity: float = ProjectSettings.get_setting("physics/2d/default_gravity")

func is_running():
	return Input.is_action_pressed("Move Left") or Input.is_action_pressed("Move Right")

func is_on_ledge():
	return is_on_floor() and not is_running() and not %Slip.is_colliding()

func did_jump():
	return Input.is_action_just_pressed("Move Up") and jumps > 0

func _ready() -> void:
	apply_floor_snap()

func clamp_dir(value: float, dir: float):
	return clampf(value, min(dir, 0.0), max(dir, 0.0))

func _physics_process(delta: float) -> void:
	velocity.y += gravity * delta
	velocity.y = minf(velocity.y, terminal_velocity)
	
	var vertical: float = Input.get_axis("Move Down", "Move Up")
	var horizontal: float = Input.get_axis("Move Left", "Move Right")
	
	velocity
	if is_on_floor():
		jumps = max_jumps
	if did_jump():
		if jumps != max_jumps:
			state_machine.start(state_machine.get_current_node(), true)
		jumps -= 1
		velocity.y = -jump_speed
	
	var max_speed: float = speed * horizontal
	
	velocity.x += acceleration * horizontal * delta
	if is_running():
		velocity.x = clamp_dir(velocity.x, max_speed)
		%animations.flip_h = horizontal > 0.0
	else:
		velocity.x -= clamp_dir(deceleration * velocity.x * delta, velocity.x)
	
	if is_on_ledge():
		jumps = 0
		var direction: float = 1.0 if %SlipTestFront.is_colliding() else -1.0
		velocity.x += 10_000 * delta * direction
	
	if move_and_slide() and is_on_wall():
		velocity.x = 0.0
