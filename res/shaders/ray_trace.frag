#version 430

#define MISS 1e30
#define EPSILON 0.000001
#define MAX_T 1000000
#define NODE_STACK_SIZE 100

in vec2 fragPos;
out vec4 fragCol;

uniform sampler2D dirTex;
uniform vec3 org;

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
    uint p0, p1, p2;
};

struct Intersection {
    float t;
    float u, v;
    uint tringleIdx;
};

struct NodeStack {
    uint nodes[NODE_STACK_SIZE];
    uint idx;
};

layout (std430, binding = 0) buffer nodeBuffer { Node nodes[]; };
layout (std430, binding = 1) buffer triangleBuffer { Triangle triangles[]; };
layout (std430, binding = 2) buffer positionBuffer { float positions[]; };

vec3 fetchPosition(uint index) {
    return vec3(
        positions[index * 3 + 0],
        positions[index * 3 + 1],
        positions[index * 3 + 2]
    );
}

float intersectAABB(const Ray ray, const AABB aabb, const float t) {
    float tx1 = (aabb.minx - ray.org.x) * ray.rDir.x, tx2 = (aabb.maxx - ray.org.x) * ray.rDir.x;
    float tmin = min(tx1, tx2), tmax = max(tx1, tx2);
    float ty1 = (aabb.miny - ray.org.y) * ray.rDir.y, ty2 = (aabb.maxy - ray.org.y) * ray.rDir.y;
    tmin = max(tmin, min(ty1, ty2)), tmax = min(tmax, max(ty1, ty2));
    float tz1 = (aabb.minz - ray.org.z) * ray.rDir.z, tz2 = (aabb.maxz - ray.org.z) * ray.rDir.z;
    tmin = max(tmin, min(tz1, tz2)), tmax = min(tmax, max(tz1, tz2));
    return (tmax >= tmin && tmin < t && tmax > 0) ? tmin : MISS;
}

void intersectTriangle(const Ray ray, const uint triangleIdx, inout Intersection i) {
    vec3 edge1, edge2, h, s, q, p0, p1, p2;
    float a, f, t, u, v;
    {
        Triangle triangle = triangles[triangleIdx];
        p0 = fetchPosition(triangle.p0);
        p1 = fetchPosition(triangle.p1);
        p2 = fetchPosition(triangle.p2);
    }
    edge1 = p1 - p0;
    edge2 = p2 - p0;
    h = cross(ray.dir, edge2);
    a = dot(edge1, h);

    // ray must hit from the front
    if (a < EPSILON) return;

    f = 1.0 / a;
    s = ray.org - p0;
    u = f * dot(s, h);

    if (u < 0.0 || u > 1.0) return;

    q = cross(s, edge1);
    v = f * dot(ray.dir, q);

    if (v < 0.0 || u + v > 1.0) return;

    t = f * dot(edge2, q);

    if (t > EPSILON && t < i.t) {
        i.t = t;
        i.u = u;
        i.v = v;
        i.tringleIdx = triangleIdx;
    }
}

void traverseBVH(const Ray ray, inout Intersection i) {
    NodeStack stack;

    stack.idx = intersectAABB(ray, nodes[0].aabb, i.t) == MISS ? 0 : 1;
    stack.nodes[0] = 0;

    while (stack.idx > 0) {
        Node node = nodes[stack.nodes[--stack.idx]];
        if (node.is_leaf) {
            for (uint idx = node.a; idx < node.a + node.b; idx++) {
                intersectTriangle(ray, idx, i);
            }
        } else {
            float dist0 = intersectAABB(ray, nodes[node.a].aabb, i.t);
            float dist1 = intersectAABB(ray, nodes[node.b].aabb, i.t);

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

void main() {
    vec3 dir = texture(dirTex, (fragPos + 1) / 2).xyz;
    Intersection i = Intersection(MAX_T, 0, 0, 0);
    Ray ray = Ray(org, dir, 1 / dir);
    traverseBVH(ray, i);
    fragCol = vec4(
        i.t, i.u, i.v,
        uintBitsToFloat(i.tringleIdx)
    );
}