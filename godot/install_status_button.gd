extends Button

@export var game: String

func _ready():
	if FileAccess.file_exists("user://%s.pck" % game):
		text = "✅"
	else:
		text = "⚠️"

func _pressed():
	if !FileAccess.file_exists("user://%s.pck" % game):
		get_tree().change_scene_to_file("res://installer.tscn")
