#version 460 core

in vec2 fragPos;

layout (location = 0) out vec3 dir;

layout (location = 0) uniform mat4 invProjView;
layout (location = 1) uniform float near;
layout (location = 2) uniform float far;

void main() {
    dir = normalize((invProjView * (vec4(fragPos, 1, 1) * far - vec4(fragPos, -1, 1) * near)).xyz);
}