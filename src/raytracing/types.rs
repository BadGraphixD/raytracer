use cgmath::{Array, Vector3};

pub struct Triangle {
    pub p0: i32,
    pub p1: i32,
    pub p2: i32,
}

impl Triangle {
    pub fn new(p0: i32, p1: i32, p2: i32) -> Self {
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
        Self::new(Vector3::from_value(f32::MAX), Vector3::from_value(f32::MIN))
    }
    pub fn include(&mut self, point: Vector3<f32>) {
        self.min.x = f32::min(self.min.x, point.x);
        self.min.y = f32::min(self.min.y, point.y);
        self.min.z = f32::min(self.min.z, point.z);

        self.max.x = f32::max(self.max.x, point.x);
        self.max.y = f32::max(self.max.y, point.y);
        self.max.z = f32::max(self.max.z, point.z);
    }
}

#[derive(Debug)]
pub struct BVHNode {
    bounds: AABB,
    is_leaf: bool,

    // can hold either:     right_node, left_node
    // or:                  first_triangle, triangle_count
    a: i32,
    b: i32,
}

impl BVHNode {
    pub fn new_node(bounds: AABB, right_node: i32, left_node: i32) -> Self {
        Self { bounds, is_leaf: false, a: right_node, b: left_node }
    }

    pub fn new_leaf(bounds: AABB, first_triangle: i32, triangle_count: i32) -> Self {
        Self { bounds, is_leaf: true, a: first_triangle, b: triangle_count }
    }

    pub fn convert_to_node(&mut self, right_node: i32, left_node: i32) {
        self.is_leaf = false;
        self.a = right_node;
        self.b = left_node;
    }

    pub fn bounds(&self) -> &AABB { &self.bounds }
    pub fn is_leaf(&self) -> bool { self.is_leaf }

    pub fn right_node(&self) -> i32 { self.a }
    pub fn left_node(&self) -> i32 { self.b }

    pub fn first_triangle(&self) -> i32 { self.a }
    pub fn triangle_count(&self) -> i32 { self.b }
}