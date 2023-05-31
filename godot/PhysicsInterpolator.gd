extends Node3D

class_name PhysicsInterpolator

var physics_delta = 1.0;

func _ready():
	top_level = true

func _process(delta):
	transform = transform.interpolate_with(
		get_parent().global_transform, delta / physics_delta)

func _physics_process(delta):
	physics_delta = delta
