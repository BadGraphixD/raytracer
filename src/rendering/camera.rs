use std::ops::Add;
use cgmath::{Deg, InnerSpace, Matrix4, Point3, Rad, Vector3};
use crate::window::window::Window;

pub struct Camera {
    fov: Rad<f32>,
    near_plane: f32,
    far_plane: f32,
    up: Vector3<f32>,

    position: Point3<f32>,
    pitch: Rad<f32>,
    yaw: Rad<f32>,
}

impl Camera {
    pub fn new(fov: f32, near_plane: f32, far_plane: f32) -> Self {
        return Camera {
            fov: Rad::from(Deg(fov)),
            near_plane,
            far_plane,
            up: Vector3::unit_y(),
            position: Point3::new(0.0, 0.0, 0.0),
            pitch: Rad(0.0),
            yaw: Rad(0.0),
        };
    }

    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.position = Point3::new(x, y, z);
    }

    pub fn add_position(&mut self, x: f32, y: f32, z: f32) {
        self.position = self.position.add(Vector3::new(x, y, z));
    }

    pub fn set_rotation(&mut self, pitch: f32, yaw: f32) {
        self.pitch = Deg(pitch).into();
        self.yaw = Deg(yaw).into();
    }

    pub fn calc_projection_matrix(&self, window: &Window) -> Matrix4<f32> {
        return cgmath::perspective(
            self.fov,
            (window.width() / window.height()) as f32,
            self.near_plane,
            self.far_plane,
        );
    }

    pub fn calc_view_matrix(&self) -> Matrix4<f32> {
        let (sin_pitch, cos_pitch) = self.pitch.0.sin_cos();
        let (sin_yaw, cos_yaw) = self.yaw.0.sin_cos();

        return Matrix4::look_to_rh(
            self.position,
            InnerSpace::normalize(Vector3::new(
                cos_pitch * cos_yaw,
                sin_pitch,
                cos_pitch * sin_yaw,
            )),
            self.up,
        );
    }
}
