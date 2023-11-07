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
pub enum ModelParseError {
    ParseIntError(ParseIntError, String),
    InvalidIndexLineArgCount(usize, String),
    InvalidStringLineArgCount(usize, String),
    InvalidLineArgCount(usize, String),
}
