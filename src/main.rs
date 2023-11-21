use std::sync::{Arc, Mutex};
use cgmath::{Vector3, Vector4};
use rand::{Rng, thread_rng};
use crate::gl_wrapper::buffer::{ShaderStorageBuffer};
use crate::gl_wrapper::framebuffer::Framebuffer;
use crate::gl_wrapper::geometry_set::GeometrySetBuilder;
use crate::gl_wrapper::types::{TextureAttachment, TextureFormat};
use crate::rendering::camera::Camera;
use crate::resource::resource_manager::ResourceManager;
use rendering::camera_controller::CameraController;
use crate::rendering::framebuffer_manager::FramebufferManager;
use crate::window::window::Window;

pub mod gl_wrapper;
pub mod raytracing;
pub mod rendering;
pub mod util;
pub mod window;
pub mod resource;

fn main() {
    // create window
    let window = Arc::new(Mutex::new(Window::new(1000, 800, "Raytracing :)").expect("Failed to create window!")));
    let mut camera = Camera::new_default(window.clone());
    let camera_controller = CameraController::new(window.clone(), 1.0, 8.0);

    // load resources
    let mut resource_manager = ResourceManager::new("res/models", "res/textures", "res/shaders").expect("Failed to create resource manager");

    let model = resource_manager.get_model("dragon.obj").expect("Failed to load model resources");
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

    // load blue noise texture
    let blue_noise_tex = resource_manager.get_texture("blue_noise.png").expect("Failed to load blue noise texture");

    // create frame buffers

    let mut fbo_manager = FramebufferManager::new(window.clone());

    let g_buffer = fbo_manager.new_framebuffer();
    let position_tex = fbo_manager.attach_texture(TextureFormat::RGB32F, TextureAttachment::Color(0), true);
    let normal_mat_tex = fbo_manager.attach_texture(TextureFormat::RGBA32F, TextureAttachment::Color(1), true);
    let tex_coord_tex = fbo_manager.attach_texture(TextureFormat::RG32F, TextureAttachment::Color(2), true);
    let _depth_rbo = fbo_manager.attach_renderbuffer(TextureFormat::Depth, TextureAttachment::Depth, false);

    let ray_buffer = fbo_manager.new_framebuffer();
    let ray_org_tex = fbo_manager.attach_texture(TextureFormat::RGB32F, TextureAttachment::Color(0), true);
    let shadow_ray_dir_tex = fbo_manager.attach_texture(TextureFormat::RGB32F, TextureAttachment::Color(1), true);
    let reflect_ray_dir_tex = fbo_manager.attach_texture(TextureFormat::RGB32F, TextureAttachment::Color(2), true);
    let ambient_ray_dir_tex = fbo_manager.attach_texture(TextureFormat::RGB32F, TextureAttachment::Color(3), true);

    let shadow_intersection_buffer = fbo_manager.new_framebuffer();
    let shadow_intersection_tex = fbo_manager.attach_texture(TextureFormat::RGBA32F, TextureAttachment::Color(0), true);

    let reflect_intersection_buffer = fbo_manager.new_framebuffer();
    let reflect_intersection_tex = fbo_manager.attach_texture(TextureFormat::RGBA32F, TextureAttachment::Color(0), true);

    let ambient_intersection_buffer = fbo_manager.new_framebuffer();
    let ambient_intersection_tex = fbo_manager.attach_texture(TextureFormat::RGBA32F, TextureAttachment::Color(0), true);

    fbo_manager.build_framebuffers();

    // create geometry
    let (quad_geometry, _ibo, _vbo) = GeometrySetBuilder::create_square_geometry();
    let (model_geometry, _m_ibo, _m_vbos) = GeometrySetBuilder::from_model(model.clone());

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

    while !window.lock().unwrap().should_close() {
        // handle events
        window.lock().unwrap().handle_events();
        camera_controller.control(&mut camera);

        println!("FPS: {}", (1.0 / window.lock().unwrap().dt()) as u32);

        // update buffers
        if window.lock().unwrap().resized() {
            unsafe {
                let window = window.lock().unwrap();
                gl::Viewport(0, 0, window.width() as i32, window.height() as i32)
            }
            fbo_manager.update_buffers();
        }

        let cvv = camera.generate_view_vectors();
        let vp_mat = camera.view_proj_matrices();

        // clear g-buffer and render
        fbo_manager.bind_fbo(g_buffer);
        Framebuffer::set_clear_color(0.0, 0.0, 0.0, 1e30); // materialIdx is set to 1e30 (code for "no material")
        Framebuffer::clear_color_depth();
        Framebuffer::enable_depth_test();
        {
            let mut program = g_buffer_program.lock().unwrap();
            program.bind();
            program.set_uniform_mat_4f(0, vp_mat.proj * vp_mat.view);
            program.set_uniform_1i(1, 0);
        }
        model_geometry.draw();
        Framebuffer::disable_depth_test();

        // create rays
        fbo_manager.bind_fbo(ray_buffer);
        Framebuffer::set_clear_color(0.0, 0.0, 0.0, 0.0); // ray org and dir is set to NO_RAY (=vec3(0, 0, 0))
        Framebuffer::clear_color();
        {
            let window = window.lock().unwrap();
            let noise_settings = Vector4::new(
                thread_rng().gen_range(0..blue_noise_tex.width()) as f32/ blue_noise_tex.width() as f32,
                thread_rng().gen_range(0..blue_noise_tex.height()) as f32 / blue_noise_tex.height() as f32,
                window.width() as f32 / blue_noise_tex.width() as f32,
                window.height() as f32 / blue_noise_tex.height() as f32,
            );
            let mut program = ray_dispatch_program.lock().unwrap();
            program.bind();
            program.set_uniform_texture(0, fbo_manager.bind_tex_to_slot(position_tex, 0));
            program.set_uniform_texture(1, fbo_manager.bind_tex_to_slot(normal_mat_tex, 1));
            program.set_uniform_texture(2, blue_noise_tex.bind_to_slot(2));
            program.set_uniform_3f(3, Vector3::new(5.0, 10.0, 5.0));
            program.set_uniform_3f(4, cvv.pos);
            program.set_uniform_4f(5, noise_settings);
        }
        quad_geometry.draw();

        // trace rays
        Framebuffer::set_clear_color(1e30, 0.0, 0.0, 0.0); // t-value of Intersection is set to MISS (=1e30)
        node_ssbo.bind_to_slot(0);
        triangle_ssbo.bind_to_slot(1);
        position_ssbo.bind_to_slot(2);
        {
            let mut program = ray_trace_program.lock().unwrap();
            program.bind();
            program.set_uniform_texture(1, fbo_manager.bind_tex_to_slot(ray_org_tex, 1));

            fbo_manager.bind_fbo(shadow_intersection_buffer);
            Framebuffer::clear_color();
            program.set_uniform_texture(0, fbo_manager.bind_tex_to_slot(shadow_ray_dir_tex, 0));
            quad_geometry.draw();

            fbo_manager.bind_fbo(reflect_intersection_buffer);
            Framebuffer::clear_color();
            program.set_uniform_texture(0, fbo_manager.bind_tex_to_slot(reflect_ray_dir_tex, 0));
            quad_geometry.draw();

            fbo_manager.bind_fbo(ambient_intersection_buffer);
            Framebuffer::clear_color();
            program.set_uniform_texture(0, fbo_manager.bind_tex_to_slot(ambient_ray_dir_tex, 0));
            quad_geometry.draw();
        }

        // temp: draw g-buffer pos to screen
        Framebuffer::bind_default();
        {
            let mut program = display_program.lock().unwrap();
            program.bind();
            program.set_uniform_texture(0, fbo_manager.bind_tex_to_slot(normal_mat_tex, 0));
            program.set_uniform_texture(0, 0);
        }
        quad_geometry.draw();

        window.lock().unwrap().update();
    }

    window.lock().unwrap().close();
}
