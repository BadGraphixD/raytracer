use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use crate::gl_wrapper::shader::{Shader, ShaderProgram, ShaderProgramBuilder};
use crate::gl_wrapper::texture::Texture;
use crate::gl_wrapper::types::{ShaderType, TextureFilter, TextureFormat};
use crate::rendering::material::Material;
use crate::rendering::model::Model;
use crate::resource::resource::Resource;
use crate::resource::resource_parser::ResourceParser;
use crate::util::error::ResourceError;

pub struct ResourceManager {
    loaded_material_libs: HashSet<String>,

    models: HashMap<String, Arc<Mutex<Model>>>,
    materials: HashMap<String, Arc<Material>>,
    textures: HashMap<String, Arc<Texture>>,
    shaders: HashMap<String, Arc<Shader>>,
    shader_programs: HashMap<String, Arc<Mutex<ShaderProgram>>>,

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
            shader_programs: HashMap::new(),

            model_res: Resource::new_rel_to_exe(model_res_path)?,
            texture_res: Resource::new_rel_to_exe(texture_res_path)?,
            shader_res: Resource::new_rel_to_exe(shader_res_path)?,
        })
    }

    pub fn load_model(&mut self, name: &str) -> Result<(), ResourceError> {
        let model = ResourceParser::parse_model(self.model_res.read_file(name)?)
            .map_err(|(e, l)| ResourceError::parse_err(e, l, name))?;
        self.load_model_material_libs(&model)?;
        self.models.insert(name.to_owned(), Arc::new(Mutex::new(model)));
        Ok(())
    }

    fn load_model_material_libs(&mut self, model: &Model) -> Result<(), ResourceError> {
        model.get_material_libs().iter().map(|lib| self.load_mat_lib(lib)).collect::<Result<_, _>>()
    }

    fn load_mat_lib(&mut self, lib_name: &str) -> Result<(), ResourceError> {
        self.loaded_material_libs.insert(lib_name.to_owned());
        let data = self.model_res.read_file(lib_name)?;
        ResourceParser::parse_material_lib(data, lib_name)?
            .into_iter().map(|(mat_name, mat)| self.handle_material(mat_name, mat, lib_name))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(())
    }

    fn handle_material(&mut self, name: String, material: Material, lib_name: &str) -> Result<(), ResourceError> {
        if self.materials.contains_key(&name) {
            Err(ResourceError::DuplicateMaterial { name, file_name: lib_name.to_owned() })
        } else {
            self.load_textures(&material)?;
            self.materials.insert(name, Arc::new(material));
            Ok(())
        }
    }

    fn load_textures(&mut self, material: &Material) -> Result<(), ResourceError> {
        material.get_texture_names().into_iter().map(|name| {
            if !self.textures.contains_key(&name) {
                self.load_texture(&name)?;
            }
            Ok(())
        }).collect::<Result<Vec<_>, _>>()?;
        Ok(())
    }

    fn load_texture(&mut self, name: &str) -> Result<(), ResourceError> {
        let texture = Texture::from_data(
            TextureFormat::RGBA8, TextureFilter::Linear,
            &self.texture_res.read_image_file(&name)?.into_rgb8(),
        );
        self.textures.insert(name.to_owned(), Arc::new(texture));
        Ok(())
    }

    pub fn create_shader_program(&mut self, name: &str, vert: &str, frag: &str) -> Result<Arc<Mutex<ShaderProgram>>, ResourceError> {
        let vert = self.get_shader(vert)?;
        let frag = self.get_shader(frag)?;
        self.shader_programs.insert(
            name.to_owned(),
            Arc::new(Mutex::new(ShaderProgramBuilder::new()
                .add_shader(vert.clone())
                .add_shader(frag.clone())
                .build().map_err(|e| ResourceError::shader_err(e, name))?
        )));
        Ok(self.shader_programs.get(name).unwrap().clone())
    }

    fn load_shader(&mut self, name: &str) -> Result<(), ResourceError> {
        let r#type = ShaderType::from_file_name(name).map_err(|e| ResourceError::shader_err(e, name))?;
        self.shaders.insert(name.to_owned(), Arc::new(Shader::new(r#type, self.shader_res.read_file(name)?)
            .map_err(|e| ResourceError::shader_err(e, name))?));
        Ok(())
    }

    pub fn get_model(&mut self, name: &str) -> Result<Arc<Mutex<Model>>, ResourceError> {
        if let Some(model) = self.models.get(name) { Ok(model.clone()) }
        else {
            self.load_model(name)?;
            Ok(self.models.get(name).unwrap().clone())
        }
    }

    pub fn get_material(&self, name: &str) -> Result<Arc<Material>, ResourceError> {
        self.materials.get(name).cloned().ok_or(ResourceError::ResourceNotLoaded(name.to_owned()))
    }

    pub fn get_texture(&mut self, name: &str) -> Result<Arc<Texture>, ResourceError> {
        if let Some(texture) = self.textures.get(name) { Ok(texture.clone()) }
        else {
            self.load_texture(name)?;
            Ok(self.textures.get(name).unwrap().clone())
        }
    }

    pub fn get_shader(&mut self, name: &str) -> Result<Arc<Shader>, ResourceError> {
        if let Some(shader) = self.shaders.get(name) { Ok(shader.clone()) }
        else {
            self.load_shader(name)?;
            Ok(self.shaders.get(name).unwrap().clone())
        }
    }

    pub fn get_shader_program(&self, name: &str) -> Option<Arc<Mutex<ShaderProgram>>> {
        self.shader_programs.get(name).cloned()
    }
}