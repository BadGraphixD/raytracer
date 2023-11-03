use std::ops::Index;
use crate::raytracing::types::{BVHNode, Triangle, AABB};
use cgmath::Vector3;

struct BVHTriangle {
    p0: usize,
    p1: usize,
    p2: usize,
    centroid: [f32; 3],
}

impl BVHTriangle {
    fn new(triangle: &Triangle, vertices: &Vec<Vector3<f32>>) -> Self {
        const THIRD: f32 = 1.0 / 3.0;
        let centroid = (
            vertices[triangle.p0 as usize] +
            vertices[triangle.p1 as usize] +
            vertices[triangle.p2 as usize]
        ) * THIRD;
        Self {
            p0: triangle.p0 as usize,
            p1: triangle.p1 as usize,
            p2: triangle.p2 as usize,
            centroid: [ centroid.x, centroid.y, centroid.z ]
        }
    }
    fn to_tri(&self) -> Triangle {
        Triangle {
            p0: self.p0 as u32,
            p1: self.p1 as u32,
            p2: self.p2 as u32,
        }
    }
}

pub struct BVHBuilder {
    vertices: Vec<Vector3<f32>>,
    triangles: Vec<BVHTriangle>,
    triangle_indices: Vec<usize>,
    nodes: Vec<BVHNode>,
    total_sah: f32,
}

impl BVHBuilder {
    pub fn new(vertices: Vec<Vector3<f32>>, triangles: Vec<Triangle>) -> Self {
        Self {
            triangles: triangles.iter().map(|tri| BVHTriangle::new(tri, &vertices)).collect(),
            triangle_indices: (0..triangles.len()).into_iter().collect(),
            nodes: vec![],
            vertices,
            total_sah: 0.0,
        }
    }

    pub fn build(mut self) -> (Vec<Vector3<f32>>, Vec<Triangle>, Vec<BVHNode>) {
        self.create_leaf_node(0, self.triangles.len());
        self.split_leaf_node(0);
        println!("{}", self.total_sah);
        (self.vertices,
         self.triangle_indices.iter().map(|idx| self.triangles[*idx].to_tri()).collect(),
         self.nodes)
    }

    fn create_leaf_node(&mut self, first_triangle: usize, triangle_count: usize) {
        let mut bounds = AABB::smallest_bounds();
        self.triangle_indices[first_triangle..first_triangle + triangle_count]
            .iter().map(|idx| self.fetch_triangle(*idx)).for_each(|tri| {
                bounds.include(self.vertices[tri.p0]);
                bounds.include(self.vertices[tri.p1]);
                bounds.include(self.vertices[tri.p2]);
        });
        self.nodes.push(BVHNode::new_leaf(bounds, first_triangle as u32, triangle_count as u32))
    }

    fn fetch_triangle(&self, index: usize) -> &BVHTriangle {
        &self.triangles[self.triangle_indices[index]]
    }

    fn fetch_node(&self, index: usize) -> &BVHNode {
        &self.nodes[index]
    }
    fn fetch_node_mut(&mut self, index: usize) -> &mut BVHNode {
        &mut self.nodes[index]
    }

    fn evaluate_sah(&self, node_idx: usize, axis: usize, split_pos: f32) -> f32 {
        let mut left_aabb = AABB::smallest_bounds();
        let mut right_aabb = AABB::smallest_bounds();
        let mut left_count = 0;
        let mut right_count = 0;
        let node = self.fetch_node(node_idx);
        for i in 0..node.triangle_count() {
            let tri = self.fetch_triangle((node.first_triangle() + i) as usize);
            if tri.centroid[axis] > split_pos {
                left_count += 1;
                left_aabb.include(self.vertices[tri.p0]);
                left_aabb.include(self.vertices[tri.p1]);
                left_aabb.include(self.vertices[tri.p2]);
            } else {
                right_count += 1;
                right_aabb.include(self.vertices[tri.p0]);
                right_aabb.include(self.vertices[tri.p1]);
                right_aabb.include(self.vertices[tri.p2]);
            }
        }
        left_count as f32 * left_aabb.area() + right_count as f32 * right_aabb.area()
    }

    fn split_triangles_along_plane(&mut self, mut first: usize, mut last: usize, axis: usize, position: f32) -> usize {
        loop {
            if self.fetch_triangle(first).centroid[axis] < position {
                first += 1;
                if first > last { break }
            } else {
                self.triangles.swap(first, last);
                if last <= first { break }
                last -= 1;
            }
        }
        first
    }

    fn split_leaf_node(&mut self, node_idx: usize) {
        let node_count = self.nodes.len();
        let first_triangle = self.nodes[node_idx].first_triangle() as usize;
        let triangle_count = self.nodes[node_idx].triangle_count() as usize;

        // if leaf has only one triangle, leave as leaf
        if triangle_count <= 1 { return }

        // choose axis to split on
        let bounds = self.fetch_node(node_idx).bounds();
        let size = bounds.max - bounds.min;
        let mut axis = 0;
        if size.y > size.x {
            axis = 1
        };
        if size.z > *size.index(axis) {
            axis = 2
        };
        let split_pos = *bounds.min.index(axis) + *size.index(axis) * 0.5;

        // sort triangles along axis
        let middle = self.split_triangles_along_plane(
            first_triangle,
            first_triangle + triangle_count - 1,
            axis, split_pos,
        );

        // if split node would contain all or no triangles, dont split
        let left_count = middle - first_triangle;
        if left_count == 0 || left_count == triangle_count {
            return;
        }

        self.total_sah += self.evaluate_sah(node_idx, axis, split_pos);

        let right_child = node_count;
        let left_child = node_count + 1;

        // create and split child nodes
        let first_tri = first_triangle;
        let tri_count = triangle_count;
        self.fetch_node_mut(node_idx).convert_to_node(right_child as u32, left_child as u32);

        self.create_leaf_node(first_tri, left_count);
        self.create_leaf_node(middle, tri_count - left_count);

        self.split_leaf_node(right_child);
        self.split_leaf_node(left_child);
    }
}
