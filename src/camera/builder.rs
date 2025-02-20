use std::f64::consts::PI;

use super::Camera;
use crate::{
    vec3,
    vec3::Vec3,
    world::World,
};

pub struct Builder {
    world: World,
    width: u64,
    aspect_ratio: f64,
    samples_per_pixel: u64,
    max_depth: u64,
    /// Vertical field of view in degrees
    vertical_fov: f64,
}

impl Builder {
    pub fn new(world: World) -> Self {
        Self {
            world,
            width: 2000,
            aspect_ratio: 16.0 / 9.0,
            samples_per_pixel: 200,
            max_depth: 100,
            vertical_fov: 90.0,
        }
    }

    pub fn draft(mut self) -> Self {
        self.width = 400;
        self.samples_per_pixel = 100;
        self.max_depth = 20;
        self
    }

    pub fn samples_per_pixel(mut self, samples_per_pixel: u64) -> Self {
        self.samples_per_pixel = samples_per_pixel;
        self
    }

    pub fn max_depth(mut self, max_depth: u64) -> Self {
        self.max_depth = max_depth;
        self
    }

    pub fn width(mut self, width: u64) -> Self {
        self.width = width;
        self
    }

    pub fn aspect_ratio(mut self, aspect_ratio: f64) -> Self {
        self.aspect_ratio = aspect_ratio;
        self
    }

    pub fn vertical_fov(mut self, vertical_fov: f64) -> Self {
        self.vertical_fov = vertical_fov;
        self
    }

    pub fn build(self) -> Camera {
        let camera_center = vec3::ORIGIN;
        let focal_length = 1.0;

        let height = (self.width as f64 / self.aspect_ratio) as u64;

        let theta = self.vertical_fov * (PI / 180.0);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * (self.width as f64 / height as f64);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / self.width as f64;
        let pixel_delta_v = viewport_v / height as f64;

        let viewport_upper_left =
            camera_center - Vec3::new(0.0, 0.0, focal_length) - (viewport_u + viewport_v) * 0.5;

        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        Camera {
            world: self.world,
            width: self.width,
            height,
            camera_center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel: self.samples_per_pixel,
            max_depth: self.max_depth,
        }
    }
}
