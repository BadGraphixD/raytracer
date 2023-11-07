use std::ops::RangeBounds;
use crate::util::error::ModelParseError;
use cgmath::{Vector2, Vector3};
use crate::raytracing::types::IndexBundle;
use crate::rendering::model::{Model, ModelBuilder};

pub struct ModelParser {}

impl ModelParser {
    pub fn parse(data: String) -> Result<Model, (ModelParseError, u32)> {
        let mut model_builder = ModelBuilder::new();

        data.split('\n').zip(1..).map(|(str, i)| {
            if str.starts_with("mtllib ") {
                let value = Self::parse_string_line(str).map_err(|e| (e, i))?;
                model_builder.add_material_lib(value);
            }
            if str.starts_with("usemtl ") {
                let value = Self::parse_string_line(str).map_err(|e| (e, i))?;
                model_builder.add_material(value);
            }
            if str.starts_with("f ") {
                let values = Self::parse_index_line(str).map_err(|e| (e, i))?;
                values.into_iter().for_each(|(i0, i1, i2)| {
                    model_builder.add_indices(i0, i1, i2);
                });
            }
            if str.starts_with("v ") {
                let values = Self::parse_line(str, 3..=3).map_err(|e| (e, i))?;
                model_builder.add_position(Vector3::new(values[0], values[1], values[2]));
            }
            if str.starts_with("vt ") {
                let values = Self::parse_line(str, 2..=3).map_err(|e| (e, i))?;
                model_builder.add_tex_coord(Vector2::new(values[0], values[1]));
            }
            if str.starts_with("vn ") {
                let values = Self::parse_line(str, 3..=3).map_err(|e| (e, i))?;
                model_builder.add_normal(Vector3::new(values[0], values[1], values[2]));
            }
            Ok(())
        }).collect::<Result<Vec<_>, _>>()?;

        Ok(model_builder.build())
    }

    fn parse_string_line(str: &str) -> Result<String, ModelParseError> {
        let values: Vec<&str> = str.split(' ').filter(|s| !s.is_empty()).collect();
        let l = values.len();
        if l != 2 { Err(ModelParseError::InvalidStringLineArgCount(l, str.to_owned())) }
        else { Ok(values[1].to_owned()) }
    }

    fn parse_line<R: RangeBounds<usize>>(str: &str, len: R) -> Result<Vec<f32>, ModelParseError> {
        let values: Vec<&str> = str.split(' ').filter(|s| !s.is_empty()).collect();
        let l = values.len();
        if !len.contains(&(l - 1)) { Err(ModelParseError::InvalidLineArgCount(l, str.to_owned())) }
        else { Ok(values.into_iter().skip(1).map(|str| str.trim().parse::<f32>().unwrap()).collect()) }
    }

    fn parse_index_line(str: &str) -> Result<Vec<(IndexBundle, IndexBundle, IndexBundle)>, ModelParseError> {
        let indices: Vec<Vec<&str>> = str.split(' ').filter(|s| !s.is_empty())
            .map(|s| s.split(['/', '\\']).collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();
        let len = indices.len();
        if len < 4 { Err(ModelParseError::InvalidIndexLineArgCount(len, str.to_owned())) }
        else {
            let num_tris = indices.len() - 3;
            let bundles: Vec<IndexBundle> = indices.iter().skip(1).map(IndexBundle::new).collect::<Result<Vec<_>, _>>()?;
            Ok((0..num_tris).into_iter().map(|i| (bundles[0].clone(), bundles[i + 1].clone(), bundles[i + 2].clone())).collect())
        }
    }
}
