use std::path::PathBuf;
use std::{env, fs};
use image::DynamicImage;
use crate::util::error::ResourceLoadError;

pub struct Resource {
    path: PathBuf,
}

impl Resource {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn new_rel_to_exe(path: &str) -> Result<Self, ResourceLoadError> {
        let exe_file = env::current_exe().map_err(|_| ResourceLoadError::FailedToGetExePath)?;
        let exe_path = exe_file.parent().ok_or(ResourceLoadError::FailedToGetExePath)?;
        Ok(Self::new(exe_path.join(path)))
    }

    fn resource_path(&self, name: &str) -> PathBuf {
        self.path.join(PathBuf::from(name))
    }

    pub fn read_file(&self, resource_name: &str) -> Result<String, ResourceLoadError> {
        fs::read_to_string(self.resource_path(resource_name)).map_err(|e| ResourceLoadError::Io { e })
    }

    pub fn read_image_file(&self, resource_name: &str) -> Result<DynamicImage, ResourceLoadError> {
        image::open(self.resource_path(resource_name)).map_err(|e| ResourceLoadError::ImageError { e })
    }
}
