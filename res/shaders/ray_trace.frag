#version 430

in vec2 fragPos;
out vec4 fragCol;

uniform sampler2D dirTex;
uniform vec3 org;

const float EPSILON = 0.000001;
const int NODE_STACK_SIZE = 100;

struct Ray {
    vec3 org, dir;
};

struct AABB {
    float minx, miny, minz;
    float maxx, maxy, maxz;
};

struct Node {
    AABB aabb;
    bool is_leaf;
    int a, b;
};

struct Triangle {
    int p0, p1, p2;
};

struct NodeStack {
    int nodes[NODE_STACK_SIZE];
    int idx;
};

layout (std430, binding = 0) buffer vertexBuffer {
    float vertices[];
};

layout (std430, binding = 1) buffer triangleBuffer {
    Triangle triangles[];
};

layout (std430, binding = 2) buffer nodeBuffer {
    Node nodes[];
};

bool intersectAABB(const Ray ray, const AABB aabb, const float t) {
    float tx1 = (aabb.minx - ray.org.x) / ray.dir.x, tx2 = (aabb.maxx - ray.org.x) / ray.dir.x;
    float tmin = min( tx1, tx2 ), tmax = max( tx1, tx2 );
    float ty1 = (aabb.miny - ray.org.y) / ray.dir.y, ty2 = (aabb.maxy - ray.org.y) / ray.dir.y;
    tmin = max( tmin, min( ty1, ty2 ) ), tmax = min( tmax, max( ty1, ty2 ) );
    float tz1 = (aabb.minz - ray.org.z) / ray.dir.z, tz2 = (aabb.maxz - ray.org.z) / ray.dir.z;
    tmin = max( tmin, min( tz1, tz2 ) ), tmax = min( tmax, max( tz1, tz2 ) );
    return tmax >= tmin && tmin < t && tmax > 0;
}

float intersectTriangle(const Ray ray, const vec3 p0, const vec3 p1, const vec3 p2) {
    vec3 edge1, edge2, h, s, q;
    float a, f, u, v;
    edge1 = p1 - p0;
    edge2 = p2 - p0;
    h = cross(ray.dir, edge2);
    a = dot(edge1, h);

    if (a > -EPSILON && a < EPSILON) return -1; // This ray is parallel to this triangle.

    f = 1.0 / a;
    s = ray.org - p0;
    u = f * dot(s, h);

    if (u < 0.0 || u > 1.0) return -1;

    q = cross(s, edge1);
    v = f * dot(ray.dir, q);

    if (v < 0.0 || u + v > 1.0) return -1;

    // At this stage we can compute t to find out where the intersection point is on the line.
    return f * dot(edge2, q);
}

vec3 fetchVertex(int index) {
    return vec3(
        vertices[index * 3 + 0],
        vertices[index * 3 + 1],
        vertices[index * 3 + 2]
    );
}

void traverseBVH(const Ray ray, inout float t, out int intersections) {
    NodeStack stack;
    stack.nodes[0] = 0;
    stack.idx = 1;

    while (stack.idx > 0) {
        Node node = nodes[stack.nodes[--stack.idx]];

        intersections++;
        if (!intersectAABB(ray, node.aabb, t)) continue;
        if (node.is_leaf) {
            for (int i = node.a; i < node.a + node.b; i++) {
                intersections++;
                float new_t = intersectTriangle(ray,
                    fetchVertex(triangles[i].p0),
                    fetchVertex(triangles[i].p1),
                    fetchVertex(triangles[i].p2)
                );
                if (new_t > EPSILON && new_t < t) {
                    t = new_t;
                }
            }
        } else {
            stack.nodes[stack.idx++] = node.a;
            stack.nodes[stack.idx++] = node.b;
        }
    }
}

void main() {
    vec3 dir = texture(dirTex, (fragPos + 1) / 2).xyz;

    float t = 1000000;
    int intersections = 0;
    traverseBVH(Ray(org, dir), t, intersections);

    //fragCol = vec4(intersections / 100.0, 0, 0, 1);
    fragCol = t > 1000 ? vec4(dir, 1) : vec4(org + dir * t, 1);
}