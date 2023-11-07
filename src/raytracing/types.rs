use cgmath::{Vector3, Zero};
use crate::util::error::ModelParseError;

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

#[derive(Copy, Clone)]
pub struct AABB {
    pub min: Vector3<f32>,
    pub max: Vector3<f32>,
}

impl AABB {
    pub fn new(min: Vector3<f32>, max: Vector3<f32>) -> Self {
        Self { min, max }
    }

    pub fn include(&mut self, point: &Vector3<f32>) {
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

#[derive(Copy, Clone)]
pub struct Bin {
    pub bounds: AABBBuilder,
    pub tri_count: u32,
}

impl Bin {
    pub const fn new() -> Self {
        Self {
            bounds: AABBBuilder::new(),
            tri_count: 0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct AABBBuilder {
    aabb: Option<AABB>,
}

impl AABBBuilder {
    pub const fn new() -> Self {
        Self { aabb: None }
    }

    pub fn include(&mut self, point: &Vector3<f32>) {
        match &mut self.aabb {
            None => self.aabb = Some(AABB::new(point.clone(), point.clone())),
            Some(aabb) => aabb.include(point),
        }
    }

    pub fn include_other(&mut self, other: &AABBBuilder) {
        match other.aabb {
            None => {}
            Some(aabb) => {
                self.include(&aabb.min);
                self.include(&aabb.max);
            }
        }
    }

    pub fn build(self) -> AABB {
        match self.aabb {
            None => AABB::new(Vector3::zero(), Vector3::zero()),
            Some(aabb) => aabb,
        }
    }

    pub fn area(&self) -> f32 {
        match &self.aabb {
            None => 0.0,
            Some(aabb) => aabb.area(),
        }
    }
}

pub struct BVHNode {
    bounds: AABB,
    is_leaf: u32,

    // can hold either:     right_node, left_node
    // or:                  first_triangle, triangle_count
    a: u32,
    b: u32,
}

impl BVHNode {
    pub fn new_leaf(bounds: AABB, first_triangle: u32, triangle_count: u32) -> Self {
        Self {
            bounds,
            is_leaf: 1,
            a: first_triangle,
            b: triangle_count,
        }
    }

    pub fn new_dummy() -> Self {
        Self {
            bounds: AABB::new(Vector3::zero(), Vector3::zero()),
            is_leaf: 1, a: 0, b: 0
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

    pub fn first_triangle(&self) -> u32 {
        self.a
    }

    pub fn triangle_count(&self) -> u32 {
        self.b
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct IndexBundle {
    pub pos_idx: i32,
    pub tex_idx: i32,
    pub nor_idx: i32,
}

impl IndexBundle {
    pub fn new(input: &Vec<&str>) -> Result<Self, ModelParseError> {
        let mut ib = Self::new_default();
        if input.len() > 0 && !input[0].is_empty() { ib.pos_idx = Self::parse(input[0])? }
        if input.len() > 1 && !input[1].is_empty() { ib.tex_idx = Self::parse(input[1])? }
        if input.len() > 2 && !input[2].is_empty() { ib.nor_idx = Self::parse(input[2])? }
        Ok(ib)
    }

    pub fn normalize(&mut self, pos_len: i32, tex_len: i32, nor_len: i32) {
        if self.pos_idx < 0 { self.pos_idx += pos_len }
        if self.tex_idx < 0 { self.tex_idx += tex_len }
        if self.nor_idx < 0 { self.nor_idx += nor_len }
    }

    pub fn new_default() -> Self {
        Self {
            pos_idx: 0,
            tex_idx: 0,
            nor_idx: 0,
        }
    }

    pub fn parse(str: &str) -> Result<i32, ModelParseError> {
        let idx = str.trim().parse::<i32>().map_err(|e| ModelParseError::ParseIntError(e, str.to_owned()))?;
        Ok(if idx > 0 { idx - 1 } else { idx })
    }
}
