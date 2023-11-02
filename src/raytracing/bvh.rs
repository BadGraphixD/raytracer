use crate::raytracing::types::{BVHNode, Triangle, AABB};
use cgmath::Vector3;
use std::ops::Index;

fn centroid(tri: &Triangle, vertices: &Vec<Vector3<f32>>) -> Vector3<f32> {
    const THIRD: f32 = 1.0 / 3.0;
    (vertices[tri.p0 as usize] + vertices[tri.p1 as usize] + vertices[tri.p2 as usize]) * THIRD
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
        let mut i = leaf.first_triangle();
        let mut j = i + leaf.triangle_count() - 1;
        while i <= j {
            if *centroid(&self.triangles[i as usize], &self.vertices).index(axis) < split_pos {
                i += 1;
            } else {
                self.triangles.swap(i as usize, j as usize);
                j -= 1;
            }
        }

        // if split node would contain all or no triangles, dont split
        let left_count = i - leaf.first_triangle();
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
        self.create_leaf_node(i, tri_count - left_count);

        self.split_leaf_node(right_child as usize);
        self.split_leaf_node(left_child as usize);
    }
}
