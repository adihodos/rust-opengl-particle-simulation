#version 450 core

in VS_OUT_PS_IN {
  flat uint texid;
  vec2 uv;
} ps_in;

layout (location = 0) out vec4 FinalFragColor;
layout (binding = 0) uniform sampler2DArray Sprites;

void main() {
  FinalFragColor = texture(Sprites, vec3(ps_in.uv, float(ps_in.texid)));
}