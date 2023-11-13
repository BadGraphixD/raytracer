#version 460 core

in vec2 fragPos;
out vec4 fragCol;

layout (location = 0) uniform sampler2D display;

void main() {
    fragCol = texture(display, fragPos);
}