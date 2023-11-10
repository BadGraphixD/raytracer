#version 460 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec2 texCoords;
layout (location = 2) in vec3 normal;

out vec3 vertPosition;
out vec2 vertTexCoords;
out vec3 vertNormal;

layout (binding = 0, std430) uniform mat4 projView;
layout (binding = 1, std430) uniform mat4 model;

void main() {
    vertPosition = position;
    vertTexCoords = texCoords;
    vertNormal = normal;

    gl_Position = viewProj * model * vec4(position, 1);
}