use std::path::PathBuf;
use std::{env, fs};
use image::DynamicImage;
use crate::util::error::{ResourceError, ResourceLoadError};

pub struct Resource {
    path: PathBuf,
}

impl Resource {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn new_rel_to_exe(path: &str) -> Result<Self, ResourceError> {
        let exe_file = env::current_exe().map_err(|_| ResourceError::FailedToGetExePath)?;
        let exe_path = exe_file.parent().ok_or(ResourceError::FailedToGetExePath)?;
        Ok(Self::new(exe_path.join(path)))
    }

    fn resource_path(&self, name: &str) -> PathBuf {
        self.path.join(PathBuf::from(name))
    }

    pub fn read_file(&self, name: &str) -> Result<String, ResourceError> {
        fs::read_to_string(self.resource_path(name))
            .map_err(|e| ResourceError::load_err(ResourceLoadError::Io { e }, name))
    }

    pub fn read_image_file(&self, name: &str) -> Result<DynamicImage, ResourceError> {
        image::open(self.resource_path(name))
            .map_err(|e| ResourceError::load_err(ResourceLoadError::ImageError { e }, name))
    }
}
