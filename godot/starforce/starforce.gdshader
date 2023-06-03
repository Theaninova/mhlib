shader_type spatial;

/*uniform vec4 color: source_color;*/

/*uniform float diffuse;
uniform float diffuse_envelope;
uniform float specular;
uniform float specular_envelope;
uniform float luminosity;
uniform float luminosity_envelope;
uniform float reflectivity;
uniform float reflectivity_envelope;
uniform float translucency;
uniform float translucency_envelope;
uniform float transparency;
uniform float transparency_envelope;*/

uniform sampler2D tex_color: source_color;
uniform sampler2D tex_diffuse: source_color;

void fragment() {
	ALBEDO = texture(tex_color, UV).rgb;
}

void light() {
	DIFFUSE_LIGHT = texture(tex_diffuse, UV2).rgb;
}
