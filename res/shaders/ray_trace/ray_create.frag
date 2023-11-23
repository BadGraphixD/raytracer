#version 460 core

in vec2 fragPos;

layout (location = 0) out vec3 dir;

layout (location = 0) uniform vec3 right;
layout (location = 1) uniform vec3 up;
layout (location = 2) uniform vec3 front;
layout (location = 3) uniform vec3 position;

void main() {
    dir = normalize(right * fragPos.x + up * fragPos.y + front);
}