use crate::raytracing::types::{BVHNode, Triangle, AABB};
use cgmath::Vector3;
use std::ops::Index;

fn centroid_axis(tri: &Triangle, vertices: &Vec<Vector3<f32>>, axis: usize) -> f32 {
    const THIRD: f32 = 1.0 / 3.0;
    (
        vertices[tri.p0 as usize].index(axis) +
        vertices[tri.p1 as usize].index(axis) +
        vertices[tri.p2 as usize].index(axis)
    ) * THIRD
}

pub struct BVHBuilder {
    vertices: Vec<Vector3<f32>>,
    triangles: Vec<Triangle>,
    nodes: Vec<BVHNode>,
}

impl BVHBuilder {
    pub fn new(vertices: Vec<Vector3<f32>>, triangles: Vec<Triangle>) -> Self {
        Self {
            vertices,
            triangles,
            nodes: vec![],
        }
    }

    pub fn build(mut self) -> (Vec<Vector3<f32>>, Vec<Triangle>, Vec<BVHNode>) {
        self.create_leaf_node(0, self.triangles.len() as i32);
        self.split_leaf_node(0);
        //self.split_leaf_node_sah(0); // much slower to trace for some reason ???
        // maybe because more memory / more nodes used
        // maybe some flaw in the construction or the sah evaluation is incorrect?
        (self.vertices, self.triangles, self.nodes)
    }

    fn create_leaf_node(&mut self, first_triangle: i32, triangle_count: i32) {
        let mut bounds = AABB::smallest_bounds();
        self.triangles[first_triangle as usize..(first_triangle + triangle_count) as usize]
            .iter()
            .for_each(|tri| {
                bounds.include(self.vertices[tri.p0 as usize]);
                bounds.include(self.vertices[tri.p1 as usize]);
                bounds.include(self.vertices[tri.p2 as usize]);
            });
        self.nodes
            .push(BVHNode::new_leaf(bounds, first_triangle, triangle_count))
    }

    fn evaluate_sah(&self, node_idx: usize, axis: usize, split_pos: f32) -> f32 {
        let mut left_aabb = AABB::smallest_bounds();
        let mut right_aabb = AABB::smallest_bounds();
        let mut left_count = 0;
        let mut right_count = 0;
        for i in 0..self.nodes[node_idx].triangle_count() as usize {
            let tri = &self.triangles[self.nodes[node_idx].first_triangle() as usize + i];
            if centroid_axis(tri, &self.vertices, axis) < split_pos {
                left_count += 1;
                left_aabb.include(self.vertices[tri.p0 as usize]);
                left_aabb.include(self.vertices[tri.p1 as usize]);
                left_aabb.include(self.vertices[tri.p2 as usize]);
            } else {
                right_count += 1;
                right_aabb.include(self.vertices[tri.p0 as usize]);
                right_aabb.include(self.vertices[tri.p1 as usize]);
                right_aabb.include(self.vertices[tri.p2 as usize]);
            }
        }
        let cost = left_count as f32 * left_aabb.area() + right_count as f32 * right_aabb.area();
        if cost > 0.0 { cost } else { 1e30 }
    }

    fn split_leaf_node_sah(&mut self, node_idx: usize) {
        let node_count = self.nodes.len() as i32;
        let first_triangle = self.nodes[node_idx].first_triangle() as usize;
        let triangle_count = self.nodes[node_idx].triangle_count() as usize;

        // if leaf has only one triangle, leave as leaf
        if triangle_count <= 1 {
            return;
        }

        // choose axis to split on
        let mut best_axis = 0;
        let mut best_split_pos: f32 = 0.0;
        let mut lowest_cost: f32 = 1e30;
        for candidate_axis in 0..3 {
            for i in 0..triangle_count {
                let candidate_split_pos = centroid_axis(&self.triangles[i], &self.vertices, candidate_axis);
                let cost = self.evaluate_sah(node_idx, candidate_axis, candidate_split_pos);
                if cost < lowest_cost {
                    best_axis = candidate_axis;
                    best_split_pos = candidate_split_pos;
                    lowest_cost = cost;
                }
            }
        }

        // sort triangles along axis
        let mut i = first_triangle;
        let mut j = i + triangle_count - 1;
        while i <= j {
            if centroid_axis(&self.triangles[i], &self.vertices, best_axis) < best_split_pos {
                i += 1;
            } else {
                self.triangles.swap(i, j);
                j -= 1;
            }
        }

        // if split node would contain all or no triangles, dont split
        let left_count = i - first_triangle;
        if left_count == 0 || left_count == triangle_count {
            return;
        }

        let right_child = node_count;
        let left_child = node_count + 1;

        // create and split child nodes
        self.nodes[node_idx].convert_to_node(right_child, left_child);

        self.create_leaf_node(first_triangle as i32, left_count as i32);
        self.create_leaf_node(i as i32, (triangle_count - left_count) as i32);

        self.split_leaf_node_sah(right_child as usize);
        self.split_leaf_node_sah(left_child as usize);
    }

    fn split_leaf_node(&mut self, node_idx: usize) {
        // todo: fix problems
        // problems:
        //  - split_pos is middle of bounding box, should be middle of all triangle positions
        //    this means, that sometimes all triangles fall on one side of the split, and there is no speedup
        //  - this method is recursive, should be iterative
        //  - triangles are being swapped, can impact performance if triangle struct get larger (uvs, material ids, etc.)
        //  - centroids are calculated newly every time, should be calculated once at start and reused
        // todo: implement surface area heuristics
        // decide to split, when split will result in lower surface area overall
        // all split positions should be checked for every axis, so that split with lowest SA can be found
        let node_count = self.nodes.len() as i32;
        let leaf = &mut self.nodes[node_idx];

        // if leaf has only one triangle, leave as leaf
        if leaf.triangle_count() <= 1 {
            return;
        }

        // choose axis to split on
        let size = leaf.bounds().max - leaf.bounds().min;
        let mut axis = 0;
        if size.y > size.x {
            axis = 1
        };
        if size.z > *size.index(axis) {
            axis = 2
        };
        let split_pos = *leaf.bounds().min.index(axis) + *size.index(axis) * 0.5;

        // sort triangles along axis
        let mut i = leaf.first_triangle() as usize;
        let mut j = i + leaf.triangle_count() as usize - 1;
        while i <= j {
            if centroid_axis(&self.triangles[i], &self.vertices, axis) < split_pos {
                i += 1;
            } else {
                self.triangles.swap(i, j);
                j -= 1;
            }
        }

        // if split node would contain all or no triangles, dont split
        let left_count = i as i32 - leaf.first_triangle();
        if left_count == 0 || left_count == leaf.triangle_count() {
            return;
        }

        let right_child = node_count;
        let left_child = node_count + 1;

        // create and split child nodes
        let first_tri = leaf.first_triangle();
        let tri_count = leaf.triangle_count();
        leaf.convert_to_node(right_child, left_child);

        self.create_leaf_node(first_tri, left_count);
        self.create_leaf_node(i as i32, tri_count - left_count);

        self.split_leaf_node(right_child as usize);
        self.split_leaf_node(left_child as usize);
    }
}
