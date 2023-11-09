use std::ops::Index;
use crate::raytracing::types::{BVHNode, Triangle, AABB, AABBBuilder, Bin};
use cgmath::Vector3;
use crate::rendering::model::Model;

struct BVHTriangle {
    p0: usize,
    p1: usize,
    p2: usize,
    mat_idx: u32,
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
            mat_idx: triangle.mat_idx,
            centroid: [ centroid.x, centroid.y, centroid.z ]
        }
    }
    fn to_tri(&self) -> Triangle {
        Triangle {
            p0: self.p0 as u32,
            p1: self.p1 as u32,
            p2: self.p2 as u32,
            mat_idx: self.mat_idx,
        }
    }
}

pub struct BVH {
    nodes: Vec<BVHNode>,
}

impl BVH {
    pub fn new(nodes: Vec<BVHNode>) -> Self {
        Self { nodes }
    }

    pub fn data(&self) -> &Vec<BVHNode> {
        &self.nodes
    }
}

pub struct BVHBuilder<'a> {
    model: &'a mut Model,
    triangles: Vec<BVHTriangle>,
    nodes: Vec<BVHNode>,
}

impl<'a> BVHBuilder<'a> {
    pub fn new(model: &'a mut Model) -> Self {
        Self {
            triangles: model.triangles().iter().map(|tri| BVHTriangle::new(tri, model.positions())).collect(),
            nodes: Vec::with_capacity(model.triangles().len() * 2),
            model,
        }
    }

    pub fn build(mut self) -> BVH {
        self.create_leaf_node_from_triangles(0, self.triangles.len());
        // push in dummy to make subsequent node-pairs reside in the same cache line
        self.nodes.push(BVHNode::new_dummy());
        self.split_leaf_node_sah(0);
        self.model.set_triangles(self.triangles.iter().map(BVHTriangle::to_tri).collect());
        BVH::new(self.nodes)
    }

    #[inline]
    fn fetch_position(&self, index: usize) -> &Vector3<f32> {
        &self.model.positions()[index]
    }

    #[inline]
    fn fetch_triangle(&self, index: usize) -> &BVHTriangle {
        &self.triangles[index]
    }

    #[inline]
    fn fetch_node(&self, index: usize) -> &BVHNode {
        &self.nodes[index]
    }

    #[inline]
    fn swap_triangles(&mut self, first: usize, second: usize) {
        self.triangles.swap(first, second);
    }

    fn aabb_from_triangles(&self, first: usize, count: usize) -> AABB {
        let mut aabb_builder = AABBBuilder::new();
        for i in first..(first + count) {
            let tri = self.fetch_triangle(i);
            aabb_builder.include(self.fetch_position(tri.p0));
            aabb_builder.include(self.fetch_position(tri.p1));
            aabb_builder.include(self.fetch_position(tri.p2));
        }
        aabb_builder.build()
    }

    fn aabb_from_centroids(&self, first: usize, count: usize) -> AABB {
        let mut aabb_builder = AABBBuilder::new();
        for i in first..(first + count) {
            let tri = self.fetch_triangle(i);
            aabb_builder.include(&Vector3::from(tri.centroid));
        }
        aabb_builder.build()
    }

    fn create_leaf_node_from_triangles(&mut self, first: usize, count: usize) {
        self.nodes.push(BVHNode::new_leaf(
            self.aabb_from_triangles(first, count),
            first as u32,
            count as u32
        ));
    }

    #[inline]
    fn convert_leaf_to_node(&mut self, node_idx: usize, right_node: usize, left_node: usize) {
        self.nodes[node_idx].convert_to_node(right_node as u32, left_node as u32);
    }

    fn split_triangles_along_plane(&mut self, mut first: usize, count: usize, axis: usize, position: f32) -> usize {
        let mut last = first + count;
        while first < last {
            if self.fetch_triangle(first).centroid[axis] < position {
                first += 1;
            } else {
                self.swap_triangles(first, last - 1);
                last -= 1;
            }
        }
        first
    }

    // third version, uses a binning system to avoid recalculating sah for every split individually
    // construction time: fast O(N)
    // traverse time: fast
    fn find_best_split_binned(&self, _node_idx: usize, first_tri: usize, tri_count: usize) -> (usize, f32, f32) {
        const BINS: usize = 50;
        let mut best_axis = 0;
        let mut best_split_pos = 0.0;
        let mut lowest_cost = 1e30;
        let bounds = self.aabb_from_centroids(first_tri, tri_count);
        for axis in 0..3 {
            let min = *bounds.min.index(axis);
            let max = *bounds.max.index(axis);
            if min == max { continue }

            let mut bins = [Bin::new(); BINS];
            let factor = BINS as f32 / (max - min);
            for i in first_tri..(first_tri + tri_count) {
                let tri = self.fetch_triangle(i);
                let bin_idx = usize::min(BINS - 1, ((tri.centroid[axis] - min) * factor) as usize);
                bins[bin_idx].tri_count += 1;
                bins[bin_idx].bounds.include(self.fetch_position(tri.p0));
                bins[bin_idx].bounds.include(self.fetch_position(tri.p1));
                bins[bin_idx].bounds.include(self.fetch_position(tri.p2));
            }

            let mut left_area = [0.0f32; BINS - 1];
            let mut right_area = [0.0f32; BINS - 1];
            let mut left_count = [0u32; BINS - 1];
            let mut right_count = [0u32; BINS - 1];
            let mut left_box = AABBBuilder::new();
            let mut right_box = AABBBuilder::new();
            let mut left_sum = 0;
            let mut right_sum = 0;

            for i in 0..(BINS - 1) {
                left_sum += bins[i].tri_count;
                left_count[i] = left_sum;
                left_box.include_other(&bins[i].bounds);
                left_area[i] = left_box.area();

                right_sum += bins[BINS - 1 - i].tri_count;
                right_count[BINS - 2 - i] = right_sum;
                right_box.include_other(&bins[BINS - 1 - i].bounds);
                right_area[BINS - 2 - i] = right_box.area();
            }

            let step_size = (max - min) / BINS as f32;
            for i in 0..(BINS - 1) {
                let cost = left_count[i] as f32 * left_area[i] + right_count[i] as f32 * right_area[i];
                if cost < lowest_cost {
                    best_axis = axis;
                    best_split_pos = min + step_size * (i + 1) as f32;
                    lowest_cost = cost;
                }
            }
        }
        (best_axis, best_split_pos, lowest_cost)
    }

    // second version, uses surface area heuristics to find best split position
    // construction time & traverse time: depend on sah evaluation algorithm
    fn split_leaf_node_sah(&mut self, node_idx: usize) {
        let node_count = self.nodes.len();
        let first_triangle = self.fetch_node(node_idx).first_triangle() as usize;
        let triangle_count = self.fetch_node(node_idx).triangle_count() as usize;

        // if leaf has only one triangle, leave as leaf
        if triangle_count <= 1 { return }

        // find axis and split pos with minimal cost
        let (axis, split_pos, cost) = self.find_best_split_binned(node_idx, first_triangle, triangle_count);

        // if the minimal cost is higher than the parent cost, don't split
        let parent_cost = triangle_count as f32 * self.fetch_node(node_idx).bounds().area();
        if parent_cost <= cost { return }

        // sort triangles along axis
        let middle = self.split_triangles_along_plane(
            first_triangle, triangle_count,
            axis, split_pos,
        );

        // if one node would contain all or no triangles, don't split
        let left_count = middle - first_triangle;
        let right_count = triangle_count - left_count;
        if left_count == 0 || left_count == triangle_count { return }

        // create and split child nodes
        self.convert_leaf_to_node(node_idx, node_count, node_count + 1);

        self.create_leaf_node_from_triangles(first_triangle, left_count);
        self.create_leaf_node_from_triangles(middle, right_count);

        self.split_leaf_node_sah(node_count);
        self.split_leaf_node_sah(node_count + 1);
    }
}
