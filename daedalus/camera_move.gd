extends Node2D

@onready var cam: Camera2D = $Camera2D

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	set_process(true)

func _input(event: InputEvent):
	if event is InputEventMouseButton:
		if event.button_index == MOUSE_BUTTON_WHEEL_UP:
			if cam.zoom.x < 1.75:
				cam.zoom += Vector2(0.05,0.05)
				
		elif event.button_index == MOUSE_BUTTON_WHEEL_DOWN:
			if cam.zoom.x > 0.8:
				cam.zoom -= Vector2(0.05,0.05)

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	
	var mouse_offset = (get_viewport().get_mouse_position() - Vector2(get_viewport().size / 2))
		
	
	position = lerp(Vector2(), mouse_offset.normalized() * 600, mouse_offset.length() / 500)
