#version 430

in vec2 fragPos;
out vec4 fragCol;

uniform sampler2D dirTex;
uniform sampler2D modelAlbedo;
uniform vec3 org;
uniform bool hasTexCoords;
uniform bool hasNormals;

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

layout (std430, binding = 0) buffer nodeBuffer {
    Node nodes[];
};

layout (std430, binding = 1) buffer triangleBuffer {
    Triangle triangles[];
};

layout (std430, binding = 2) buffer positionBuffer {
    float positions[];
};

layout (std430, binding = 3) buffer texCoordBuffer {
    float texCoords[];
};

layout (std430, binding = 4) buffer normalBuffer {
    float normals[];
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

float intersectTriangle(const Ray ray, const vec3 p0, const vec3 p1, const vec3 p2, out vec2 uv) {
    vec3 edge1, edge2, h, s, q;
    float a, f;
    edge1 = p1 - p0;
    edge2 = p2 - p0;
    h = cross(ray.dir, edge2);
    a = dot(edge1, h);

    //if (abs(a) < EPSILON) return -1; // This ray is parallel to this triangle.
    if (a < EPSILON) return -1; // This ray is parallel to this triangle or hitting it from the back

    f = 1.0 / a;
    s = ray.org - p0;
    uv.x = f * dot(s, h);

    if (uv.x < 0.0 || uv.x > 1.0) return -1;

    q = cross(s, edge1);
    uv.y = f * dot(ray.dir, q);

    if (uv.y < 0.0 || uv.x + uv.y > 1.0) return -1;

    // At this stage we can compute t to find out where the intersection point is on the line.
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
    // only push root onto stack when ray hits it
    // dont remove this, even with tlas, because the root node
    // of the blas may be smaller than the leaf node of the tlas (translation)
    stack.idx = intersectAABB(ray, nodes[0].aabb, t) == 1e30 ? 0 : 1;
    stack.nodes[0] = 0;

    while (stack.idx > 0) {
        // we assume, that all nodes in the stack were hit
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
        return n1 * uv.x + n2 * uv.y + n0 * w;
    } else {
        vec3 v0 = fetchPosition(triangles[idx].p0);
        vec3 v1 = fetchPosition(triangles[idx].p1);
        vec3 v2 = fetchPosition(triangles[idx].p2);
        return normalize(cross(v1 - v0, v2 - v0));
    }
}

void main() {
    vec3 dir = texture(dirTex, (fragPos + 1) / 2).xyz;

    float t = 1000000;
    uint intersections = 0;
    uint triangleIdx = 0;
    vec2 uv = vec2(0);

    traverseBVH(Ray(org, dir, 1 / dir), t, triangleIdx, uv, intersections);

    vec3 normal = triangleNormal(triangleIdx, uv);
    vec2 texCoord = triangleTexCoord(triangleIdx, uv);
    vec3 reflected = reflect(dir, normal);

    vec3 albedo = texture(modelAlbedo, vec2(texCoord.x, -texCoord.y)).xyz;

    fragCol = t > 1000 ? vec4(dir, 1) : vec4(albedo, 1);
    //fragCol = t > 1000 ? vec4(dir, 1) : vec4(texCoord, 0, 1);
    //fragCol = t > 1000 ? vec4(dir, 1) : vec4(normal, 1);
    //fragCol = t > 1000 ? vec4(dir, 1) : vec4(uv, 0, 1);
    //fragCol = t > 1000 ? vec4(dir, 1) : vec4(reflected, 1);
    //fragCol = vec4(intersections / 100.0, 0, t > 1000 ? 0 : 1, 1);
    //fragCol = vec4(intersections / 100.0, 0, 0, 1);
    //fragCol = t > 1000 ? vec4(dir, 1) : vec4(org + dir * t, 1);
}