extends Nebula

@onready var polygon: Polygon2D = $Polygon2D

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	set_process(true)

func _input(event: InputEvent):
	# Check if the event is a key press and the key is 'C'
	if Input.is_key_pressed(KEY_C):
		polygon.apply_shader_with_color("Mercune")

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass
