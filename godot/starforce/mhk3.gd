@tool
extends EditorScript

func _run():
	var result = Mhk3Map.install("/home/theaninova/Projects/mhlib/games/Moorhuhn Kart 3/data.sar", "mhk3")
	print(result)
