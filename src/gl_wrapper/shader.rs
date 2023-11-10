use std::collections::HashMap;
use crate::gl_wrapper::types::ShaderType;
use crate::util::error::ShaderError;
use cgmath::Vector3;
use gl::types::GLchar;
use std::ffi::CString;
use std::sync::Arc;

fn create_shader(r#type: u32, source: String) -> Result<u32, String> {
    unsafe {
        let cstr_source = CString::new(source).unwrap();
        let shader = gl::CreateShader(r#type);
        gl::ShaderSource(shader, 1, &cstr_source.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);
        match get_shader_compile_error(shader) {
            Some(e) => Err(e),
            None => Ok(shader),
        }
    }
}

fn get_shader_compile_error(shader: u32) -> Option<String> {
    unsafe {
        let mut compile_status = 0;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut compile_status);

        if compile_status != gl::TRUE as i32 {
            let mut info_log_length = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut info_log_length);
            let error: CString = cstring_with_capacity(info_log_length);
            gl::GetShaderInfoLog(
                shader,
                info_log_length,
                std::ptr::null_mut(),
                error.as_ptr() as *mut GLchar,
            );

            return Some(error.to_string_lossy().into_owned());
        }
        None
    }
}

fn link_program(program: u32) -> Result<(), String> {
    unsafe {
        gl::LinkProgram(program);
        match get_program_error(program, gl::LINK_STATUS) {
            Some(e) => Err(e),
            None => {
                gl::ValidateProgram(program);
                match get_program_error(program, gl::VALIDATE_STATUS) {
                    Some(e) => Err(e),
                    None => Ok(()),
                }
            }
        }
    }
}

fn get_program_error(program: u32, r#type: u32) -> Option<String> {
    unsafe {
        let mut status = 0;
        gl::GetProgramiv(program, r#type, &mut status);

        if status != gl::TRUE as i32 {
            let mut info_log_length = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut info_log_length);
            let error: CString = cstring_with_capacity(info_log_length);
            gl::GetProgramInfoLog(
                program,
                info_log_length,
                std::ptr::null_mut(),
                error.as_ptr() as *mut GLchar,
            );

            return Some(error.to_string_lossy().into_owned());
        }
        None
    }
}

fn cstring_with_capacity(capacity: i32) -> CString {
    let mut buffer = Vec::with_capacity(capacity as usize + 1);
    buffer.extend([b' '].iter().cycle().take(capacity as usize));
    unsafe { CString::from_vec_unchecked(buffer) }
}

pub struct Shader {
    id: u32,
}

impl Shader {
    pub fn new(r#type: ShaderType, source: String) -> Result<Self, ShaderError> {
        match create_shader(r#type.to_gl_internal(), source) {
            Ok(id) => Ok(Self { id }),
            Err(err) => Err(ShaderError::CompileError(err)),
        }
    }

    pub fn new_vertex(source: String) -> Result<Self, ShaderError> {
        Self::new(ShaderType::VertexShader, source)
    }

    pub fn new_fragment(source: String) -> Result<Self, ShaderError> {
        Self::new(ShaderType::FragmentShader, source)
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.id) }
    }
}

pub struct ShaderProgram {
    id: u32,
    uniform_locations: HashMap<String, i32>,
}

impl ShaderProgram {
    pub fn new() -> Self {
        Self {
            id: unsafe { gl::CreateProgram() },
            uniform_locations: HashMap::new(),
        }
    }

    pub fn bind(&self) {
        unsafe { gl::UseProgram(self.id) }
    }
    pub fn unbind(&self) {
        unsafe { gl::UseProgram(0) }
    }

    pub fn set_uniform_texture(&mut self, name: &str, texture: i32) {
        unsafe { gl::Uniform1i(self.uniform_location(name), texture) }
    }

    pub fn set_uniform_texture_array(&mut self, name: &str, v: Vec<i32>) {
        unsafe { gl::Uniform1iv(self.uniform_location(name), v.len() as i32, v.as_ptr()) }
    }

    pub fn set_uniform_3f(&mut self, name: &str, v: Vector3<f32>) {
        unsafe { gl::Uniform3f(self.uniform_location(name), v[0], v[1], v[2]) }
    }

    pub fn set_uniform_1i(&mut self, name: &str, i: i32) {
        unsafe { gl::Uniform1i(self.uniform_location(name), i) }
    }

    pub fn set_uniform_1b(&mut self, name: &str, b: bool) {
        unsafe { gl::Uniform1i(self.uniform_location(name), b as i32) }
    }

    pub fn uniform_location(&mut self, name: &str) -> i32 {
        if let Some(loc) = self.uniform_locations.get(name) { *loc }
        else {
            let name_cstring = CString::new(name).unwrap();
            let loc = unsafe { gl::GetUniformLocation(self.id, name_cstring.as_ptr()) };
            self.uniform_locations.insert(name.to_owned(), loc);
            loc
        }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.id) }
    }
}

pub struct ShaderProgramBuilder {
    shaders: Vec<Arc<Shader>>,
}

impl ShaderProgramBuilder {
    pub fn new() -> Self {
        Self { shaders: vec![] }
    }

    pub fn add_shader(mut self, shader: Arc<Shader>) -> Self {
        self.shaders.push(shader);
        self
    }

    pub fn build(self) -> Result<ShaderProgram, ShaderError> {
        let program = ShaderProgram::new();
        self.shaders.iter()
            .for_each(|shader| unsafe { gl::AttachShader(program.id, shader.id) });
        match link_program(program.id) {
            Ok(_) => Ok(program),
            Err(err) => Err(ShaderError::LinkError(err)),
        }
    }
}
