use std::time::SystemTime;
use crate::gl_wrapper::buffer::ShaderStorageBuffer;
use crate::gl_wrapper::framebuffer::Framebuffer;
use crate::gl_wrapper::geometry_set::GeometrySetBuilder;
use crate::gl_wrapper::shader::{Shader, ShaderProgramBuilder};
use crate::gl_wrapper::texture::Texture;
use crate::gl_wrapper::types::{ShaderType, TextureAttachment, TextureFilter, TextureFormat};
use crate::raytracing::bvh::BVHBuilder;
use crate::rendering::camera::Camera;
use crate::util::camera_controller::CameraController;
use crate::util::model_parser::ModelParser;
use crate::util::resource::Resource;
use crate::window::window::Window;

pub mod gl_wrapper;
pub mod raytracing;
pub mod rendering;
pub mod util;
pub mod window;

fn main() {
    // create window
    let mut window = Window::new(1000, 800, "Raytracing :)").expect("Failed to create window!");
    let mut camera = Camera::new_default();
    let camera_controller = CameraController::new(10.0, 8.0);

    // load resources
    let shaders = Resource::from_relative_exe_path("res/shaders").unwrap();
    let models = Resource::from_relative_exe_path("res/models").unwrap();

    // load shaders
    let default_vert = Shader::new(
        ShaderType::VertexShader,
        shaders.read_file("default.vert").unwrap(),
    ).unwrap();
    let ray_create_frag = Shader::new(
        ShaderType::FragmentShader,
        shaders.read_file("ray_create.frag").unwrap(),
    ).unwrap();
    let ray_trace_frag = Shader::new(
        ShaderType::FragmentShader,
        shaders.read_file("ray_trace.frag").unwrap(),
    ).unwrap();
    let display_frag = Shader::new(
        ShaderType::FragmentShader,
        shaders.read_file("display.frag").unwrap(),
    ).unwrap();

    let ray_create_program = ShaderProgramBuilder::new()
        .add_shader(&default_vert)
        .add_shader(&ray_create_frag)
        .build()
        .unwrap();

    let ray_trace_program = ShaderProgramBuilder::new()
        .add_shader(&default_vert)
        .add_shader(&ray_trace_frag)
        .build()
        .unwrap();

    let display_program = ShaderProgramBuilder::new()
        .add_shader(&default_vert)
        .add_shader(&display_frag)
        .build()
        .unwrap();

    // load models
    let (model_vertices, model_triangles) = ModelParser::parse(models.read_file("armadillo_midres.obj").unwrap()).unwrap();

    let start = SystemTime::now();
    let (model_vertices, model_triangles, model_nodes) = BVHBuilder::new(model_vertices, model_triangles).build();
    let end = SystemTime::now();
    let build_time = end.duration_since(start).expect("Time measurement failed!");
    println!("{build_time:?}");

    // get shader uniform locations
    let ray_create_right_loc = ray_create_program.uniform_location("right");
    let ray_create_up_loc = ray_create_program.uniform_location("up");
    let ray_create_front_loc = ray_create_program.uniform_location("front");

    let ray_trace_dir_tex_loc = ray_trace_program.uniform_location("dirTex");
    let ray_trace_org_loc = ray_trace_program.uniform_location("org");

    let display_program_tex_loc = display_program.uniform_location("display");

    // create frame buffers
    let mut ray_dir_framebuffer = Framebuffer::new();
    let mut ray_dir_texture = Texture::new(
        window.width(),
        window.height(),
        TextureFormat::RGB32F,
        TextureFilter::Nearest,
    );
    ray_dir_framebuffer.attach_texture(&ray_dir_texture, TextureAttachment::Color(0));
    ray_dir_framebuffer.bind_draw_buffers();

    let mut col_framebuffer = Framebuffer::new();
    let mut col_texture = Texture::new(
        window.width(),
        window.height(),
        TextureFormat::RGB32F,
        TextureFilter::Nearest,
    );
    col_framebuffer.attach_texture(&col_texture, TextureAttachment::Color(0));
    col_framebuffer.bind_draw_buffers();

    // create square geometry
    let (_vbo, _ibo, square_geometry) = GeometrySetBuilder::create_square_geometry();

    // create buffer with mesh data
    let vertex_ssbo = ShaderStorageBuffer::new();
    let triangle_ssbo = ShaderStorageBuffer::new();
    let node_ssbo = ShaderStorageBuffer::new();
    vertex_ssbo.buffer_data(&model_vertices);
    triangle_ssbo.buffer_data(&model_triangles);
    node_ssbo.buffer_data(&model_nodes);

    while !window.should_close() {
        // handle events
        window.handle_events();
        camera_controller.control(&mut camera, &window, window.dt());
        println!("{}", 1.0 / window.dt());

        // todo: move everything opengl-related from here to render thread

        // update buffers
        if window.resized() {
            unsafe { gl::Viewport(0, 0, window.width() as i32, window.height() as i32) }
            ray_dir_texture.resize(window.width(), window.height());
            col_texture.resize(window.width(), window.height());
        }

        // create rays
        ray_dir_framebuffer.bind();
        ray_create_program.bind();
        let cvv = camera.generate_view_vectors(&window);
        ray_create_program.set_uniform_3f(ray_create_right_loc, cvv.right);
        ray_create_program.set_uniform_3f(ray_create_up_loc, cvv.up);
        ray_create_program.set_uniform_3f(ray_create_front_loc, cvv.front);
        square_geometry.draw();

        // ray trace
        col_framebuffer.bind();
        ray_trace_program.bind();
        ray_dir_texture.bind_to_slot(0);
        ray_trace_program.set_uniform_texture(ray_trace_dir_tex_loc, 0);
        ray_trace_program.set_uniform_3f(ray_trace_org_loc, cvv.pos);
        vertex_ssbo.bind_to_slot(0);
        triangle_ssbo.bind_to_slot(1);
        node_ssbo.bind_to_slot(2);
        square_geometry.draw();

        // render ray data onto screen
        col_framebuffer.unbind();
        display_program.bind();
        col_texture.bind_to_slot(0);
        display_program.set_uniform_texture(display_program_tex_loc, 0);
        square_geometry.draw();

        window.update();
    }

    window.close();
}
