use cgmath::{Matrix4, SquareMatrix};
use crate::gl_wrapper::buffer::{ShaderStorageBuffer};
use crate::gl_wrapper::framebuffer::Framebuffer;
use crate::gl_wrapper::geometry_set::GeometrySetBuilder;
use crate::gl_wrapper::texture::Texture;
use crate::gl_wrapper::types::{TextureAttachment, TextureFilter, TextureFormat};
use crate::rendering::camera::Camera;
use crate::resource::resource_manager::ResourceManager;
use rendering::camera_controller::CameraController;
use crate::gl_wrapper::renderbuffer::Renderbuffer;
use crate::window::window::Window;

pub mod gl_wrapper;
pub mod raytracing;
pub mod rendering;
pub mod util;
pub mod window;
pub mod resource;

fn main() {
    // create window
    let mut window = Window::new(1000, 800, "Raytracing :)").expect("Failed to create window!");
    let mut camera = Camera::new_default();
    let camera_controller = CameraController::new(1.0, 8.0);

    // load resources
    let mut resource_manager = ResourceManager::new("res/models", "res/textures", "res/shaders").expect("Failed to create resource manager");

    let model = resource_manager.get_model("f16.obj").expect("Failed to load model resources");
    model.lock().unwrap().build_bvh();

    let g_buffer_program = resource_manager.create_shader_program(
        "gBuffer", "rasterize/default.vert", "rasterize/default.frag"
    ).expect("Failed to load shader");
    let ray_dispatch_program = resource_manager.create_shader_program(
        "rayDispatch", "util/quad01.vert", "ray_dispatcher.frag"
    ).expect("Failed to load shader");
    let ray_trace_program = resource_manager.create_shader_program(
        "rayTrace", "util/quad01.vert", "ray_trace.frag"
    ).expect("Failed to load shader");
    let display_program = resource_manager.create_shader_program(
        "display", "util/quad01.vert", "util/display.frag"
    ).expect("Failed to load shader");

    // create frame buffers
    let mut g_buffer = Framebuffer::new();
    let mut position_tex = Texture::new(
        window.width(), window.height(),
        TextureFormat::RGB32F,
        TextureFilter::Nearest,
    );
    let mut normal_mat_tex = Texture::new(
        window.width(), window.height(),
        TextureFormat::RGBA32F,
        TextureFilter::Nearest,
    );
    let mut tex_coord_tex = Texture::new(
        window.width(), window.height(),
        TextureFormat::RG32F,
        TextureFilter::Nearest,
    );
    let mut depth_tex = Renderbuffer::new(
        window.width(), window.height(),
        TextureFormat::Depth,
    );
    g_buffer.attach_renderbuffer(&depth_tex, TextureAttachment::Depth);
    g_buffer.attach_texture(&position_tex, TextureAttachment::Color(0));
    g_buffer.attach_texture(&normal_mat_tex, TextureAttachment::Color(1));
    g_buffer.attach_texture(&tex_coord_tex, TextureAttachment::Color(2));
    g_buffer.bind_draw_buffers();

    // create geometry
    let (quad_geometry, _ibo, _vbo) = GeometrySetBuilder::create_square_geometry();
    let (model_geometry, _m_ibo, _m_vbo) = GeometrySetBuilder::from_model(model.clone());

    // create buffer with mesh
    let node_ssbo = ShaderStorageBuffer::new();
    let triangle_ssbo = ShaderStorageBuffer::new();
    let position_ssbo = ShaderStorageBuffer::new();
    let tex_coord_ssbo = ShaderStorageBuffer::new();
    let normal_ssbo = ShaderStorageBuffer::new();
    node_ssbo.buffer_data(model.lock().unwrap().get_bvh().unwrap().data());
    triangle_ssbo.buffer_data(model.lock().unwrap().triangles());
    position_ssbo.buffer_data(model.lock().unwrap().positions());
    if let Some(model_uvs) = model.lock().unwrap().tex_coords() { tex_coord_ssbo.buffer_data(model_uvs) }
    if let Some(model_normals) = model.lock().unwrap().normals() { normal_ssbo.buffer_data(model_normals) }

    Framebuffer::set_clear_color(0.0, 0.0, 0.0, 0.0);

    while !window.should_close() {
        // handle events
        window.handle_events();
        camera_controller.control(&mut camera, &window, window.dt());
        println!("FPS: {}", (1.0 / window.dt()) as i32);

        // update buffers
        if window.resized() {
            unsafe { gl::Viewport(0, 0, window.width() as i32, window.height() as i32) }
            position_tex.resize(window.width(), window.height());
            normal_mat_tex.resize(window.width(), window.height());
            tex_coord_tex.resize(window.width(), window.height());
            depth_tex.resize(window.width(), window.height());
        }

        let cvv = camera.generate_view_vectors(&window);
        let vp_mat = camera.view_proj_matrices(&window);

        // clear g-buffer and render
        g_buffer.bind();
        Framebuffer::clear_color_depth();
        Framebuffer::enable_depth_test();
        {
            let mut program = g_buffer_program.lock().unwrap();
            program.bind();
            program.set_uniform_mat_4f(0, vp_mat.proj * vp_mat.view);
            program.set_uniform_1i(1, 0);
        }
        model_geometry.draw();

        // temp: draw g-buffer pos to screen
        Framebuffer::bind_default();
        Framebuffer::disable_depth_test();
        {
            let mut program = display_program.lock().unwrap();
            program.bind();
            position_tex.bind_to_slot(0);
            program.set_uniform_texture(0, 0);
        }
        quad_geometry.draw();

        window.update();
    }

    window.close();
}
