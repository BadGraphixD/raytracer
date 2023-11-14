use std::sync::{Arc, Mutex};
use crate::rendering::camera::Camera;
use crate::window::window::Window;

pub struct CameraController {
    window: Arc<Mutex<Window>>,
    movement_speed: f32,
    mouse_sensitivity: f32,
}

impl CameraController {
    pub fn new(window: Arc<Mutex<Window>>, movement_speed: f32, mouse_sensitivity: f32) -> Self {
        Self {
            window,
            movement_speed,
            mouse_sensitivity,
        }
    }

    pub fn control(&self, camera: &mut Camera) {
        let window = self.window.lock().unwrap();
        let (cursor_x, cursor_y) = window.input().cursor_pos();
        let (input_x, input_y, input_z) = window.input().movement();

        let move_factor = window.dt() * self.movement_speed;
        let yaw = (1.0 - cursor_x / window.width() as f32 * 2.0) * self.mouse_sensitivity;
        let pitch = (1.0 - cursor_y / window.height() as f32 * 2.0) * self.mouse_sensitivity;

        camera.set_rotation(yaw, pitch);
        camera.add_position(
            (input_x * yaw.sin() + input_z * yaw.cos()) * move_factor,
            input_y * move_factor,
            (input_x * yaw.cos() - input_z * yaw.sin()) * move_factor,
        );
    }
}
