use std::ffi::CString;
use std::panic::panic_any;
use crate::util::error::ModelParseError;

pub struct ModelParser { }

impl ModelParser {
    pub fn parse(data: CString) -> Result<(Vec<f32>, Vec<i32>), ModelParseError> {
        let data = data.to_str().unwrap();

        let mut vertices = vec![];
        let mut indices = vec![];

        let mut counter = 0;

        data.split("\n").for_each(|str| {
            counter += 1;
            match str.chars().next() {
                Some('v') => {
                    let values: Vec<&str> = str.split(" ").filter(|s| !s.is_empty()).collect();
                    if values.len() != 4 { panic!("{}", counter) }
                    vertices.push(values[1].parse::<f32>().expect(values[1]));
                    vertices.push(values[2].parse::<f32>().expect(values[2]));
                    vertices.push(values[3].parse::<f32>().expect(&counter.to_string()));
                }
                Some('f') => {
                    let values: Vec<&str> = str.split(" ").filter(|s| !s.is_empty()).collect();
                    if values.len() != 4 { panic!("{}", counter) }
                    indices.push(values[1].parse::<i32>().expect(values[1]));
                    indices.push(values[2].parse::<i32>().expect(values[2]));
                    indices.push(values[3].parse::<i32>().expect(values[3]));
                }
                _ => ()
            }
        });

        Ok((vertices, indices))
    }
}