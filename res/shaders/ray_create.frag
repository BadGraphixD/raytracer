#version 330

in vec2 fragPos;
out vec3 fragCol;

uniform vec3 right, up, front;

void main() {
    fragCol = normalize(right * fragPos.x + up * fragPos.y + front);
}