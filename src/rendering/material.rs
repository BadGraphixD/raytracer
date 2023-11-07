use std::collections::{HashMap, HashSet};
use cgmath::{Array, Vector3};
use crate::util::error::{ResourceError, ResourceParseError};
use crate::util::model_parser::ResourceParser;
use crate::util::resource::Resource;

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
}

pub struct MaterialManager {
    loaded_libs: HashSet<String>,
    materials: HashMap<String, Material>,
}

impl MaterialManager {
    pub fn new() -> Self {
        Self { loaded_libs: HashSet::new(), materials: HashMap::new() }
    }

    pub fn load_lib(&mut self, res: &Resource, name: &str) -> Result<(), ResourceError> {
        let data = res.read_file(name)?;
        self.loaded_libs.insert(name.to_owned());
        ResourceParser::parse_material_lib(data)?.into_iter().map(|(name, mat)| {
            if self.materials.contains_key(&name) {
                Err(ResourceParseError::DuplicateMaterialDefinition)
            } else {
                self.materials.insert(name, mat);
                Ok(())
            }
        }).collect::<Result<Vec<_>, _>>()?;
        Ok(())
    }

    pub fn load_libs(&mut self, res: &Resource, names: &Vec<String>) -> Result<(), ResourceError> {
        names.iter().map(|lib| self.load_lib(res, lib)).collect::<Result<Vec<_>, _>>()?;
        Ok(())
    }

    pub fn get_material(&self, name: &str) -> Option<&Material> {
        self.materials.get(name)
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