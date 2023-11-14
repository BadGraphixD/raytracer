use std::collections::{HashMap, HashSet};
use cgmath::{Vector2, Vector3};
use crate::raytracing::bvh::{BVH, BVHBuilder};
use crate::raytracing::types::{IndexBundle, Triangle};

pub struct Model {
    triangles: Vec<Triangle>,
    indices: Vec<u32>,
    positions: Vec<Vector3<f32>>,
    tex_coords: Option<Vec<Vector2<f32>>>,
    normals: Option<Vec<Vector3<f32>>>,

    material_libs: Vec<String>,
    materials: Vec<String>,

    bvh: Option<BVH>,
}

impl Model {
    pub fn triangles(&self) -> &Vec<Triangle> { &self.triangles }
    pub fn indices(&self) -> &Vec<u32> { &self.indices }
    pub fn positions(&self) -> &Vec<Vector3<f32>> { &self.positions }
    pub fn tex_coords(&self) -> &Option<Vec<Vector2<f32>>> { &self.tex_coords }
    pub fn normals(&self) -> &Option<Vec<Vector3<f32>>> { &self.normals }

    pub fn has_tex_coords(&self) -> bool { self.tex_coords.is_some() }
    pub fn has_normals(&self) -> bool { self.normals.is_some() }

    pub fn set_triangles(&mut self, triangles: Vec<Triangle>) {
        self.triangles = triangles;
    }
    pub fn set_indices(&mut self, indices: Vec<u32>) {
        self.indices = indices;
    }

    pub fn build_bvh(&mut self) { self.bvh = Some(BVHBuilder::new(self).build()) }

    pub fn get_material_libs(&self) -> &Vec<String> { &self.material_libs }
    pub fn get_materials(&self) -> &Vec<String> { &self.materials }

    pub fn get_bvh(&self) -> Option<&BVH> { self.bvh.as_ref() }
}

struct IBTriangle {
    index_bundles: [IndexBundle; 3],
    mat_idx: u32,
}

impl IBTriangle {
    pub fn new(i0: IndexBundle, i1: IndexBundle, i2: IndexBundle, mat_idx: u32) -> Self {
        Self { index_bundles: [i0, i1, i2], mat_idx, }
    }
}

pub struct ModelBuilder {
    indices: Vec<IBTriangle>,
    positions: Vec<Vector3<f32>>,
    tex_coords: Vec<Vector2<f32>>,
    normals: Vec<Vector3<f32>>,

    material_libs: HashSet<String>,
    materials: HashMap<String, u32>,

    current_mat: u32,
}

impl ModelBuilder {
    pub fn new() -> Self {
        Self {
            indices: vec![],
            positions: vec![],
            tex_coords: vec![],
            normals: vec![],
            material_libs: HashSet::new(),
            materials: HashMap::new(),
            current_mat: 0,
        }
    }

    pub fn add_indices(&mut self, i0: IndexBundle, i1: IndexBundle, i2: IndexBundle, mat_idx: u32) {
        self.indices.push(IBTriangle::new(i0, i1, i2, mat_idx));
    }

    pub fn add_position(&mut self, position: Vector3<f32>) { self.positions.push(position) }
    pub fn add_tex_coord(&mut self, tex_coord: Vector2<f32>) { self.tex_coords.push(tex_coord) }
    pub fn add_normal(&mut self, normal: Vector3<f32>) { self.normals.push(normal) }

    pub fn add_material_lib(&mut self, lib: String) { self.material_libs.insert(lib); }
    pub fn add_material(&mut self, mat: String) {
        if let Some(id) = self.materials.get(&mat) {
            self.current_mat = *id;
        } else {
            let next_id = self.materials.len() as u32;
            self.materials.insert(mat, next_id);
            self.current_mat = next_id;
        }
    }

    pub fn get_current_mat(&self) -> u32 { self.current_mat }

    pub fn build(self) -> Model {
        let mut bundle_map: HashMap<IndexBundle, u32> = HashMap::new();
        let mut new_triangles: Vec<Triangle> = vec![];
        let mut new_indices: Vec<u32> = vec![];
        let mut new_positions: Vec<Vector3<f32>> = vec![];
        let mut new_tex_coords: Vec<Vector2<f32>> = vec![];
        let mut new_normals: Vec<Vector3<f32>> = vec![];

        let has_tex_coords = self.tex_coords.len() > 0;
        let has_normals = self.normals.len() > 0;

        let pos_len = self.positions.len() as i32;
        let tex_len = self.tex_coords.len() as i32;
        let nor_len = self.normals.len() as i32;

        let mut new_idx = 0;
        self.indices.into_iter().for_each(|ib_tri| {
            let indices = ib_tri.index_bundles.into_iter().map(|mut ib| {
                ib.normalize(pos_len, tex_len, nor_len);
                if let Some(idx) = bundle_map.get(&ib) { *idx }
                else {
                    new_positions.push(self.positions[ib.pos_idx as usize]);
                    if has_tex_coords { new_tex_coords.push(self.tex_coords[ib.tex_idx as usize]) }
                    if has_normals { new_normals.push(self.normals[ib.nor_idx as usize]) }
                    bundle_map.insert(ib, new_idx);
                    new_idx += 1;
                    new_idx - 1
                }
            }).collect::<Vec<u32>>();
            new_triangles.push(Triangle::new(indices[0], indices[1], indices[2], ib_tri.mat_idx));
            new_indices.push(indices[0]);
            new_indices.push(indices[1]);
            new_indices.push(indices[2]);
        });

        let mut sorted_materials: Vec<String> = vec![String::new(); self.materials.len()];
        self.materials.into_iter().for_each(|(k, v)| sorted_materials[v as usize] = k);

        Model {
            triangles: new_triangles,
            indices: new_indices,
            positions: new_positions,
            tex_coords: if has_tex_coords { Some(new_tex_coords) } else { None },
            normals: if has_normals { Some(new_normals) } else { None },
            material_libs: self.material_libs.into_iter().collect(),
            materials: sorted_materials,
            bvh: None,
        }
    }
}