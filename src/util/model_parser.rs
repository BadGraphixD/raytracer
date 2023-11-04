use crate::raytracing::types::Triangle;
use crate::util::error::ModelParseError;
use cgmath::{Vector2, Vector3};

pub struct ModelParser {}

impl ModelParser {
    pub fn parse(data: String) -> Result<(Vec<Vector3<f32>>, Vec<Triangle>), ModelParseError> {
        let mut triangles: Vec<Triangle> = vec![];
        let mut vertices: Vec<Vector3<f32>> = vec![];
        let mut normals: Vec<Vector3<f32>> = vec![];
        let mut uvs: Vec<Vector2<f32>> = vec![];

        data.split('\n').for_each(|str| {
            if str.starts_with("f ") {
                let values = Self::parse_index_line(str).expect("Too many/few indices in line");
                triangles.push(Triangle::new(values[0], values[1], values[2]));
            }
            if str.starts_with("v ") {
                let values = Self::parse_line(str, 3).expect("Too many/few vertices in line");
                vertices.push(Vector3::new(values[0], values[1], values[2]));
            }
            if str.starts_with("vn ") {
                let values = Self::parse_line(str, 3).expect("Too many/few normals in line");
                normals.push(Vector3::new(values[0], values[1], values[2]));
            }
            if str.starts_with("vt ") {
                let values = Self::parse_line(str, 2).expect("Too many/few uvs in line");
                uvs.push(Vector2::new(values[0], values[1]));
            }
        });

        Ok((vertices, triangles))
    }

    fn parse_line(str: &str, len: usize) -> Result<Vec<f32>, ()> {
        let values: Vec<&str> = str.split(' ').filter(|s| !s.is_empty()).collect();
        if values.len() != len + 1 { Err(()) }
        else { Ok((1..=len).into_iter().map(|i| values[i].trim().parse::<f32>().unwrap()).collect()) }
    }

    fn parse_index_line(str: &str) -> Result<Vec<u32>, ()> {
        let indices: Vec<Vec<&str>> = str.split(' ').filter(|s| !s.is_empty())
            .map(|s| s.split(['/', '\\']).collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();
        // todo: system that splits large polygons into triangles
        // todo: system that can distinguish between vertex-, normal- and uv-indices
        if indices.len() != 3 + 1 { Err(()) }
        else { Ok((1..=3).into_iter().map(|i| indices[i][0].trim().parse::<u32>().unwrap() - 1).collect()) }
    }
}
