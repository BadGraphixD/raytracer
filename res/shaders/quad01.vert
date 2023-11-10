#version 460 core

layout (location = 0) in vec2 pos;

out vec2 fragPos;

void main() {
    fragPos = (pos + 1) / 2;
    gl_Position = vec4(pos, 0, 1);
}