#version 430

in vec2 fragPos;
out vec4 fragCol;

uniform sampler2D dirTex;
uniform vec3 org;

layout (std430, binding = 0) buffer vertexBuffer {
    float vertices[];
};

layout (std430, binding = 1) buffer indexBuffer {
    int indices[];
};

bool intersect(vec3 org, vec3 dir, vec3 p0, vec3 p1, vec3 p2) {
    const float EPSILON = 0.000001;
    vec3 edge1, edge2, h, s, q;
    float a, f, u, v;
    edge1 = p1 - p0;
    edge2 = p2 - p0;
    h = cross(dir, edge2);
    a = dot(edge1, h);

    if (a > -EPSILON && a < EPSILON) return false; // This ray is parallel to this triangle.

    f = 1.0 / a;
    s = org - p0;
    u = f * dot(s, h);

    if (u < 0.0 || u > 1.0) return false;

    q = cross(s, edge1);
    v = f * dot(dir, q);

    if (v < 0.0 || u + v > 1.0) return false;

    // At this stage we can compute t to find out where the intersection point is on the line.
    float t = f * dot(edge2, q);

    if (t > EPSILON) {
        // ray intersection
        //outIntersectionPoint = org + dir * t;
        return true;
    }
    else return false; // This means that there is a line intersection but not a ray intersection.
}

vec3 fetchVertex(int index) {
    return vec3(
    vertices[index * 3 + 0],
    vertices[index * 3 + 1],
    vertices[index * 3 + 2]
    );
}

void main() {
    vec3 dir = texture(dirTex, (fragPos + 1) / 2).xyz;

    int collider = -1;
    for (int i = 0; i < indices.length() / 3; i++) {
        if (intersect(
        org, dir,
        fetchVertex(indices[i * 3 + 0]),
        fetchVertex(indices[i * 3 + 1]),
        fetchVertex(indices[i * 3 + 2])
        )) {
            collider = i;
            break;
        }
    }

    fragCol = collider == -1 ? vec4(dir, 1) : vec4((float(collider) / indices.length()), 0, 0, 1);
    //fragCol = vec4(dir, 1);
}