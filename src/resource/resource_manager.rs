use std::collections::{HashMap, HashSet};
use crate::gl_wrapper::shader::{Shader, ShaderProgram, ShaderProgramBuilder};
use crate::gl_wrapper::texture::Texture;
use crate::gl_wrapper::types::{ShaderType, TextureFilter, TextureFormat};
use crate::raytracing::bvh::BVH;
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
    shader_programs: HashMap<String, ShaderProgram>,

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
        self.models.insert(name.to_owned(), model);
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
            self.materials.insert(name, material);
            Ok(())
        }
    }

    fn load_textures(&mut self, material: &Material) -> Result<(), ResourceError> {
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

    pub fn load_shader_program(&mut self, names: (&str, &str)) -> Result<(), ResourceError> {
        if !self.shaders.contains_key(names.0) { self.load_shader(names.0, ShaderType::VertexShader)? }
        if !self.shaders.contains_key(names.1) { self.load_shader(names.1, ShaderType::FragmentShader)? }
        let program_name = format!("{}{}", names.0, names.1);
        self.shader_programs.insert(
            program_name.clone(),
            ShaderProgramBuilder::new()
                .add_shader(self.shaders.get(names.0).unwrap())
                .add_shader(self.shaders.get(names.1).unwrap())
                .build().map_err(|e| ResourceError::shader_err(e, &program_name))?
        );
        Ok(())
    }

    fn load_shader(&mut self, name: &str, r#type: ShaderType) -> Result<(), ResourceError> {
        self.shaders.insert(name.to_owned(), Shader::new(r#type, self.shader_res.read_file(name)?)
            .map_err(|e| ResourceError::shader_err(e, name))?);
        Ok(())
    }

    pub fn get_model(&self, name: &str) -> Option<&Model> {
        self.models.get(name)
    }

    pub fn get_material(&self, name: &str) -> Option<&Material> {
        self.materials.get(name)
    }

    pub fn get_texture(&self, name: &str) -> Option<&Texture> {
        self.textures.get(name)
    }

    pub fn get_shader(&self, name: &str) -> Option<&Shader> {
        self.shaders.get(name)
    }

    pub fn get_shader_program(&self, names: (&str, &str)) -> Option<&ShaderProgram> {
        self.shader_programs.get(&format!("{}{}", names.0, names.1))
    }

    pub fn build_model_bvh(&mut self, name: &str) -> Result<(), ResourceError> {
        self.models.get_mut(name).ok_or(ResourceError::ModelNotLoaded { name: name.to_owned() })?
            .build_bvh();
        Ok(())
    }

    pub fn get_model_bvh(&self, name: &str) -> Option<&BVH> {
        match self.models.get(name) {
            None => None,
            Some(model) => model.get_bvh(),
        }
    }
}