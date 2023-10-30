use std::io::Read;
use std::path::{Path, PathBuf};
use std::{env, ffi, fs};
use crate::util::error::ResourceError;

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

    pub fn load_cstring(&self, resource_name: &str) -> Result<ffi::CString, ResourceError> {
        let mut file = fs::File::open(Self::resource_name_to_path(&self.root_path, resource_name))?;

        let mut buffer: Vec<u8> = Vec::with_capacity(file.metadata()?.len() as usize + 1);
        file.read_to_end(&mut buffer)?;

        if buffer.iter().find(|i| **i == 0).is_some() {
            return Err(ResourceError::FileContainsNil(resource_name.to_owned()));
        }

        return Ok(unsafe { ffi::CString::from_vec_unchecked(buffer) });
    }
}
