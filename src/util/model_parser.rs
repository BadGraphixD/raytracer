use crate::util::error::ModelParseError;

pub struct ModelParser { }

impl ModelParser {
    pub fn parse(data: String) -> Result<(Vec<f32>, Vec<i32>), ModelParseError> {
        let mut vertices: Vec<f32> = vec![];
        let mut indices: Vec<i32> = vec![];

        data.split('\n').for_each(|str| {
            match str.chars().next() {
                Some('v') => {
                    let values: Vec<&str> = str.split(" ").filter(|s| !s.is_empty()).collect();
                    if values.len() != 4 { panic!() } // todo: remove and replace with more sophisticated system
                    vertices.push(values[1].trim().parse::<f32>().unwrap());
                    vertices.push(values[2].trim().parse::<f32>().unwrap());
                    vertices.push(values[3].trim().parse::<f32>().unwrap());
                }
                Some('f') => {
                    let values: Vec<&str> = str.split(" ").filter(|s| !s.is_empty()).collect();
                    if values.len() != 4 { panic!() } // todo: remove and replace with polygon to triangle splitter
                    indices.push(values[1].trim().parse::<i32>().unwrap() - 1);
                    indices.push(values[2].trim().parse::<i32>().unwrap() - 1);
                    indices.push(values[3].trim().parse::<i32>().unwrap() - 1);
                }
                _ => ()
            }
        });

        Ok((vertices, indices))
    }
}