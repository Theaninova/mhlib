extends ScrollContainer

func _on_install():
	pass

func _start_mhk():
	get_tree().change_scene_to_file("res://kart/entry.tscn")
