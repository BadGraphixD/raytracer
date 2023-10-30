#version 330

in vec2 fragPos;
out vec4 fragCol;

uniform sampler2D display;

void main() {
    fragCol = texture(display, (fragPos + 1) / 2) + vec4(0, 0, 1, 0);
}