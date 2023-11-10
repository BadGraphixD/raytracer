#version 460 core

in vec2 fragPos;
out vec3 fragCol;

layout (binding = 0) uniform vec3 right;
layout (binding = 1) uniform vec3 up;
layout (binding = 2) uniform vec3 front;

void main() {
    fragCol = normalize(right * fragPos.x + up * fragPos.y + front);
}