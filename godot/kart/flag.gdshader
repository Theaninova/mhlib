shader_type canvas_item;

uniform float rotation: hint_range(0.0, 3.14156, 0.01) = 1.4;
uniform float size: hint_range(0.0, 1000.0, 1.0) = 40.0;
uniform float time_scale = 50.0;
uniform float wave_amount = 0.5;
uniform float wave_scale = 0.25;

uniform vec4 color1: source_color = vec4(0, 0, 0, 1);
uniform vec4 color2: source_color = vec4(1);

float aaStep(float compValue, float gradient){
  float halfChange = fwidth(gradient) / 2.0;
  //base the range of the inverse lerp on the change over one pixel
  float lowerEdge = compValue - halfChange;
  float upperEdge = compValue + halfChange;
  //do the inverse interpolation
  float stepped = (gradient - lowerEdge) / (upperEdge - lowerEdge);
  stepped = clamp(stepped, 0.0, 1.0);
  return stepped;
}

void fragment() {
	vec2 pos = FRAGCOORD.xy;
	
	float t = TIME * time_scale;
	float w = size * wave_amount;
	float s = size * wave_scale;
	
	float wave = sin(pos.x / w + t) * s;
	mat2 rot = mat2(vec2(sin(rotation), -cos(rotation)), vec2(cos(rotation), sin(rotation)));
	
	pos.y += wave;
	pos *= rot;
	pos /= size;
	
	float shade = sin(pos.y / w + t) * s;
	
	pos = abs(fract(pos) - 0.5);
	COLOR = mix(color1, color2, aaStep(0.5, pos.x + pos.y));
	
	// COLOR -= (1.0 - (shade / 2.0 + 0.5)) / 80.0;
}
