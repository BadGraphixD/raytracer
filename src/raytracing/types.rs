use cgmath::{Array, Vector3};

pub struct Triangle {
    pub p0: u32,
    pub p1: u32,
    pub p2: u32,
}

impl Triangle {
    pub fn new(p0: u32, p1: u32, p2: u32) -> Self {
        Self { p0, p1, p2 }
    }
}

#[derive(Debug)]
pub struct AABB {
    pub min: Vector3<f32>,
    pub max: Vector3<f32>,
}

impl AABB {
    pub fn new(min: Vector3<f32>, max: Vector3<f32>) -> Self {
        Self { min, max }
    }
    pub fn smallest_bounds() -> Self {
        // Values squared must fit into f32, so that area of smallest box will not be NaN
        Self::new(Vector3::from_value(1e16), Vector3::from_value(-1e16))
    }
    pub fn include(&mut self, point: Vector3<f32>) {
        self.min.x = f32::min(self.min.x, point.x);
        self.min.y = f32::min(self.min.y, point.y);
        self.min.z = f32::min(self.min.z, point.z);

        self.max.x = f32::max(self.max.x, point.x);
        self.max.y = f32::max(self.max.y, point.y);
        self.max.z = f32::max(self.max.z, point.z);
    }
    pub fn area(&self) -> f32 {
        let e = self.max - self.min;
        e.x * e.y + e.y * e.z + e.z * e.x
    }
}

#[derive(Debug)]
pub struct BVHNode {
    bounds: AABB,
    is_leaf: u32,

    // can hold either:     right_node, left_node
    // or:                  first_triangle, triangle_count
    a: u32,
    b: u32,
}

impl BVHNode {
    pub fn new_node(bounds: AABB, right_node: u32, left_node: u32) -> Self {
        Self {
            bounds,
            is_leaf: 0,
            a: right_node,
            b: left_node,
        }
    }

    pub fn new_leaf(bounds: AABB, first_triangle: u32, triangle_count: u32) -> Self {
        Self {
            bounds,
            is_leaf: 1,
            a: first_triangle,
            b: triangle_count,
        }
    }

    pub fn convert_to_node(&mut self, right_node: u32, left_node: u32) {
        self.is_leaf = 0;
        self.a = right_node;
        self.b = left_node;
    }

    pub fn bounds(&self) -> &AABB {
        &self.bounds
    }
    pub fn is_leaf(&self) -> bool {
        self.is_leaf != 0
    }

    pub fn right_node(&self) -> u32 {
        self.a
    }
    pub fn left_node(&self) -> u32 {
        self.b
    }

    pub fn first_triangle(&self) -> u32 {
        self.a
    }
    pub fn triangle_count(&self) -> u32 {
        self.b
    }
}
