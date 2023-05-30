@tool
extends EditorScript

var map_id = "08_shimalaya"

var res_path = "res://games/"
var actual_path = "user://.install/"

func _run() -> void:
	var path = "mhk3/D/Moorhuhnkart/3dobjects_tracks/track%s" % map_id
	
	var dir = actual_path + path
	
	var objects = DirAccess.get_directories_at(dir)
	var root = get_scene()
	
	for object in objects:
		var node = Node3D.new()
		root.add_child(node)
		node.owner = root
		node.name = object.trim_suffix(".lwo")
		var obj_path = dir + '/' + object
		
		for file in DirAccess.get_files_at(obj_path):
			var instance = MeshInstance3D.new()
			instance.mesh = load(res_path + path + '/' + object + '/' + file)
			instance.name = file.trim_suffix(".res")
			node.add_child(instance)
			instance.owner = root
