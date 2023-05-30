@tool
extends EditorScript

func _run():
	if ProjectSettings.load_resource_pack("user://mhk3.pck"):
		print("success!")
	else:
		print("failed :(")
