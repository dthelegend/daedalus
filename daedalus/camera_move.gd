extends Node2D

@onready var cam: Camera2D = $Camera2D

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	set_process(true)
	cam.position.x = 1800
	cam.position.y = 1400
	cam.zoom = Vector2(1.5,1.5)

func _input(event: InputEvent):
	var mouse_offset = (get_viewport().get_mouse_position() - Vector2(get_viewport().size / 2))
		
	if event is InputEventMouseButton:
		if event.button_index == MOUSE_BUTTON_WHEEL_UP:
			if cam.zoom.x < 1.75:
				cam.zoom += Vector2(0.05,0.05)
				#position = lerp(Vector2(), mouse_offset.normalized() * 400, mouse_offset.length() / 700)
				
		elif event.button_index == MOUSE_BUTTON_WHEEL_DOWN:
			if cam.zoom.x > 1:
				cam.zoom -= Vector2(0.05,0.05)
				#position = lerp(Vector2(), mouse_offset.normalized() * 400, mouse_offset.length() / 700)
			

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	
	var mouse_offset = (get_viewport().get_mouse_position() - Vector2(get_viewport().size / 2))
		
	position = lerp(Vector2(-50,-50), mouse_offset.normalized() *600, mouse_offset.length() / 700)
	#position = Vector2(clamp(position.x, (3840*cam.zoom.x)/2, 3840-(3840*cam.zoom.x)/2), clamp(position.y, (2160*cam.zoom.y)/2, 2160-(2160*cam.zoom.y)/2))
