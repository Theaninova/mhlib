extends Resource

class_name Statistics

@export var num_shots_fired: int
@export var num_coins_collected: int
@export var num_diamonds_collected: int
@export var enemies_bonus: int
@export var num_powerups_collected: int
@export var num_gold_statues: int

func from_object_data(data: ObjectData):
	num_shots_fired = data["numShotsFired"]
	num_coins_collected = data["numCoinsCollected"]
	num_diamonds_collected = data["numDiamondsCollected"]
	enemies_bonus = data["enemiesBonus"]
	num_powerups_collected = data["numPowerupsCollected"]
	num_gold_statues = data["numGoldStatues"]
