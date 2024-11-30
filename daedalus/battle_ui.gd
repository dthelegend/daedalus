extends Control
class_name BattleUI

var planet = ""
var percentage = 0

@onready var title: Label = $Base/Title
@onready var bar: Control = $Base/BattleBar

func update_planet(planet_name:String) -> void:
	planet = planet_name
	title.text =  str("WAR: ", planet_name)
	
# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	
	update_planet("Darius-4")

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass

func _on_spend_button_pressed() -> void:
	percentage += 10
	bar.set_percentage(percentage)
