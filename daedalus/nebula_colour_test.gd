extends Nebula

@onready var polygon: Polygon2D = $Polygon2D
@onready var bt: BattleUI = get_node("/root/Map/BattleUI")

var factions = ["None", "Zeta", "Icari", "Jrahass", "Mercune", "Oracles"]
var count = 0
var label : RichTextLabel = null
var isIn = false


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	set_process(true)
	label = RichTextLabel.new()
	add_child(label)
	
	if energy_yield == 0:
		energy_yield = randi() % 100
	if title == "":
		title = str("N-", randi() % 9999)
	
	
	label.hide()
	label.bbcode_enabled = true
	label.text = str("[color=white]Nebula ", title, "| Energy Yield ", energy_yield, "[/color]")
	label.set_size(Vector2(2024, 1240), false)
	label.add_theme_font_size_override("normal_font_size", 42)
	label.z_index = 5
	#polygon.apply_shader_with_color(factions[ownership+1])

# Called every frame. 'delta' is the elapsed time since the previous frame.

func _process(delta: float) -> void:
	if count > 100:
		polygon.apply_shader_with_color(factions[randi() % 6])
		count = 0
	else:
		count += 1
	
	if isIn && count % 2 == 0:
		polygon.apply_shader_with_color(factions[0])
		var oldpos = get_viewport().get_mouse_position()
		label.position = Vector2(oldpos[0]-50,oldpos[1]-50)
		
	
func _on_area_2d_mouse_entered() -> void:
	label.show()
	isIn = true
	
	
func _on_area_2d_mouse_exited() -> void:
	label.hide()
	isIn = false
