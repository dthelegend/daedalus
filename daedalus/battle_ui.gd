extends Control
class_name BattleUI

var planet = ""

func _new(planet_name: String) -> void:
	planet = planet_name
	

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	var text = str("War on ", planet)
	
	get_node("Base/Title").text = text
	
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass
