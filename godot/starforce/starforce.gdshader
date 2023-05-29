shader_type spatial;
render_mode skip_vertex_transform;

uniform vec4 color: source_color;

uniform float diffuse;
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
uniform float transparency_envelope;

uniform sampler2D tex_color: source_color;
uniform int tex_color_axis;
uniform int tex_color_projection;
uniform mat4 tex_color_projection_transform;
uniform vec3 tex_color_projection_falloff;
uniform int tex_color_projection_falloff_type;
uniform bool tex_color_projection_world_coords;

uniform sampler2D tex_diffuse: source_color;
uniform int tex_diffuse_axis;
uniform int tex_diffuse_projection;
uniform mat4 tex_diffuse_projection_transform;
uniform vec3 tex_diffuse_projection_falloff;
uniform int tex_diffuse_projection_falloff_type;
uniform bool tex_diffuse_projection_world_coords;

varying vec3 position;
varying vec3 normal;

vec3 project(
	sampler2D tex,
	int mode,
	mat4 transform,
	vec3 falloff,
	int falloff_type,
	bool world_coords,
	vec2 uv,
) {
	switch (mode) {
		case 5: // UV
			return texture(tex, uv).rgb;
		case 4: // Front Projection
			return vec3(0.0, 1.0, 0.0);
		case 3:
			vec3 p = (transform * vec4(position, 1.0)).xyz / 10.0;
			vec3 n = normalize(abs(mat3(transform) * normal));
			vec2 uv2 = (n.x > n.y && n.x > n.z) ? vec2(p.z, p.y)
				: ((n.y > n.x && n.y > n.z) ? vec2(p.x, p.z) : vec2(p.x, p.y));
			return texture(tex, uv2).rgb;
		case 2: // Spherical
			return vec3(0.0, 0.0, 1.0);
		case 1: // Cylindrical
			return vec3(1.0, 1.0, 0.0);
		case 0: // Planar
			return texture(tex, (transform * vec4(position, 1.0)).xz / 10.0).rgb;
		default:
			return vec3(0.0);
	}
}

void vertex() {
	position = VERTEX;
	VERTEX = (MODELVIEW_MATRIX * vec4(VERTEX, 1.0)).xyz;
    NORMAL = normalize((MODELVIEW_MATRIX * vec4(NORMAL, 0.0)).xyz);
}

void fragment() {
	vec3 rot = vec3(PI / 4.0, PI / -2.0, PI / -4.0);
	mat3 x = mat3(vec3(1.0, 0.0, 0.0), vec3(0.0, cos(rot.x), -sin(rot.x)), vec3(0.0, sin(rot.x), cos(rot.x)));
	mat3 y = mat3(vec3(cos(rot.y), 0.0, sin(rot.y)), vec3(0.0, 1.0, 0.0), vec3(-sin(rot.y), 0.0, cos(rot.y)));
	mat3 z = mat3(vec3(cos(rot.z), -sin(rot.z), 0.0), vec3(sin(rot.z), cos(rot.z), 0.0), vec3(0.0, 0.0, 1.0));
	mat3 mat = x * y * z;
	
	normal = (INV_VIEW_MATRIX * vec4(NORMAL, 0.0)).xyz;
	
	ALBEDO = project(
		tex_color,
		tex_color_projection,
		tex_color_projection_transform,
		tex_color_projection_falloff,
		tex_color_projection_falloff_type,
		tex_color_projection_world_coords,
		UV2
	);
}

void light() {
	DIFFUSE_LIGHT =project(
		tex_diffuse,
		tex_diffuse_projection,
		tex_diffuse_projection_transform,
		tex_diffuse_projection_falloff,
		tex_diffuse_projection_falloff_type,
		tex_diffuse_projection_world_coords,
		UV
	);
}
