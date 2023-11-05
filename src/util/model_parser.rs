use crate::util::error::ModelParseError;
use cgmath::{Vector2, Vector3};
use crate::rendering::model::{Model, ModelBuilder};

#[derive(Clone)]
struct IndexBundle {
    pos_idx: u32,
    tex_idx: u32,
    nor_idx: u32,
}

impl IndexBundle {
    fn new(input: &Vec<&str>) -> Self {
        let mut ib = Self::new_default();
        if input.len() > 0 && !input[0].is_empty() { ib.pos_idx = Self::parse(input[0]) }
        if input.len() > 1 && !input[1].is_empty() { ib.tex_idx = Self::parse(input[1]) }
        if input.len() > 2 && !input[2].is_empty() { ib.nor_idx = Self::parse(input[2]) }
        ib
    }

    fn new_default() -> Self {
        Self {
            pos_idx: u32::MAX,
            tex_idx: u32::MAX,
            nor_idx: u32::MAX,
        }
    }

    fn parse(str: &str) -> u32 {
        str.trim().parse::<u32>().unwrap() - 1
    }
}

pub struct ModelParser {}

impl ModelParser {
    pub fn parse(data: String) -> Result<Model, ModelParseError> {
        let mut model_builder = ModelBuilder::new();

        data.split('\n').for_each(|str| {
            if str.starts_with("f ") {
                let values = Self::parse_index_line(str).expect("Too many/few indices in line");
                values.iter().for_each(|tri| {
                    model_builder.add_indices(
                        tri[0].pos_idx, tri[1].pos_idx, tri[2].pos_idx,
                        tri[0].tex_idx, tri[1].tex_idx, tri[2].tex_idx,
                        tri[0].nor_idx, tri[1].nor_idx, tri[2].nor_idx,
                    );
                    /*
                    model_builder.add_position_indices(tri[0].pos_idx, tri[1].pos_idx, tri[2].pos_idx);
                    model_builder.add_tex_coord_indices(tri[0].uv_idx, tri[1].uv_idx, tri[2].uv_idx);
                    model_builder.add_normal_indices(tri[0].nor_idx, tri[1].nor_idx, tri[2].nor_idx);
                     */
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

    fn parse_line(str: &str, len: usize) -> Result<Vec<f32>, ()> {
        let values: Vec<&str> = str.split(' ').filter(|s| !s.is_empty()).collect();
        if values.len() != len + 1 { Err(()) }
        else { Ok((1..=len).into_iter().map(|i| values[i].trim().parse::<f32>().unwrap()).collect()) }
    }

    fn parse_index_line(str: &str) -> Result<Vec<[IndexBundle; 3]>, ()> {
        let indices: Vec<Vec<&str>> = str.split(' ').filter(|s| !s.is_empty())
            .map(|s| s.split(['/', '\\']).collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();
        if indices.len() < 4 { Err(()) }
        else {
            let num_tris = indices.len() - 3;
            let bundles: Vec<IndexBundle> = indices.iter().skip(1).map(IndexBundle::new).collect();
            Ok((0..num_tris).into_iter().map(|i| [bundles[0].clone(), bundles[i + 1].clone(), bundles[i + 2].clone()]).collect())
        }
    }
}
