use cgmath::{Vector2, Vector3};
use crate::raytracing::types::Triangle;

pub struct Model {
    triangles: Vec<Triangle>,
    positions: Vec<Vector3<f32>>,
    tex_coords: Option<Vec<Vector2<f32>>>,
    normals: Option<Vec<Vector3<f32>>>,
}

impl Model {
    pub fn triangles(&self) -> &Vec<Triangle> { &self.triangles }
    pub fn positions(&self) -> &Vec<Vector3<f32>> { &self.positions }
    pub fn tex_coords(&self) -> &Option<Vec<Vector2<f32>>> { &self.tex_coords }
    pub fn normals(&self) -> &Option<Vec<Vector3<f32>>> { &self.normals }

    pub fn has_tex_coords(&self) -> bool { self.tex_coords.is_some() }
    pub fn has_normals(&self) -> bool { self.normals.is_some() }

    pub fn set_triangles(&mut self, triangles: Vec<Triangle>) {
        self.triangles = triangles;
    }
}

pub struct ModelBuilder {
    triangles: Vec<Triangle>,
    positions: Vec<Vector3<f32>>,
    tex_coords: Vec<Vector2<f32>>,
    normals: Vec<Vector3<f32>>,
}

impl ModelBuilder {
    pub fn new() -> Self {
        Self {
            triangles: vec![],
            positions: vec![],
            tex_coords: vec![],
            normals: vec![],
        }
    }

    pub fn add_indices(&mut self, p0: u32, p1: u32, p2: u32, t0: u32, t1: u32, t2: u32, n0: u32, n1: u32, n2: u32) {
        // TEMPORARY !!!
        self.triangles.push(Triangle::new(p0, p1, p2, t0, t1, t2, n0, n1, n2));
    }

    pub fn add_position_indices(&mut self, i0: u32, i1: u32, i2: u32) {
        // todo
        self.triangles.push(Triangle::new(i0, i1, i2, 0, 0, 0, 0, 0, 0));
    }

    pub fn add_tex_coord_indices(&mut self, i0: u32, i1: u32, i2: u32) {
        // todo
    }

    pub fn add_normal_indices(&mut self, i0: u32, i1: u32, i2: u32) {
        // todo
    }

    pub fn add_position(&mut self, position: Vector3<f32>) { self.positions.push(position) }
    pub fn add_tex_coord(&mut self, tex_coord: Vector2<f32>) { self.tex_coords.push(tex_coord) }
    pub fn add_normal(&mut self, normal: Vector3<f32>) { self.normals.push(normal) }

    pub fn build(self) -> Model {
        // todo: algorithm that expands position, tex_coord and normal data, so that vertices of a
        // todo: triangle aren't split up to different locations (meaning, each triangle)
        // todo: only needs 3 indices to access all of its data

        // currently all tex_coord and normal data is discarded
        Model {
            triangles: self.triangles,
            positions: self.positions,
            tex_coords: if self.tex_coords.len() == 0 { None } else { Some(self.tex_coords) },
            normals: if self.normals.len() == 0 { None } else { Some(self.normals) },
        }
    }
}