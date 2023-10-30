use crate::gl_wrapper::buffer::{IndexBuffer, VertexBuffer};
use crate::gl_wrapper::framebuffer::Framebuffer;
use crate::gl_wrapper::mesh::MeshBuilder;
use crate::gl_wrapper::shader::{Shader, ShaderProgramBuilder};
use crate::gl_wrapper::texture::Texture;
use crate::gl_wrapper::types::{AttributeType, Primitive, ShaderType, TextureAttachment, TextureFormat};
use crate::util::resource::Resource;
use crate::window::window::Window;

pub mod window;
pub mod rendering;
pub mod gl_wrapper;
pub mod util;

fn main() {
    let mut window = match Window::new(400, 300, "Test Hello Window!") {
        Ok(w) => w,
        Err(_) => panic!()
    };

    // let camera = Camera::new(90.0, 0.1, 1000.0);

    // load resources
    let shaders = Resource::from_relative_exe_path("res/shaders").unwrap();

    // create shaders
    let default_vert = Shader::new(ShaderType::VertexShader, shaders.load_cstring("default.vert").unwrap()).unwrap();
    let ray_create_frag = Shader::new(ShaderType::FragmentShader, shaders.load_cstring("ray_create.frag").unwrap()).unwrap();
    let display_frag = Shader::new(ShaderType::FragmentShader, shaders.load_cstring("display.frag").unwrap()).unwrap();

    let ray_create_program = ShaderProgramBuilder::new()
        .add_shader(&default_vert)
        .add_shader(&ray_create_frag)
        .build().unwrap();

    let display_program = ShaderProgramBuilder::new()
        .add_shader(&default_vert)
        .add_shader(&display_frag)
        .build().unwrap();

    let display_program_tex_loc = display_program.uniform_location("display");

    // create frame buffers
    let ray_framebuffer = Framebuffer::new();
    let ray_dir_texture = Texture::new(window.width(), window.height(), TextureFormat::RGB32F);

    ray_framebuffer.attach_texture(&ray_dir_texture, TextureAttachment::Color(0));

    unsafe {
        let draw_buffers = [ gl::COLOR_ATTACHMENT0 ];
        gl::DrawBuffers(1, &draw_buffers as *const u32);
    }

    ray_framebuffer.check_status().expect("Framebuffer incomplete!");

    // create objects and load into vertex buffer
    let vertices: [f32; 8] = [
        -1.0, -1.0,
        -1.0,  1.0,
         1.0,  1.0,
         1.0, -1.0,
    ];
    let indices: [i32; 6] = [
        0, 2, 1,
        0, 2, 3,
    ];

    let mut vbo = VertexBuffer::new();
    let mut ibo = IndexBuffer::new();

    vbo.buffer_data(&vertices);
    ibo.buffer_data(&indices);

    let mesh = MeshBuilder::new()
        .add_buffer(&vbo)
        .add_attribute(2, AttributeType::Float)
        .build(&ibo, Primitive::Triangles);

    while !window.should_close() {
        window.handle_events();

        if window.resized() {
            unsafe { gl::Viewport(0, 0, window.width() as i32, window.height() as i32) }
            // resize frame buffers
        }

        unsafe {
            gl::ClearColor(1.0, 0.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // render ray data into frame buffer
        ray_framebuffer.bind();
        ray_create_program.bind();
        mesh.draw();

        // render ray data onto screen
        ray_framebuffer.unbind();
        display_program.bind();
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + 0);
            ray_dir_texture.bind();
        }
        display_program.set_uniform_texture(display_program_tex_loc, 0);
        mesh.draw();

        window.update();
    }

    window.close();
}