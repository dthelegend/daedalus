extends Area2D

@onready var bounds: CollisionPolygon2D = $Polygon2D2
# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	$CollisionPolygon2D.polygon = bounds.polygon

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass
