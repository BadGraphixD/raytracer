use crate::gl_wrapper::buffer::{IndexBuffer, ShaderStorageBuffer, VertexBuffer};
use crate::gl_wrapper::framebuffer::Framebuffer;
use crate::gl_wrapper::mesh::MeshBuilder;
use crate::gl_wrapper::shader::{Shader, ShaderProgramBuilder};
use crate::gl_wrapper::texture::Texture;
use crate::gl_wrapper::types::{AttributeType, Primitive, ShaderType, TextureAttachment, TextureFilter, TextureFormat};
use crate::rendering::camera::Camera;
use crate::util::resource::Resource;
use crate::window::window::Window;
use crate::util::model_parser::ModelParser;

pub mod window;
pub mod rendering;
pub mod gl_wrapper;
pub mod util;

fn main() {
    let mut window = Window::new(1000, 800, "Test Hello Window!").expect("Failed to create window!");
    let mut camera = Camera::new_default();

    // load resources
    let shaders = Resource::from_relative_exe_path("res/shaders").unwrap();
    let models = Resource::from_relative_exe_path("res/models").unwrap();

    // load models
    let (model_vertices, model_indices) = ModelParser::parse(models.read_file("dome.obj").unwrap()).unwrap();

    // create shaders
    let default_vert = Shader::new(ShaderType::VertexShader, shaders.read_file("default.vert").unwrap()).unwrap();
    let ray_create_frag = Shader::new(ShaderType::FragmentShader, shaders.read_file("ray_create.frag").unwrap()).unwrap();
    let ray_trace_frag = Shader::new(ShaderType::FragmentShader, shaders.read_file("ray_trace.frag").unwrap()).unwrap();
    let display_frag = Shader::new(ShaderType::FragmentShader, shaders.read_file("display.frag").unwrap()).unwrap();

    let ray_create_program = ShaderProgramBuilder::new()
        .add_shader(&default_vert)
        .add_shader(&ray_create_frag)
        .build().unwrap();

    let ray_trace_program = ShaderProgramBuilder::new()
        .add_shader(&default_vert)
        .add_shader(&ray_trace_frag)
        .build().unwrap();

    let display_program = ShaderProgramBuilder::new()
        .add_shader(&default_vert)
        .add_shader(&display_frag)
        .build().unwrap();

    let ray_create_right_loc = ray_create_program.uniform_location("right");
    let ray_create_up_loc = ray_create_program.uniform_location("up");
    let ray_create_front_loc = ray_create_program.uniform_location("front");

    let ray_trace_dir_tex_loc = ray_trace_program.uniform_location("dirTex");
    let ray_trace_org_loc = ray_trace_program.uniform_location("org");

    let display_program_tex_loc = display_program.uniform_location("display");

    // create frame buffers
    let mut ray_dir_framebuffer = Framebuffer::new();
    let mut ray_dir_texture = Texture::new(window.width(), window.height(), TextureFormat::RGB32F, TextureFilter::Nearest);
    ray_dir_framebuffer.attach_texture(&ray_dir_texture, TextureAttachment::Color(0));
    ray_dir_framebuffer.bind_draw_buffers();

    ray_dir_framebuffer.check_status().expect("Framebuffer incomplete!");

    let mut col_framebuffer = Framebuffer::new();
    let mut col_texture = Texture::new(window.width(), window.height(), TextureFormat::RGB32F, TextureFilter::Nearest);
    col_framebuffer.attach_texture(&col_texture, TextureAttachment::Color(0));
    col_framebuffer.bind_draw_buffers();

    col_framebuffer.check_status().expect("Framebuffer incomplete!");

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

    let vertex_ssbo = ShaderStorageBuffer::new();
    let index_ssbo = ShaderStorageBuffer::new();
    vertex_ssbo.buffer_data(&model_vertices);
    index_ssbo.buffer_data(&model_indices);

    let mesh = MeshBuilder::new()
        .add_buffer(&vbo)
        .add_attribute(2, AttributeType::Float)
        .build(&ibo, Primitive::Triangles);

    while !window.should_close() {
        window.handle_events();

        let dt = 0.5;
        let (cursor_x, cursor_y) = window.input().cursor_pos();
        let (input_x, input_y, input_z) = window.input().movement();

        let yaw = -(2.0 * cursor_x / window.width() as f32 - 1.0) * 8.0;
        let pitch = -(2.0 * cursor_y / window.height() as f32 - 1.0) * 8.0;

        let move_x = (input_x * yaw.sin() + input_z * yaw.cos()) * dt;
        let move_y = input_y * dt;
        let move_z = (input_x * yaw.cos() - input_z * yaw.sin()) * dt;

        camera.set_rotation(yaw, pitch);
        camera.add_position(move_x, move_y, move_z);

        if window.resized() {
            unsafe { gl::Viewport(0, 0, window.width() as i32, window.height() as i32) }
            ray_dir_texture.resize(window.width(), window.height());
            col_texture.resize(window.width(), window.height());
        }

        unsafe {
            gl::ClearColor(1.0, 0.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // create rays
        ray_dir_framebuffer.bind();
        ray_create_program.bind();
        let cvv = camera.generate_view_vectors(&window);
        ray_create_program.set_uniform_3f(ray_create_right_loc, cvv.right);
        ray_create_program.set_uniform_3f(ray_create_up_loc, cvv.up);
        ray_create_program.set_uniform_3f(ray_create_front_loc, cvv.front);
        mesh.draw();

        // ray trace
        col_framebuffer.bind();
        ray_trace_program.bind();
        ray_dir_texture.bind_to_slot(0);
        ray_trace_program.set_uniform_texture(ray_trace_dir_tex_loc, 0);
        ray_trace_program.set_uniform_3f(ray_trace_org_loc, cvv.pos);
        vertex_ssbo.bind_to_slot(0);
        index_ssbo.bind_to_slot(1);
        mesh.draw();

        // render ray data onto screen
        col_framebuffer.unbind();
        display_program.bind();
        col_texture.bind_to_slot(0);
        display_program.set_uniform_texture(display_program_tex_loc, 0);
        mesh.draw();

        window.update();
    }

    window.close();
}