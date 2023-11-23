#version 460 core

#extension GL_EXT_nonuniform_qualifier : enable
#ifdef GL_EXT_nonuniform_qualifier
#define NON_UNIFORM nonuniformEXT
#else
#define NON_UNIFORM
#endif

#define MISS 1e30
#define NO_MATERIAL 1e30

const vec3 SUN_DIR = normalize(vec3(1, 2, 1));
const vec3 SUN_COL = vec3(1, 0.97, 0.86);
#define TOP_SKY vec3(0.5, 0.7, 0.9)
#define BOTTOM_SKY vec3(0.2, 0.5, 0.8)

#define DEFAULT_AMBIENT vec3(0.3)
#define DEFAULT_DIFFUSE vec3(1.0)
#define DEFAULT_SPECULAR vec3(1.0)
#define DEFAULT_SPEC_EXP 30.00

in vec2 fragPos;
layout (location = 0) out vec4 color;

struct Intersection {
    float t;
    float u, v;
    uint tringleIdx;
};

struct Triangle {
    uint p0, p1, p2, matIdx;
};

layout (location = 0) uniform sampler2D position;
layout (location = 1) uniform sampler2D normalMat;
layout (location = 2) uniform sampler2D texCoord;

layout (location = 3) uniform sampler2D viewDir;

layout (location = 4) uniform sampler2D shadowDir;
layout (location = 5) uniform sampler2D shadowHits;
layout (location = 6) uniform sampler2D reflectDir;
layout (location = 7) uniform sampler2D reflectHits;
layout (location = 8) uniform sampler2D ambientDir;
layout (location = 9) uniform sampler2D ambientHits;

layout (location = 10) uniform vec3 lightPos;
layout (location = 11) uniform vec3 cameraPos;

layout (location = 12) uniform bool hasNormalBuffer;
layout (location = 13) uniform bool hasTexCoordBuffer;

layout (std430, binding = 0) buffer triangleBuffer { Triangle triangles[]; };
layout (std430, binding = 1) buffer positionBuffer { float triPositions[]; };
layout (std430, binding = 2) buffer texCoordBuffer { float triTexCoords[]; };
layout (std430, binding = 3) buffer normalBuffer { float triNormals[]; };

vec3 fetchPosition(uint index) {
    return vec3(
        triPositions[index * 3 + 0],
        triPositions[index * 3 + 1],
        triPositions[index * 3 + 2]
    );
}

vec2 fetchTexCoord(uint index) {
    return vec2(
        triTexCoords[index * 2 + 0],
        triTexCoords[index * 2 + 1]
    );
}

vec3 fetchNormal(uint index) {
    return vec3(
        triNormals[index * 3 + 0],
        triNormals[index * 3 + 1],
        triNormals[index * 3 + 2]
    );
}

vec2 triangleTexCoord(const uint idx, const vec2 uv) {
    if (hasTexCoordBuffer) {
        vec2 t0 = fetchTexCoord(triangles[idx].p0);
        vec2 t1 = fetchTexCoord(triangles[idx].p1);
        vec2 t2 = fetchTexCoord(triangles[idx].p2);
        float w = 1 - uv.x - uv.y;
        return t1 * uv.x + t2 * uv.y + t0 * w;
    } else {
        return vec2(0, 0);
    }
}

vec3 triangleNormal(const uint idx, const vec2 uv) {
    if (hasNormalBuffer) {
        vec3 n0 = fetchNormal(triangles[idx].p0);
        vec3 n1 = fetchNormal(triangles[idx].p1);
        vec3 n2 = fetchNormal(triangles[idx].p2);
        float w = 1 - uv.x - uv.y;
        return normalize(n1 * uv.x + n2 * uv.y + n0 * w);
    } else {
        vec3 v0 = fetchPosition(triangles[idx].p0);
        vec3 v1 = fetchPosition(triangles[idx].p1);
        vec3 v2 = fetchPosition(triangles[idx].p2);
        return normalize(cross(v1 - v0, v2 - v0));
    }
}

vec3 skybox(const vec3 dir) {
    float sun_fac = clamp(pow(dot(dir, SUN_DIR), 200.0), 0, 1);
    float sky_fac = (dir.y + 1) * .5;
    return TOP_SKY * sky_fac + BOTTOM_SKY * (1 - sky_fac) + SUN_COL * sun_fac;
}

Intersection toIntersection(const vec4 data) {
    return Intersection(data.x, data.y, data.z, floatBitsToUint(data.w));
}

vec3 getColor(const vec3 dir, const Intersection i) {
    if (i.t == MISS) return skybox(dir);
    // todo: big TODO!!!
    return vec3(1, 0, 1);
}

void main() {
    // fetch parameters
    vec3 position = texture(position, fragPos).xyz;
    vec2 texCoord = texture(texCoord, fragPos).xy;
    vec3 normal;
    int materialIdx;
    {
        vec4 normalMat = texture(normalMat, fragPos).xyzw;
        normal = normalMat.xyz;
        float material = normalMat.w;
        if (material == NO_MATERIAL) {
            color = vec4(.2, .5, .8, 1);
            return;
        }
        materialIdx = floatBitsToInt(normalMat.w);
    }

    vec3 viewDir = texture(viewDir, fragPos).xyz;

    vec3 shadowDir = texture(shadowDir, fragPos).xyz;
    vec3 reflectDir = texture(reflectDir, fragPos).xyz;
    vec3 ambientDir = texture(ambientDir, fragPos).xyz;

    Intersection shadowHit = toIntersection(texture(shadowHits, fragPos));
    Intersection reflectHit = toIntersection(texture(reflectHits, fragPos));
    Intersection ambientHit = toIntersection(texture(ambientHits, fragPos));

    // calculate necessary values
    vec3 vecToLight = lightPos - position;
    vec3 dirToLight = normalize(vecToLight);
    float distToLight = length(vecToLight);
    bool shadow = shadowHit.t < distToLight;

    float diffuse = clamp(shadow ? 0.0 : dot(normal, dirToLight), 0.0, 1.0) * 0.8;
    float specular = clamp(shadow ? 0.0 : pow(dot(reflectDir, dirToLight), 30.0), 0.0, 1.0) * 0.5;
    float ambient = ambientHit.t == MISS ? 0.2 : 0.0;

    //color = vec4((reflectDir * 0.5 + ambientDir * 0.5) * (shadow ? 0.2 : 1), 1);
    color = vec4((diffuse + specular + ambient).xxx, 1);
}