extends Resource

class_name PlayerProfile

const Statistics = preload("res://mhjnr/profile/statistics.gd")
const Status = preload("res://mhjnr/profile/status.gd")

@export var name: String

@export var volume_music: int
@export var volume_sfx: int

@export var current_level: int
@export var game_finished: bool

@export var input_keys: Array[int] = []
@export var input_buttons: Array[int] = []

@export var status: Status = Status.new()
@export var statistics: Statistics = Statistics.new()

func from_object_data(data: ObjectData):
	name = data["name"]
	volume_music = data["volumeMusic"]
	volume_sfx = data["volumeSfx"]

	for i in range(7):
		input_keys.push_back(data["input %d key"] % i)
		input_buttons.push_back(data["input %d button"] % i)
	
	current_level = data["currentLevel"]
	game_finished = data["gameFinished"] != 0
	
	for child in data.children:
		match child.resource_type:
			"Status":
				status.from_object_data(child)
			"Statistics":
				statistics.from_object_data(child)
