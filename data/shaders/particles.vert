#version 450 core

layout (location = 0) in vec2 VsInPos;
layout (location = 1) in vec2 VsInUV;

struct ParticleInstance {
  mat4 transform;
  uint texid;
};

layout (binding = 0, std430) readonly buffer InstanceData {
  ParticleInstance particles[];
} Instances;

out gl_PerVertex {
  vec4 gl_Position;
};

out VS_OUT_PS_IN {
  flat uint texid;
  vec2 uv;
} vs_out;

void main() {
  ParticleInstance pi = Instances.particles[gl_InstanceID];
  gl_Position = pi.transform * vec4(VsInPos, 0.0, 1.0);

  vs_out.texid = pi.texid;
  vs_out.uv = VsInUV;
}