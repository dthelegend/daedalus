extends Nebula

@onready var polygon: Polygon2D = $Polygon2D

var factions = ["None", "Zeta", "Icari", "Jrahass", "Mercune", "Oracles"]

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	set_process(true)
	if not polygon:
		queue_free()
	#polygon.apply_shader_with_color(factions[randi() % 6])

func _input(event: InputEvent):
	# Check if the event is a key press and the key is 'C'
	if Input.is_key_pressed(KEY_C):
		polygon.apply_shader_with_color("Mercune")

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	#polygon.apply_shader_with_color(factions[ownership+1])
	pass
	
func _on_area_2d_mouse_entered() -> void:
	print("bruh")
	polygon.apply_shader_with_color(factions[0])
