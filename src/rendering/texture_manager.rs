use std::collections::HashMap;
use crate::gl_wrapper::texture::Texture;
use crate::gl_wrapper::types::{TextureFilter, TextureFormat};
use crate::rendering::material::Material;
use crate::util::error::ResourceError;
use crate::util::resource::Resource;

pub struct TextureManager {
    textures: HashMap<String, Texture>,
}

impl TextureManager {
    pub fn new() -> Self {
        Self { textures: HashMap::new() }
    }

    pub fn load_textures(&mut self, res: &Resource, mat: &Material) -> Result<(), ResourceError> {
        let texture_names = mat.get_texture_names();
        texture_names.into_iter().map(|name| {
            if !self.textures.contains_key(&name) {
                let data = res.read_image_file(&name).map_err(|e| ResourceError::load_err(e, &name))?;
                let texture = Texture::from_data(
                    TextureFormat::RGBA8,
                    TextureFilter::Linear,
                    &data.into_rgb8(),
                );
                self.textures.insert(name, texture);
            }
            Ok(())
        }).collect::<Result<Vec<_>, _>>()?;
        Ok(())
    }

    pub fn get_texture(&self, name: &str) -> Option<&Texture> {
        self.textures.get(name)
    }
}