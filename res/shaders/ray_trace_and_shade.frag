#version 430

#define MISS 1e30
#define EPSILON 0.000001
#define NODE_STACK_SIZE 100

in vec2 fragPos;
out vec4 fragCol;

uniform sampler2D dirTex;
uniform sampler2D modelAlbedo1;
uniform sampler2D modelAlbedo2;
uniform vec3 org;
uniform bool hasTexCoords;
uniform bool hasNormals;

struct Ray {
    vec3 org, dir, rDir;
};

struct AABB {
    float minx, miny, minz;
    float maxx, maxy, maxz;
};

struct Node {
    AABB aabb;
    bool is_leaf;
    uint a, b;
};

struct Triangle {
    uint p0, p1, p2, matIdx;
};

struct NodeStack {
    uint nodes[NODE_STACK_SIZE];
    uint idx;
};

layout (std430, binding = 0) buffer nodeBuffer { Node nodes[]; };
layout (std430, binding = 1) buffer triangleBuffer { Triangle triangles[]; };
layout (std430, binding = 2) buffer positionBuffer { float positions[]; };
layout (std430, binding = 3) buffer texCoordBuffer { float texCoords[]; };
layout (std430, binding = 4) buffer normalBuffer { float normals[]; };

float intersectAABB(const Ray ray, const AABB aabb, const float t) {
    float tx1 = (aabb.minx - ray.org.x) * ray.rDir.x, tx2 = (aabb.maxx - ray.org.x) * ray.rDir.x;
    float tmin = min(tx1, tx2), tmax = max(tx1, tx2);
    float ty1 = (aabb.miny - ray.org.y) * ray.rDir.y, ty2 = (aabb.maxy - ray.org.y) * ray.rDir.y;
    tmin = max(tmin, min(ty1, ty2)), tmax = min(tmax, max(ty1, ty2));
    float tz1 = (aabb.minz - ray.org.z) * ray.rDir.z, tz2 = (aabb.maxz - ray.org.z) * ray.rDir.z;
    tmin = max(tmin, min(tz1, tz2)), tmax = min(tmax, max(tz1, tz2));
    return (tmax >= tmin && tmin < t && tmax > 0) ? tmin : MISS;
}

float intersectTriangle(const Ray ray, const vec3 p0, const vec3 p1, const vec3 p2, out vec2 uv) {
    vec3 edge1, edge2, h, s, q;
    float a, f;
    edge1 = p1 - p0;
    edge2 = p2 - p0;
    h = cross(ray.dir, edge2);
    a = dot(edge1, h);

    // ray must hit from the front
    if (abs(a) < EPSILON) return -1;

    f = 1.0 / a;
    s = ray.org - p0;
    uv.x = f * dot(s, h);

    if (uv.x < 0.0 || uv.x > 1.0) return -1;

    q = cross(s, edge1);
    uv.y = f * dot(ray.dir, q);

    if (uv.y < 0.0 || uv.x + uv.y > 1.0) return -1;

    // return distance to hitpoint (may be negative)
    return f * dot(edge2, q);
}

vec3 fetchPosition(uint index) {
    return vec3(
    positions[index * 3 + 0],
    positions[index * 3 + 1],
    positions[index * 3 + 2]
    );
}

vec2 fetchTexCoord(uint index) {
    return vec2(
    texCoords[index * 2 + 0],
    texCoords[index * 2 + 1]
    );
}

vec3 fetchNormal(uint index) {
    return vec3(
    normals[index * 3 + 0],
    normals[index * 3 + 1],
    normals[index * 3 + 2]
    );
}

void traverseBVH(const Ray ray, inout float t, inout uint triangleIdx, inout vec2 triangleUV, inout uint intersections) {
    NodeStack stack;

    intersections++;
    stack.idx = intersectAABB(ray, nodes[0].aabb, t) == 1e30 ? 0 : 1;
    stack.nodes[0] = 0;

    while (stack.idx > 0) {
        Node node = nodes[stack.nodes[--stack.idx]];
        if (node.is_leaf) {
            for (uint i = node.a; i < node.a + node.b; i++) {
                intersections++;
                vec2 uv;
                float new_t = intersectTriangle(ray,
                    fetchPosition(triangles[i].p0),
                    fetchPosition(triangles[i].p1),
                    fetchPosition(triangles[i].p2),
                    uv
                );
                if (new_t > EPSILON && new_t < t) {
                    triangleIdx = i;
                    triangleUV = uv;
                    t = new_t;
                }
            }
        } else {
            intersections += 2;
            float dist0 = intersectAABB(ray, nodes[node.a].aabb, t);
            float dist1 = intersectAABB(ray, nodes[node.b].aabb, t);

            if (dist0 < dist1) {
                if (dist1 != 1e30) stack.nodes[stack.idx++] = node.b;
                if (dist0 != 1e30) stack.nodes[stack.idx++] = node.a;
            } else {
                if (dist0 != 1e30) stack.nodes[stack.idx++] = node.a;
                if (dist1 != 1e30) stack.nodes[stack.idx++] = node.b;
            }
        }
    }
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

const vec3 TOP_SKY = vec3(.5, .7, .9);
const vec3 BOTTOM_SKY = vec3(.2, .5, .8);
const vec3 SUN_DIR = normalize(vec3(1, 2, 1));
const vec3 SUN_COL = vec3(1, 0.97, 0.86);
const float AMBIENT = 0.5;
const float DIFFUSE = 0.8;
const float SPECULAR = 0.4;
const float SPEC_POW = 30.0;

vec3 skybox(const vec3 dir) {
    float sun_fac = clamp(pow(dot(dir, SUN_DIR), 200.0), 0, 1);
    float sky_fac = (dir.y + 1) * .5;
    return TOP_SKY * sky_fac + BOTTOM_SKY * (1 - sky_fac) + SUN_COL * sun_fac;
}

void main() {
    vec3 dir = texture(dirTex, (fragPos + 1) / 2).xyz;

    float t = 1000000;
    uint intersections = 0;
    uint triangleIdx = 0;
    vec2 uv = vec2(0);

    traverseBVH(Ray(org, dir, 1 / dir), t, triangleIdx, uv, intersections);

    if (t < 1000) {
        vec3 normal = triangleNormal(triangleIdx, uv);
        vec2 texCoord = triangleTexCoord(triangleIdx, uv);
        vec3 reflected = reflect(dir, normal);

        float shadow_t = 1000000;
        vec3 shadow_ray_dir = SUN_DIR;
        uint shadow_triangleIdx = 0;
        Ray shadow_ray = Ray(
            org + dir * t + normal * 0.0001,
            shadow_ray_dir, 1 / shadow_ray_dir
        );
        traverseBVH(shadow_ray, shadow_t, shadow_triangleIdx, uv, intersections);
        bool shadow = shadow_t < 1000;

        vec3 albedo = texture(triangles[triangleIdx].matIdx == 0 ? modelAlbedo1 : modelAlbedo2, vec2(texCoord.x, -texCoord.y)).xyz;
        vec3 ambient = albedo * skybox(normal) * AMBIENT;
        vec3 diffuse = albedo * SUN_COL * clamp(dot(normal, SUN_DIR), 0, 1) * DIFFUSE;
        vec3 specular = SUN_COL * clamp(pow(dot(reflected, SUN_DIR), SPEC_POW), 0, 1) * SPECULAR;

        if (shadow) {
            diffuse *= 0;
            specular *= 0;
        }

        fragCol = vec4(ambient + diffuse + specular, 1);
    } else {
        fragCol = vec4(skybox(dir), 1);
    }
}