use cgmath::{Array, Vector3};
use crate::util::error::ResourceParseError;

#[derive(Debug)]
pub struct Material {
    ambient_color: Vector3<f32>,
    diffuse_color: Vector3<f32>,
    specular_color: Vector3<f32>,
    transmission_color: Vector3<f32>,

    specular_exp: f32,
    transmission: f32,
    optical_density: f32,

    ambient_tex: Option<String>,
    diffuse_tex: Option<String>,
    specular_tex: Option<String>,
    specular_exp_tex: Option<String>,
    //alpha_tex: Option<String>,
    //bump_tex: Option<String>,
    //displacement_tex: Option<String>,
    //decal_tex: Option<String>,
}

impl Material {
    pub fn default() -> Self {
        Self {
            ambient_color: Vector3::from_value(0.3),
            diffuse_color: Vector3::from_value(1.0),
            specular_color: Vector3::from_value(1.0),
            transmission_color: Vector3::from_value(1.0),
            specular_exp: 10.0,
            transmission: 0.0,
            optical_density: 1.0,
            ambient_tex: None,
            diffuse_tex: None,
            specular_tex: None,
            specular_exp_tex: None,
        }
    }

    pub fn get_texture_names(&self) -> Vec<String> {
        let mut names = vec![];
        if let Some(t) = &self.ambient_tex { names.push(t.to_owned()) };
        if let Some(t) = &self.diffuse_tex { names.push(t.to_owned()) };
        if let Some(t) = &self.specular_tex { names.push(t.to_owned()) };
        if let Some(t) = &self.specular_exp_tex { names.push(t.to_owned()) };
        names
    }
}

pub struct MaterialLibBuilder {
    materials: Vec<(String, Material)>,
}

impl MaterialLibBuilder {
    pub fn new() -> Self {
        Self { materials: vec![] }
    }

    pub fn build(self) -> Vec<(String, Material)> {
        self.materials
    }

    fn current(&mut self) -> Result<&mut Material, ResourceParseError> {
        Ok(&mut self.materials.last_mut().ok_or(ResourceParseError::NoMaterialNamed)?.1)
    }

    pub fn add_material(&mut self, name: String) {
        self.materials.push((name, Material::default()))
    }

    pub fn ambient_color(&mut self, col: Vector3<f32>) -> Result<(), ResourceParseError> {
        self.current()?.ambient_color = col; Ok(())
    }

    pub fn diffuse_color(&mut self, col: Vector3<f32>) -> Result<(), ResourceParseError> {
        self.current()?.diffuse_color = col; Ok(())
    }

    pub fn specular_color(&mut self, col: Vector3<f32>) -> Result<(), ResourceParseError> {
        self.current()?.specular_color = col; Ok(())
    }

    pub fn transmission_color(&mut self, col: Vector3<f32>) -> Result<(), ResourceParseError> {
        self.current()?.transmission_color = col; Ok(())
    }

    pub fn specular_exp(&mut self, f: f32) -> Result<(), ResourceParseError> {
        self.current()?.specular_exp = f; Ok(())
    }

    pub fn transmission(&mut self, f: f32) -> Result<(), ResourceParseError> {
        self.current()?.transmission = f; Ok(())
    }

    pub fn optical_density(&mut self, f: f32) -> Result<(), ResourceParseError> {
        self.current()?.optical_density = f; Ok(())
    }

    pub fn ambient_tex(&mut self, name: String) -> Result<(), ResourceParseError> {
        self.current()?.ambient_tex = Some(name); Ok(())
    }

    pub fn diffuse_tex(&mut self, name: String) -> Result<(), ResourceParseError> {
        self.current()?.diffuse_tex = Some(name); Ok(())
    }

    pub fn specular_tex(&mut self, name: String) -> Result<(), ResourceParseError> {
        self.current()?.specular_tex = Some(name); Ok(())
    }

    pub fn specular_exp_tex(&mut self, name: String) -> Result<(), ResourceParseError> {
        self.current()?.specular_exp_tex = Some(name); Ok(())
    }
}