use cgmath::{Vector2, Vector3};
use crate::raytracing::types::Triangle;

pub struct Model {
    triangles: Vec<Triangle>,
    positions: Vec<Vector3<f32>>,
    uvs: Option<Vec<Vector2<f32>>>,
    normals: Option<Vec<Vector3<f32>>>,
}

impl Model {
    pub fn triangles(&self) -> &Vec<Triangle> { &self.triangles }
    pub fn positions(&self) -> &Vec<Vector3<f32>> { &self.positions }
    pub fn uvs(&self) -> &Option<Vec<Vector2<f32>>> { &self.uvs }
    pub fn normals(&self) -> &Option<Vec<Vector3<f32>>> { &self.normals }

    pub fn has_uvs(&self) -> bool { self.uvs.is_some() }
    pub fn has_normals(&self) -> bool { self.normals.is_some() }

    pub fn set_triangles(&mut self, triangles: Vec<Triangle>) {
        self.triangles = triangles;
    }
}

pub struct ModelBuilder {
    triangles: Vec<Triangle>,
    positions: Vec<Vector3<f32>>,
    uvs: Vec<Vector2<f32>>,
    normals: Vec<Vector3<f32>>,
}

impl ModelBuilder {
    pub fn new() -> Self {
        Self {
            triangles: vec![],
            positions: vec![],
            uvs: vec![],
            normals: vec![],
        }
    }

    pub fn add_position_indices(&mut self, i0: u32, i1: u32, i2: u32) {
        // todo
        self.triangles.push(Triangle::new(i0, i1, i2));
    }

    pub fn add_uv_indices(&mut self, i0: u32, i1: u32, i2: u32) {
        // todo
    }

    pub fn add_normal_indices(&mut self, i0: u32, i1: u32, i2: u32) {
        // todo
    }

    pub fn add_position(&mut self, position: Vector3<f32>) { self.positions.push(position) }
    pub fn add_uv(&mut self, uv: Vector2<f32>) { self.uvs.push(uv) }
    pub fn add_normal(&mut self, normal: Vector3<f32>) { self.normals.push(normal) }

    pub fn build(self) -> Model {
        // todo: algorithm that expands position, uv and normal data, so that vertices of a
        // todo: triangle aren't split up to different locations (meaning, each triangle)
        // todo: only needs 3 indices to access all of its data

        // currently all uv and normal data is discarded
        Model {
            triangles: self.triangles,
            positions: self.positions,
            uvs: if self.uvs.len() == 0 { None } else { Some(self.uvs) },
            normals: if self.normals.len() == 0 { None } else { Some(self.normals) },
        }
    }
}