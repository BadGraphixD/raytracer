use cgmath::Vector3;
use crate::raytracing::types::Triangle;
use crate::util::error::ModelParseError;

pub struct ModelParser { }

impl ModelParser {
    pub fn parse(data: String) -> Result<(Vec<Vector3<f32>>, Vec<Triangle>), ModelParseError> {
        let mut vertices: Vec<Vector3<f32>> = vec![];
        let mut triangles: Vec<Triangle> = vec![];

        data.split('\n').for_each(|str| {
            match str.chars().next() {
                Some('v') => {
                    let values: Vec<&str> = str.split(" ").filter(|s| !s.is_empty()).collect();
                    if values.len() != 4 { panic!() } // todo: remove and replace with more sophisticated system
                    vertices.push(Vector3::new(
                        values[1].trim().parse::<f32>().unwrap(),
                        values[2].trim().parse::<f32>().unwrap(),
                        values[3].trim().parse::<f32>().unwrap(),
                    ));
                }
                Some('f') => {
                    let values: Vec<&str> = str.split(" ").filter(|s| !s.is_empty()).collect();
                    if values.len() != 4 { panic!() } // todo: remove and replace with polygon to triangle splitter
                    triangles.push(Triangle::new(
                        values[1].trim().parse::<i32>().unwrap() - 1,
                        values[2].trim().parse::<i32>().unwrap() - 1,
                        values[3].trim().parse::<i32>().unwrap() - 1,
                    ));
                }
                _ => ()
            }
        });

        Ok((vertices, triangles))
    }
}