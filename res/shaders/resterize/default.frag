#version 460 core

in vec3 vertPosition;
in vec2 vertTexCoords;
in vec3 vertNormal;

layout (location = 0) out vec3 position;
layout (location = 1) out vec4 normalMat;
layout (location = 2) out vec2 texCoords;

layout (location = 0) uniform int materialIdx;

void main() {
    position = vertPosition;
    normalMat = vec4(vertNormal, intBitsToFloat(materialIdx));
    texCoords = vertTexCoords;
}