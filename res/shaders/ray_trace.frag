#version 430

in vec2 fragPos;
out vec4 fragCol;

uniform sampler2D dirTex;
uniform vec3 org;

const float EPSILON = 0.000001;
const int NODE_STACK_SIZE = 100;

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

struct NodeStack {
    uint nodes[NODE_STACK_SIZE];
    uint idx;
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

float intersectAABB(const Ray ray, const AABB aabb, const float t) {
    float tx1 = (aabb.minx - ray.org.x) * ray.rDir.x, tx2 = (aabb.maxx - ray.org.x) * ray.rDir.x;
    float tmin = min( tx1, tx2 ), tmax = max( tx1, tx2 );
    float ty1 = (aabb.miny - ray.org.y) * ray.rDir.y, ty2 = (aabb.maxy - ray.org.y) * ray.rDir.y;
    tmin = max( tmin, min( ty1, ty2 ) ), tmax = min( tmax, max( ty1, ty2 ) );
    float tz1 = (aabb.minz - ray.org.z) * ray.rDir.z, tz2 = (aabb.maxz - ray.org.z) * ray.rDir.z;
    tmin = max( tmin, min( tz1, tz2 ) ), tmax = min( tmax, max( tz1, tz2 ) );
    return (tmax >= tmin && tmin < t && tmax > 0) ? tmin : 1e30;
    // 1e30 denotes miss
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

vec3 fetchVertex(uint index) {
    return vec3(
        vertices[index * 3 + 0],
        vertices[index * 3 + 1],
        vertices[index * 3 + 2]
    );
}

void traverseBVH(const Ray ray, inout float t, inout uint triangleIdx, inout uint intersections) {
    NodeStack stack;

    intersections++;
    // only push root onto stack when ray hits it
    stack.idx = intersectAABB(ray, nodes[0].aabb, t) == 1e30 ? 0 : 1;
    stack.nodes[0] = 0;

    while (stack.idx > 0) {
        // we assume, that all nodes in the stack were hit
        Node node = nodes[stack.nodes[--stack.idx]];
        if (node.is_leaf) {
            for (uint i = node.a; i < node.a + node.b; i++) {
                intersections++;
                float new_t = intersectTriangle(ray,
                    fetchVertex(triangles[i].p0),
                    fetchVertex(triangles[i].p1),
                    fetchVertex(triangles[i].p2)
                );
                if (new_t > EPSILON && new_t < t) {
                    triangleIdx = i;
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

            /*
            // todo: test if this is faster
            // note: probably not
            // Less branching:
            int ordered = int(dist0 < dist1);
            uint first = node.b * ordered + node.a * (1 - ordered);
            uint second = node.a * ordered + node.b * (1 - ordered);
            float firstDist = dist1 * ordered + dist0 * (1 - ordered);
            float secondDist = dist0 * ordered + dist1 * (1 - ordered);
            if (firstDist != 1e30) stack.nodes[stack.idx++] = first;
            if (secondDist != 1e30) stack.nodes[stack.idx++] = second;
            */
        }
    }
}

vec3 triangleNormal(const uint idx) {
    vec3 v0 = fetchVertex(triangles[idx].p0);
    vec3 v1 = fetchVertex(triangles[idx].p1);
    vec3 v2 = fetchVertex(triangles[idx].p2);
    return normalize(cross(v1 - v0, v2 - v0));
}

void main() {
    vec3 dir = texture(dirTex, (fragPos + 1) / 2).xyz;

    float t = 1000000;
    uint intersections = 0;
    uint triangleIdx = 0;

    /*
    // OVERKILL (400 objects):
    const uint range = 20;
    for (int i = 0; i < range; i++) {
        for (int j = 0; j < range; j++) {
            traverseBVH(Ray(org + vec3(i * 14, 0, j * 6), dir, 1 / dir), t, triangleIdx, intersections);
        }
    }
    */

    traverseBVH(Ray(org, dir, 1 / dir), t, triangleIdx, intersections);

    vec3 normal = triangleNormal(triangleIdx);
    vec3 reflected = reflect(dir, normal);

    //fragCol = t > 1000 ? vec4(dir, 1) : vec4(reflected, 1);
    fragCol = vec4(intersections / 100.0, 0, t > 1000 ? 0 : 1, 1);
    //fragCol = vec4(intersections / 100.0, 0, 0, 1);
    //fragCol = t > 1000 ? vec4(dir, 1) : vec4(org + dir * t, 1);
}