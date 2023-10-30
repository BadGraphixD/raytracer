#version 330

in vec2 fragPos;
out vec3 fragCol;

void main() {
    fragCol = vec3(fragPos, 0.0);
}