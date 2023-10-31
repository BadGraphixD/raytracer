use crate::gl_wrapper::buffer::{IndexBuffer, ShaderStorageBuffer, VertexBuffer};
use crate::gl_wrapper::framebuffer::Framebuffer;
use crate::gl_wrapper::mesh::MeshBuilder;
use crate::gl_wrapper::shader::{Shader, ShaderProgramBuilder};
use crate::gl_wrapper::texture::Texture;
use crate::gl_wrapper::types::{AttributeType, Primitive, ShaderType, TextureAttachment, TextureFilter, TextureFormat};
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
    let mut ray_dir_texture = Texture::new(window.width(), window.height(), TextureFormat::RGB32F, TextureFilter::Nearest);

    ray_framebuffer.attach_texture(&ray_dir_texture, TextureAttachment::Color(0));

    unsafe {
        let draw_buffers = [ gl::COLOR_ATTACHMENT0 ];
        gl::DrawBuffers(1, &draw_buffers as *const u32);
    }

    ray_framebuffer.check_status().expect("Framebuffer incomplete!");

    // create buffer for drawing square
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

    // create buffer for storing triangle data
    let triangleVertices: [f32; 24] = [
        // front
        -1.0, -1.0,  1.0,
        1.0, -1.0,  1.0,
        1.0,  1.0,  1.0,
        -1.0,  1.0,  1.0,
        // back
        -1.0, -1.0, -1.0,
        1.0, -1.0, -1.0,
        1.0,  1.0, -1.0,
        -1.0,  1.0, -1.0
    ];

    let triangleIndeces: [i32; 36] = [
        // front
        0, 1, 2,
        2, 3, 0,
        // right
        1, 5, 6,
        6, 2, 1,
        // back
        7, 6, 5,
        5, 4, 7,
        // left
        4, 0, 3,
        3, 7, 4,
        // bottom
        4, 5, 1,
        1, 0, 4,
        // top
        3, 2, 6,
        6, 7, 3,
    ];

    let vertexSSBO = ShaderStorageBuffer::new();
    let indexSSBO = ShaderStorageBuffer::new();

    vertexSSBO.buffer_data(&triangleVertices);
    indexSSBO.buffer_data(&triangleIndeces);

    let mesh = MeshBuilder::new()
        .add_buffer(&vbo)
        .add_attribute(2, AttributeType::Float)
        .build(&ibo, Primitive::Triangles);

    while !window.should_close() {
        window.handle_events();

        if window.resized() {
            unsafe { gl::Viewport(0, 0, window.width() as i32, window.height() as i32) }
            ray_dir_texture.resize(window.width(), window.height());
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
        ray_dir_texture.bind_to_slot(0);
        display_program.set_uniform_texture(display_program_tex_loc, 0);
        mesh.draw();

        window.update();
    }

    window.close();
}