extends Node3D

@onready var finish_line = %FinishLine

func _ready():
	var kart = preload("res://kart/kart.tscn").instantiate()
	add_child(kart)
	
	var pos: Marker3D = finish_line.start_positions[0]
	kart.position = pos.global_position
	kart.rotation = pos.global_rotation
