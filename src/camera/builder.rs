use std::f32::consts::PI;

use glam::Vec3A as Vec3;

use super::{
    Camera,
    Stores,
};
use crate::hittable::Hittable;

pub struct Builder {
    world: Box<dyn Hittable>,
    stores: Stores,
    width: usize,
    aspect_ratio: f32,
    samples_per_pixel: usize,
    max_depth: usize,
    vertical_fov: f32,
    look_from: Vec3,
    look_at: Vec3,
    vup: Vec3,
    defocus_angle: f32,
    focus_dist: f32,
    quiet: bool,
}

impl Builder {
    pub fn new(world: impl Hittable + 'static, stores: Stores) -> Self {
        Self {
            world: Box::new(world),
            stores,
            width: 2000,
            aspect_ratio: 16.0 / 9.0,
            samples_per_pixel: 400,
            max_depth: 50,
            vertical_fov: 90.0,
            look_from: Vec3::ZERO,
            look_at: Vec3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,
            quiet: false,
        }
    }

    pub fn draft(mut self) -> Self {
        self.width = 400;
        self.samples_per_pixel = 100;
        self.max_depth = 20;
        self
    }

    pub fn samples_per_pixel(mut self, samples_per_pixel: usize) -> Self {
        self.samples_per_pixel = samples_per_pixel;
        self
    }

    pub fn max_depth(mut self, max_depth: usize) -> Self {
        self.max_depth = max_depth;
        self
    }

    pub fn width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }

    pub fn aspect_ratio(mut self, aspect_ratio: f32) -> Self {
        self.aspect_ratio = aspect_ratio;
        self
    }

    pub fn vertical_fov(mut self, vertical_fov: f32) -> Self {
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

    pub fn defocus_angle(mut self, defocus_angle: f32) -> Self {
        self.defocus_angle = defocus_angle;
        self
    }

    pub fn focus_dist(mut self, focus_dist: f32) -> Self {
        self.focus_dist = focus_dist;
        self
    }

    pub fn quiet(mut self, quiet: bool) -> Self {
        self.quiet = quiet;
        self
    }

    pub fn build(self) -> Camera {
        let camera_center = self.look_from;

        let height = (self.width as f32 / self.aspect_ratio) as usize;

        let theta = self.vertical_fov * (PI / 180.0);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.width as f32 / height as f32);

        let w = (self.look_from - self.look_at).normalize();
        let u = self.vup.cross(w).normalize();
        let v = w.cross(u);

        let viewport_u = u * viewport_width;
        let viewport_v = (-v) * viewport_height;

        let pixel_delta_u = viewport_u / self.width as f32;
        let pixel_delta_v = viewport_v / height as f32;

        let viewport_upper_left =
            camera_center - w * self.focus_dist - (viewport_u + viewport_v) * 0.5;

        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        let theta = self.defocus_angle * (PI / 180.0);
        let defocus_radius = self.focus_dist * (theta / 2.0).tan();
        let defocus_dist_u = u * defocus_radius;
        let defocus_dist_v = v * defocus_radius;

        Camera {
            world: self.world,
            stores: self.stores,
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
            quiet: self.quiet,
        }
    }
}
