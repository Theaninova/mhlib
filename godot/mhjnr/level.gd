@tool
extends Node2D

@export var level_id: int = 1

func _ready() -> void:
	var player = preload("res://mhjnr/Moorhuhn.tscn").instantiate()
	add_child(player)
	%Camera.player = player
	%HudLevel.text = "Level %d" % level_id
	player.position = Vector2(200, 10)
	var camera_rect: Rect2i
	
	var level: ObjectScript = load("datafile://data/level%02d/settings/level.txt" % level_id)
	for data in level.static_objects:
		match data.resource_type:
			"LevelSettings":
				%LevelTimer.start(data.props["levelTime"])
			"TiledLayer":
				var scene = load("datafile://data/level%02d/layers/%s.dat" % [level_id, data.name.to_lower()])
				var tiles: TileMap = scene.instantiate()
				tiles.name = data.name
				tiles.visible = data.props["is visible"] == 1

				var used = tiles.get_used_rect()
				used.position *= tiles.cell_quadrant_size
				used.size *= tiles.cell_quadrant_size
				camera_rect = used if camera_rect == null else camera_rect.merge(used)

				var scroll_speed: Vector2 = data.props["scroll speed"]
				if scroll_speed.is_equal_approx(Vector2(1, 1)):
					add_child(tiles)
				else:
					var parallax = ParallaxLayer.new()
					parallax.name = data.name
					parallax.visible = data.props["is visible"] == 1
					parallax.motion_scale = data.props["scroll speed"]
					%parallax.add_child(parallax)
					parallax.add_child(tiles)
	
	%Camera.limit_left = camera_rect.position.x
	%Camera.limit_top = camera_rect.position.y
	%Camera.limit_right = camera_rect.position.x + camera_rect.size.x
	%Camera.limit_bottom = camera_rect.position.y + camera_rect.size.y

	%WorldBoundLeft.position.x = camera_rect.position.x
	%WorldBoundTop.position.y = camera_rect.position.y
	%WorldBoundRight.position.x = camera_rect.position.x + camera_rect.size.x
	%WorldBoundBottom.position.y = camera_rect.position.y + camera_rect.size.y
	
	#var enemies: ObjectScript = load("datafile://data/level%02d/layers/%s.dat" % [level_id, name.to_lower()])
	#for object in enemies.dynamic_objects:
	#	match object.props["subType"]:
	##		1: create_movable(object)
	#		0: create_enemy(object)

func create_enemy(data: ObjectData):
	pass

func create_movable(data: ObjectData):
	pass
