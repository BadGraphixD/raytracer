use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use crate::gl_wrapper::shader::Shader;
use crate::gl_wrapper::texture::Texture;
use crate::gl_wrapper::types::ShaderType;
use crate::rendering::material::Material;
use crate::rendering::model::Model;
use crate::util::error::ResourceError;
use crate::util::error::ResourceError::ResourceParseError;
use crate::util::model_parser::ResourceParser;
use crate::util::resource::Resource;

pub struct ResourceManager {
    loaded_material_libs: HashSet<String>,

    models: HashMap<String, Model>,
    materials: HashMap<String, Material>,
    textures: HashMap<String, Texture>,
    shaders: HashMap<String, Shader>,

    model_res: Resource,
    texture_res: Resource,
    shader_res: Resource,
}

impl ResourceManager {
    pub fn new(model_res_path: &str, texture_res_path: &str, shader_res_path: &str) -> Result<Self, ResourceError> {
        Ok(Self {
            loaded_material_libs: HashSet::new(),

            models: HashMap::new(),
            materials: HashMap::new(),
            textures: HashMap::new(),

            model_res: Resource::new_rel_to_exe(model_res_path)?,
            texture_res: Resource::new_rel_to_exe(texture_res_path)?,
            shader_res: Resource::new_rel_to_exe(shader_res_path)?,
        })
    }

    pub fn load_material_lib(&mut self, name: &str) -> Result<(), ResourceError> {
        let data = self.model_res.read_file(name).map_err(|e| ResourceError::load_err(e, name))?;
        self.loaded_material_libs.insert(name.to_owned());
        ResourceParser::parse_material_lib(data).map_err(|(e, l)| ResourceError::parse_err(e, l, name))?
            .into_iter().map(|(name, mat)| {
            if self.materials.contains_key(&name) {
                Err(ResourceError::DuplicateMaterialDefinition { name })
            } else {
                self.materials.insert(name, mat);
                Ok(())
            }
        }).collect::<Result<Vec<_>, _>>()?;
        Ok(())
    }

    pub fn load_material_libs(&mut self, names: &Vec<String>) -> Result<(), ResourceError> {
        names.iter().map(|lib| self.load_lib(lib)).collect::<Result<Vec<_>, _>>()?;
        Ok(())
    }

    fn load_shader(&mut self, name: &str, r#type: ShaderType) -> Result<Shader, ResourceError> {
        Shader::new(r#type,
             self.shader_res.read_file(name).map_err(|e| ResourceError::load_err(e, name))?
        ).map_err(|e| ResourceError::shader_err(e))
    }

    pub fn get_shader(&mut self, name: &str, r#type: ShaderType) -> Result<&Shader, ResourceError> {
        if let Some(shader) = self.shaders.get(name) {
            Ok(shader)
        } else {
            self.shaders.insert(name.to_owned(), self.load_shader(name, r#type)?);
            Ok(self.shaders.get(name).unwrap())
        }
    }

    pub fn get_material(&self, name: &str) -> Option<&Material> {
        self.materials.get(name)
    }

}