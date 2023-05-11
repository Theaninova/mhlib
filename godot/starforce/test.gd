@tool
extends Node3D

@export var click = false:
	get:
		return false
	set(value):
		var lwo = Lwo.new()
		var mesh = lwo.get_mesh("E:\\Games\\Moorhuhn Kart 3\\extract\\D\\Moorhuhnkart\\3dobjects_cars\\affe.lwo")
		var instance = MeshInstance3D.new()
		instance.mesh = mesh
		add_child(instance)
