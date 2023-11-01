use std::ops::{Add, Div, Mul};
use cgmath::{Angle, Deg, InnerSpace, Point3, Rad, Vector3};
use crate::window::window::Window;

pub struct Camera {
    fov: Rad<f32>,

    position: Point3<f32>,

    up: Vector3<f32>,
    direction: Vector3<f32>,
}

impl Camera {
    pub fn new(fov: Rad<f32>, position: Point3<f32>, up: Vector3<f32>, direction: Vector3<f32>) -> Self {
        Self { fov, position, up, direction }
    }

    pub fn new_default() -> Self {
        Self::new(
            Rad::from(Deg(120.0)),
            Point3::new(0.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0)
        )
    }

    pub fn add_position(&mut self, x: f32, y: f32, z: f32) {
        self.position = self.position.add(Vector3::new(x, y, z));
    }

    pub fn set_rotation(&mut self, yaw: f32, pitch: f32) {
        let pi = std::f32::consts::PI;
        let pitch = f32::max(f32::min(pitch, pi / 2.1), -pi / 2.1);
        let sp = pitch.sin();
        let cp = pitch.cos();
        let sy = yaw.sin();
        let cy = yaw.cos();
        self.direction = Vector3::new(cy * cp, sp, -sy * cp).normalize();
    }

    pub fn generate_view_vectors(&self, window: &Window) -> CameraViewVectors {
        let sin_fov = self.fov.div(2.0).sin();
        let cos_fov = self.fov.div(2.0).cos();

        let front  = self.direction.clone().normalize();
        let right  = self.direction.cross(self.up).normalize();
        let up = right.cross(self.direction).normalize();

        CameraViewVectors {
            right: right.mul(sin_fov),
            up: up.mul(sin_fov / window.aspect()),
            front: front.mul(cos_fov),
            pos: Vector3::new(self.position.x, self.position.y, self.position.z),
        }
    }
}

pub struct CameraViewVectors {
    pub right: Vector3<f32>,
    pub up: Vector3<f32>,
    pub front: Vector3<f32>,
    pub pos: Vector3<f32>,
}
