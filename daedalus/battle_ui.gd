extends Control
class_name BattleUI

var planet = ""
@onready var title: Label = $Base/Title

func update_planet(planet_name:String) -> void:
	planet = planet_name
	title.text =  str("War on ", planet)
	
func _new(planet_name: String) -> void:
	
	pass
	
# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	
	
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass
