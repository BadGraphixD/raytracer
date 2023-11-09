use std::collections::{HashMap, HashSet};
use crate::gl_wrapper::shader::Shader;
use crate::gl_wrapper::texture::Texture;
use crate::gl_wrapper::types::{ShaderType, TextureFilter, TextureFormat};
use crate::rendering::material::Material;
use crate::rendering::model::Model;
use crate::resource::resource::Resource;
use crate::resource::resource_parser::ResourceParser;
use crate::util::error::ResourceError;

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
            shaders: HashMap::new(),

            model_res: Resource::new_rel_to_exe(model_res_path)?,
            texture_res: Resource::new_rel_to_exe(texture_res_path)?,
            shader_res: Resource::new_rel_to_exe(shader_res_path)?,
        })
    }

    pub fn load_textures(&mut self, material: &Material) -> Result<(), ResourceError> {
        material.get_texture_names().into_iter().map(|name| {
            if !self.textures.contains_key(&name) {
                let texture = Texture::from_data(
                    TextureFormat::RGBA8, TextureFilter::Linear,
                    &self.texture_res.read_image_file(&name)?.into_rgb8(),
                );
                self.textures.insert(name, texture);
            }
            Ok(())
        }).collect::<Result<Vec<_>, _>>()?;
        Ok(())
    }

    fn handle_material(&mut self, name: String, material: Material, lib_name: &str) -> Result<(), ResourceError> {
        if self.materials.contains_key(&name) {
            Err(ResourceError::DuplicateMaterial { name, file_name: lib_name.to_owned() })
        } else {
            self.materials.insert(name, material);
            Ok(())
        }
    }

    fn load_mat_lib(&mut self, lib_name: &str) -> Result<(), ResourceError> {
        self.loaded_material_libs.insert(lib_name.to_owned());
        let data = self.model_res.read_file(lib_name)?;
        ResourceParser::parse_material_lib(data, lib_name)?
            .into_iter().map(|(mat_name, mat)| self.handle_material(mat_name, mat, lib_name))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(())
    }

    fn load_model_materials(&mut self, model: &Model) -> Result<(), ResourceError> {
        model.get_material_libs().iter().map(|lib| self.load_mat_lib(lib)).collect::<Result<_, _>>()
    }

    fn load_shader(&mut self, name: &str, r#type: ShaderType) -> Result<Shader, ResourceError> {
        Shader::new(r#type, self.shader_res.read_file(name)?)
            .map_err(|e| ResourceError::shader_err(e, name))
    }

    fn load_model(&mut self, name: &str) -> Result<Model, ResourceError> {
        let model = ResourceParser::parse_model(self.model_res.read_file(name)?)
            .map_err(|(e, l)| ResourceError::parse_err(e, l, name))?;
        self.load_model_materials(&model)?;
        Ok(model)
    }

    pub fn get_shader(&mut self, name: &str, r#type: ShaderType) -> Result<&Shader, ResourceError> {
        if let Some(shader) = self.shaders.get(name) { Ok(shader) }
        else {
            self.shaders.insert(name.to_owned(), self.load_shader(name, r#type)?);
            Ok(self.shaders.get(name).unwrap())
        }
    }

    pub fn get_model(&mut self, name: &str) -> Result<&Model, ResourceError> {
        self.get_model_mut(name).map(|m| m as &Model)
    }

    pub fn get_model_mut(&mut self, name: &str) -> Result<&mut Model, ResourceError> {
        if let Some(model) = self.models.get_mut(name) { Ok(model) }
        else {
            self.models.insert(name.to_owned(), self.load_model(name)?);
            Ok(self.models.get_mut(name).unwrap())
        }
    }

    pub fn get_material(&self, name: &str) -> Option<&Material> {
        self.materials.get(name)
    }

    pub fn get_texture(&self, name: &str) -> Option<&Texture> {
        self.textures.get(name)
    }

}