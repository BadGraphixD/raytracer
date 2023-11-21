#version 460 core

#define NO_RAY vec3(0, 0, 0)
#define RAY_ORG_OFFSET 0.001

in vec2 fragPos;

layout (location = 0) out vec3 org;
layout (location = 1) out vec3 shadowDir;
layout (location = 2) out vec3 reflectDir;
layout (location = 3) out vec3 ambientDir;

struct Material {
    bool reflect;
};

layout (location = 0) uniform sampler2D positionData; // xyz: position
layout (location = 1) uniform sampler2D normalMatData; // xyz: normal, w: material idx
layout (location = 2) uniform sampler2D blueNoise; // xyz: noise, is unit length vector
layout (location = 3) uniform vec3 lightPos;
layout (location = 4) uniform vec3 cameraPos;
layout (location = 5) uniform vec4 noiseOffsetScale; // xy: offset, zw: scale

layout (std430, binding = 0) buffer matBuffer { Material materials[]; };

void main() {
    vec4 normalMat = texture(normalMatData, fragPos);
    if (normalMat.w == 1e30) discard;

    vec3 normal = normalMat.xyz;
    int materialIdx = floatBitsToInt(normalMat.w);
    vec3 position = texture(positionData, fragPos).xyz;
    //vec3 random = texelFetch(blueNoise, ivec2((fragPos * noiseOffsetScale.zw + noiseOffsetScale.xy) * 512) % 512, 0).xyz;
    vec3 random = texture(blueNoise, fragPos * noiseOffsetScale.zw + noiseOffsetScale.xy).xyz;
    Material material = materials[materialIdx];

    org = position + normal * RAY_ORG_OFFSET;
    shadowDir = normalize(lightPos - position);
    //reflectDir = material.reflect ? reflect(position - cameraPos, normal) : NO_RAY;
    reflectDir = reflect(position - cameraPos, normal);
    ambientDir = normal + random * 2 - 1;
}