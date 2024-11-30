extends Polygon2D

func apply_shader_with_color(faction: String):
	var shader_path = str("res://shaders/", faction, ".gdshader")   
 # Remove the current shader (if any)
	if material and material is ShaderMaterial:
		material = null  # Detaches the current material

	# Load the new shader
	var shader = load(shader_path)
	if shader:
		var new_shader_material = ShaderMaterial.new()
		new_shader_material.shader = shader

		# Apply the new shader material to the Polygon2D
		material = new_shader_material
	else:
		print("Failed to load shader from path: ", shader_path)
