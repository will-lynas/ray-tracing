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
    vertical_fov: f64,
    look_from: Vec3,
    look_at: Vec3,
    vup: Vec3,
    defocus_angle: f64,
    focus_dist: f64,
}

impl Builder {
    pub fn new(world: World) -> Self {
        Self {
            world,
            width: 4000,
            aspect_ratio: 16.0 / 9.0,
            samples_per_pixel: 400,
            max_depth: 200,
            vertical_fov: 90.0,
            look_from: vec3::ORIGIN,
            look_at: Vec3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,
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

    pub fn look_from(mut self, look_from: Vec3) -> Self {
        self.look_from = look_from;
        self
    }

    pub fn look_at(mut self, look_at: Vec3) -> Self {
        self.look_at = look_at;
        self
    }

    pub fn vup(mut self, vup: Vec3) -> Self {
        self.vup = vup;
        self
    }

    pub fn defocus_angle(mut self, defocus_angle: f64) -> Self {
        self.defocus_angle = defocus_angle;
        self
    }

    pub fn focus_dist(mut self, focus_dist: f64) -> Self {
        self.focus_dist = focus_dist;
        self
    }

    pub fn build(self) -> Camera {
        let camera_center = self.look_from;

        let height = (self.width as f64 / self.aspect_ratio) as u64;

        let theta = self.vertical_fov * (PI / 180.0);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.width as f64 / height as f64);

        let w = (self.look_from - self.look_at).unit_vector();
        let u = self.vup.cross(&w).unit_vector();
        let v = w.cross(&u);

        let viewport_u = u * viewport_width;
        let viewport_v = (-v) * viewport_height;

        let pixel_delta_u = viewport_u / self.width as f64;
        let pixel_delta_v = viewport_v / height as f64;

        let viewport_upper_left =
            camera_center - w * self.focus_dist - (viewport_u + viewport_v) * 0.5;

        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        let theta = self.defocus_angle * (PI / 180.0);
        let defocus_radius = self.focus_dist * (theta / 2.0).tan();
        let defocus_dist_u = u * defocus_radius;
        let defocus_dist_v = v * defocus_radius;

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
            defocus_dist_u,
            defocus_dist_v,
        }
    }
}
