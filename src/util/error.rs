use std::num::ParseIntError;
use image::ImageError;

#[derive(Debug)]
pub enum WindowError {
    GlfwInitError,
    CreateWindowError,
}

#[derive(Debug)]
pub enum ResourceLoadError {
    FileContainsNil,
    ImageError { e: ImageError },
    Io { e: std::io::Error },
}

#[derive(Debug)]
pub enum ResourceParseError {
    ParseIntError { err: ParseIntError, line: String },
    InvalidLineArgCount { count: usize, line: String },
    NoMaterialNamed,
}

#[derive(Debug)]
pub enum ResourceError {
    FailedToGetExePath,
    ResourceLoadError { e: ResourceLoadError, file_name: String },
    ResourceParseError { e: ResourceParseError, line: u32, file_name: String },
    ShaderError { e: ShaderError, file_name: String },
    DuplicateMaterial { name: String, file_name: String },
    MaterialNotLoaded { name: String },
    ResourceNotLoaded(String),
}

impl ResourceError {
    pub fn load_err(e: ResourceLoadError, file_name: &str) -> Self {
        Self::ResourceLoadError { e, file_name: file_name.to_owned() }
    }

    pub fn parse_err(e: ResourceParseError, line: u32, file_name: &str) -> Self {
        Self::ResourceParseError { e, line, file_name: file_name.to_owned() }
    }

    pub fn shader_err(e: ShaderError, file_name: &str) -> Self {
        Self::ShaderError { e, file_name: file_name.to_owned() }
    }
}

#[derive(Debug)]
pub enum ShaderError {
    InvalidFileExtension(String),
    CompileError(String),
    LinkError(String),
}

#[derive(Debug)]
pub enum FramebufferError {
    Error(u32),
}
