extends Resource

class_name Status

@export var lives: int
@export var score: int
@export var bullets: int

func from_object_data(data: ObjectData):
	lives = data["lives"]
	score = data["score"]
	bullets = data["bullets"]
