use crate::gl_wrapper::buffer::{ShaderStorageBuffer};
use crate::gl_wrapper::framebuffer::Framebuffer;
use crate::gl_wrapper::geometry_set::GeometrySetBuilder;
use crate::gl_wrapper::texture::Texture;
use crate::gl_wrapper::types::{TextureAttachment, TextureFilter, TextureFormat};
use crate::rendering::camera::Camera;
use crate::resource::resource_manager::ResourceManager;
use crate::util::camera_controller::CameraController;
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

    // config
    let model_name = "f16.obj";
    let ray_create_shader_names = ("quad.vert", "ray_create.frag");
    let ray_trace_shader_names = ("quad.vert", "ray_trace_and_shade.frag");
    let display_shader_names = ("quad.vert", "display.frag");

    // load resources
    let mut resource_manager = ResourceManager::new("res/models", "res/textures", "res/shaders").expect("Failed to create resource manager");

    resource_manager.load_model(model_name).expect("Failed to load model resources");

    resource_manager.load_shader_program(ray_create_shader_names).expect("Failed to load shader");
    resource_manager.load_shader_program(ray_trace_shader_names).expect("Failed to load shader");
    resource_manager.load_shader_program(display_shader_names).expect("Failed to load shader");

    resource_manager.build_model_bvh(model_name).expect("Failed to build model bvh");

    let model_bvh = resource_manager.get_model_bvh(model_name).unwrap();
    let model = resource_manager.get_model(model_name).unwrap();
    let model_texture = resource_manager.get_texture("F16s.bmp").expect("Texture not present");
    let ray_create_program = resource_manager.get_shader_program(ray_create_shader_names).unwrap();
    let ray_trace_program = resource_manager.get_shader_program(ray_trace_shader_names).unwrap();
    let display_program = resource_manager.get_shader_program(display_shader_names).unwrap();

    // create frame buffers
    let mut ray_dir_framebuffer = Framebuffer::new();
    let mut ray_dir_texture = Texture::new(
        window.width(), window.height(),
        TextureFormat::RGB32F,
        TextureFilter::Nearest,
    );
    ray_dir_framebuffer.attach_texture(&ray_dir_texture, TextureAttachment::Color(0));
    ray_dir_framebuffer.bind_draw_buffers();

    let mut col_framebuffer = Framebuffer::new();
    let mut col_texture = Texture::new(
        window.width(), window.height(),
        TextureFormat::RGB32F,
        TextureFilter::Nearest,
    );
    col_framebuffer.attach_texture(&col_texture, TextureAttachment::Color(0));
    col_framebuffer.bind_draw_buffers();

    // create geometry
    let (quad_geometry, _ibo, _vbo) = GeometrySetBuilder::create_square_geometry();
    //let (model_geometry, _m_ibo, _m_vbo) = GeometrySetBuilder::from_model(&model); // todo: things stop working when this line is second ???

    // create buffer with mesh
    let node_ssbo = ShaderStorageBuffer::new();
    let triangle_ssbo = ShaderStorageBuffer::new();
    let position_ssbo = ShaderStorageBuffer::new();
    let tex_coord_ssbo = ShaderStorageBuffer::new();
    let normal_ssbo = ShaderStorageBuffer::new();
    node_ssbo.buffer_data(model_bvh.data());
    triangle_ssbo.buffer_data(model.triangles());
    position_ssbo.buffer_data(model.positions());
    if let Some(model_uvs) = model.tex_coords() { tex_coord_ssbo.buffer_data(model_uvs) }
    if let Some(model_normals) = model.normals() { normal_ssbo.buffer_data(model_normals) }

    while !window.should_close() {
        // handle events
        window.handle_events();
        camera_controller.control(&mut camera, &window, window.dt());
        println!("FPS: {}", (1.0 / window.dt()) as i32);

        // todo: move everything opengl-related from here to render thread

        // update buffers
        if window.resized() {
            unsafe { gl::Viewport(0, 0, window.width() as i32, window.height() as i32) }
            ray_dir_texture.resize(window.width(), window.height());
            col_texture.resize(window.width(), window.height());
        }

        let cvv = camera.generate_view_vectors(&window);

        // create rays
        ray_dir_framebuffer.bind();
        ray_create_program.bind();
        ray_create_program.set_uniform_3f("right", cvv.right);
        ray_create_program.set_uniform_3f("up", cvv.up);
        ray_create_program.set_uniform_3f("front", cvv.front);
        quad_geometry.draw();

        // ray trace
        col_framebuffer.bind();
        ray_trace_program.bind();
        ray_dir_texture.bind_to_slot(0);
        ray_trace_program.set_uniform_texture("dirTex", 0);
        ray_trace_program.set_uniform_3f("org", cvv.pos);

        // ### if also shade ###
        model_texture.bind_to_slot(1);
        ray_trace_program.set_uniform_texture("modelAlbedo", 1);
        ray_trace_program.set_uniform_1b("hasTexCoords", model.has_tex_coords());
        ray_trace_program.set_uniform_1b("hasNormals", model.has_normals());
        if model.has_tex_coords() { tex_coord_ssbo.bind_to_slot(3) }
        if model.has_normals() { normal_ssbo.bind_to_slot(4) }
        // ###

        node_ssbo.bind_to_slot(0);
        triangle_ssbo.bind_to_slot(1);
        position_ssbo.bind_to_slot(2);
        quad_geometry.draw();

        // render ray data onto screen
        col_framebuffer.unbind();
        display_program.bind();
        col_texture.bind_to_slot(0);
        display_program.set_uniform_texture("display", 0);
        quad_geometry.draw();

        window.update();
    }

    window.close();
}
