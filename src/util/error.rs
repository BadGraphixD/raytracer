use std::num::ParseIntError;
use image::ImageError;

#[derive(Debug)]
pub enum WindowError {
    GlfwInitError,
    CreateWindowError,
}

#[derive(Debug)]
pub enum ResourceError {
    FailedToGetExePath,
    FileContainsNil(String),
    ImageError(ImageError),
    Io(std::io::Error),
    ResourceParseError(ResourceParseError),
    ResourceParseErrorLine(ResourceParseError, u32),
}

impl From<std::io::Error> for ResourceError {
    fn from(value: std::io::Error) -> Self {
        return ResourceError::Io(value);
    }
}

impl From<ImageError> for ResourceError {
    fn from(value: ImageError) -> Self {
        return ResourceError::ImageError(value);
    }
}

impl From<ResourceParseError> for ResourceError {
    fn from(value: ResourceParseError) -> Self {
        return ResourceError::ResourceParseError(value);
    }
}

impl From<(ResourceParseError, u32)> for ResourceError {
    fn from(value: (ResourceParseError, u32)) -> Self {
        return ResourceError::ResourceParseErrorLine(value.0, value.1);
    }
}

#[derive(Debug)]
pub enum ShaderError {
    CompileError(String),
    LinkError(String),
}

#[derive(Debug)]
pub enum FramebufferError {
    Error(u32),
}

#[derive(Debug)]
pub enum ResourceParseError {
    ParseIntError(ParseIntError, String),
    InvalidIndexLineArgCount(usize, String),
    InvalidStringLineArgCount(usize, String),
    InvalidLineArgCount(usize, String),
    NoMaterialNamed,
    DuplicateMaterialDefinition,
}
