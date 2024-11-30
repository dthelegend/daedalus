shader_type canvas_item;

uniform vec4 overlay_color = vec4(0.851, 0.514, 0.141, 0.5);

void fragment() {
    // Sample the texture at this fragment's UV
    vec4 base_color = texture(TEXTURE, UV);
    // Blend the overlay color with the texture
    COLOR = mix(base_color, overlay_color, overlay_color.a);
}

