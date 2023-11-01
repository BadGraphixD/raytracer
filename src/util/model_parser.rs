use std::ffi::CString;
use crate::util::error::ModelParseError;

pub struct ModelParser { }

impl ModelParser {
    pub fn parse(data: CString) -> Result<(Vec<f32>, Vec<i32>), ModelParseError> {
        let data = data.to_str().unwrap();

        let mut vertices = vec![];
        let mut indices = vec![];

        data.split("\n").for_each(|str| {
            match str.chars().next() {
                Some('v') => {
                    let values: Vec<&str> = str.split(" ").collect();
                    vertices.push(values[1].parse::<f32>().unwrap());
                    vertices.push(values[2].parse::<f32>().unwrap());
                    vertices.push(values[3].parse::<f32>().unwrap());
                }
                Some('f') => {
                    let values: Vec<&str> = str.split(" ").collect();
                    indices.push(values[1].parse::<i32>().unwrap());
                    indices.push(values[2].parse::<i32>().unwrap());
                    indices.push(values[3].parse::<i32>().unwrap());
                }
                _ => ()
            }
        });

        Ok((vertices, indices))
    }
}