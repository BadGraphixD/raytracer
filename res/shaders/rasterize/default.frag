#version 460 core

in vec3 vertPosition;
in vec2 vertTexCoords;
in vec3 vertNormal;

layout (location = 0) out vec4 position;
layout (location = 1) out vec4 normalMat;
layout (location = 2) out vec4 texCoords;

layout (location = 1) uniform int materialIdx;

void main() {
    position = vec4(vertPosition, 0.0);
    normalMat = vec4(vertNormal, intBitsToFloat(materialIdx));
    texCoords = vec4(vertTexCoords, 0.0, 0.0);
}