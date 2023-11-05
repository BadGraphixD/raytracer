use crate::util::error::ResourceError;
use std::path::{Path, PathBuf};
use std::{env, fs};
use image::DynamicImage;

pub struct Resource {
    root_path: PathBuf,
}

impl Resource {
    pub fn from_relative_exe_path(path: &str) -> Result<Self, ResourceError> {
        let exe_file_name = env::current_exe().map_err(|_| ResourceError::FailedToGetExePath)?;
        let exe_path = exe_file_name
            .parent()
            .ok_or(ResourceError::FailedToGetExePath)?;
        return Ok(Self {
            root_path: exe_path.join(Path::new(path)),
        });
    }

    fn resource_name_to_path(root_dir: &Path, location: &str) -> PathBuf {
        let mut path: PathBuf = root_dir.into();

        for part in location.split("/") {
            path = path.join(part);
        }

        return path;
    }

    pub fn read_file(&self, resource_name: &str) -> Result<String, ResourceError> {
        fs::read_to_string(Self::resource_name_to_path(&self.root_path, resource_name))
            .map_err(|e| ResourceError::Io(e))
    }

    pub fn read_image_file(&self, resource_name: &str) -> Result<DynamicImage, ResourceError> {
        Ok(image::open(Self::resource_name_to_path(&self.root_path, resource_name))?)
    }
}
