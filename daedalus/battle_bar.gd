extends Control

# Exported variables to customize the bar
@export var background_color: Color = Color(0.2, 0.2, 0.2)  # Default background color
@export var fill_color: Color = Color(0.1, 0.7, 0.2)  # Default fill color
@export var bar_width: int = 130  # Default width of the bar
@export var bar_height: int = 20  # Default height of the bar

@onready var background_rect: ColorRect = $BarBackground
@onready var fill_rect: ColorRect = $BarFill
@onready var label: Label = $BarText

# Initialize the bar
func _ready() -> void:
	# Apply default size and colors
	update_bar_size()
	update_colors()

	# Initialize the bar at 0%
	set_percentage(87)

# Update the bar's size
func update_bar_size() -> void:
	background_rect.size = Vector2(bar_width, bar_height)
	fill_rect.size = Vector2(0, bar_height)

# Update the colors of the bar
func update_colors() -> void:
	background_rect.color = background_color
	fill_rect.color = fill_color

# Set the percentage of the bar (0 to 100)
func set_percentage(percent: float) -> void:
	percent = clamp(percent, 0, 100)  # Clamp value between 0 and 100
	var fill_width = (percent / 100.0) * bar_width
	fill_rect.size.x = fill_width

	# Update label (optional)
	if label:
		label.text = str(percent) + "%"
		label.position = Vector2(
			fill_width - label.size.x / 2,
			label.position.y
		)
