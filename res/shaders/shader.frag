#version 460 core

#extension GL_EXT_nonuniform_qualifier : enable
#ifdef GL_EXT_nonuniform_qualifier
#define NON_UNIFORM nonuniformEXT
#else
#define NON_UNIFORM
#endif

#define MISS 1e30

#define TOP_SKY vec3(0.5, 0.7, 0.9)
#define BOTTOM_SKY vec3(0.2, 0.5, 0.8)

#define DEFAULT_AMBIENT vec3(0.3)
#define DEFAULT_DIFFUSE vec3(1.0)
#define DEFAULT_SPECULAR vec3(1.0)
#define DEFAULT_SPEC_EXP 30.0

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

layout (location = 3) uniform sampler2D shadowDir;
layout (location = 4) uniform sampler2D shadowHits;
layout (location = 5) uniform sampler2D reflectDir;
layout (location = 6) uniform sampler2D reflectHits;
layout (location = 7) uniform sampler2D ambientDir;
layout (location = 8) uniform sampler2D ambientHits;

layout (location = 9) uniform vec3 lightPos;
layout (location = 10) uniform vec3 cameraPos;

layout (std430, binding = 0) buffer triangleBuffer { Triangle triangles[]; };
layout (std430, binding = 1) buffer texCoordBuffer { float triPositions[]; };
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
    if (hasTexCoords) {
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
    if (hasNormals) {
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
    vec3 position = texture(position, fragPos).xyz;
    vec4 normalMat = texture(normalMat, fragPos).xyzw;
    vec2 texCoord = texture(texCoord, fragPos).xy;

    float distToLight = distance(position, lightPos);

    bool shadow = texture(shadowHits, fragPos).x >= distToLight;
    vec3 reflectColor = getColor(texture(reflectDir, fragPos).xyz, toIntersection(texture(reflectHits, fragPos)));
    vec3 ambientColor = getColor(texture(ambientDir, fragPos).xyz, toIntersection(texture(ambientHits, fragPos)));

    // todo: mix and so on...
}