#version 330

in vec2 fragPos;
out vec4 fragCol;

void main() {
    fragCol = vec4(fragPos, 0.0, 1.0);
}