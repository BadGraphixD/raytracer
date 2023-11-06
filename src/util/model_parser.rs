use crate::util::error::ModelParseError;
use cgmath::{Vector2, Vector3};
use crate::raytracing::types::IndexBundle;
use crate::rendering::model::{Model, ModelBuilder};

pub struct ModelParser {}

impl ModelParser {
    pub fn parse(data: String) -> Result<Model, ModelParseError> {
        let mut model_builder = ModelBuilder::new();

        data.split('\n').for_each(|str| {
            if str.starts_with("mtllib ") {
                let value = Self::parse_string_line(str).expect("No string value in line");
                model_builder.add_material_lib(value);
            }
            if str.starts_with("usemtl ") {
                let value = Self::parse_string_line(str).expect("No string value in line");
                model_builder.add_material(value);
            }
            if str.starts_with("f ") {
                let values = Self::parse_index_line(str).expect("Too many/few indices in line");
                values.into_iter().for_each(|(i0, i1, i2)| {
                    model_builder.add_indices(i0, i1, i2);
                });
            }
            if str.starts_with("v ") {
                let values = Self::parse_line(str, 3).expect("Too many/few vertices in line");
                model_builder.add_position(Vector3::new(values[0], values[1], values[2]));
            }
            if str.starts_with("vt ") {
                let values = Self::parse_line(str, 2).expect("Too many/few uvs in line");
                model_builder.add_tex_coord(Vector2::new(values[0], values[1]));
            }
            if str.starts_with("vn ") {
                let values = Self::parse_line(str, 3).expect("Too many/few normals in line");
                model_builder.add_normal(Vector3::new(values[0], values[1], values[2]));
            }
        });

        Ok(model_builder.build())
    }

    fn parse_string_line(str: &str) -> Result<String, ()> {
        let values: Vec<&str> = str.split(' ').filter(|s| !s.is_empty()).collect();
        if values.len() != 2 { Err(()) }
        else { Ok(values[1].to_owned()) }
    }

    fn parse_line(str: &str, len: usize) -> Result<Vec<f32>, ()> {
        let values: Vec<&str> = str.split(' ').filter(|s| !s.is_empty()).collect();
        if values.len() != len + 1 { Err(()) }
        else { Ok((1..=len).into_iter().map(|i| values[i].trim().parse::<f32>().unwrap()).collect()) }
    }

    fn parse_index_line(str: &str) -> Result<Vec<(IndexBundle, IndexBundle, IndexBundle)>, ()> {
        let indices: Vec<Vec<&str>> = str.split(' ').filter(|s| !s.is_empty())
            .map(|s| s.split(['/', '\\']).collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();
        if indices.len() < 4 { Err(()) }
        else {
            let num_tris = indices.len() - 3;
            let bundles: Vec<IndexBundle> = indices.iter().skip(1).map(IndexBundle::new).collect();
            Ok((0..num_tris).into_iter().map(|i| (bundles[0].clone(), bundles[i + 1].clone(), bundles[i + 2].clone())).collect())
        }
    }
}
