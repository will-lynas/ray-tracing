use crate::color::Color;
use crate::ray::Ray;
use crate::vec3;
use crate::vec3::Vec3;
use crate::world::World;
use indicatif::ProgressIterator;
use itertools::Itertools;
use rand::Rng;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Instant;

pub struct Camera {
    world: World,
    width: u64,
    height: u64,
    camera_center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    samples_per_pixel: u64,
    max_depth: u64,
}

impl Camera {
    pub fn render_to_file(&self, file_name: &str) {
        let colors = self.render();

        println!("Writing to file...");
        let file = File::create(file_name).unwrap();
        let mut writer = BufWriter::new(file);

        writeln!(writer, "P3").unwrap();
        writeln!(writer, "{} {}", self.width, self.height).unwrap();
        writeln!(writer, "255").unwrap();

        colors.into_iter().progress().for_each(|color| {
            writeln!(writer, "{}", color).unwrap();
        });
    }

    pub fn render(&self) -> Vec<Color> {
        println!("Rendering...");
        let start = Instant::now();
        let colors = (0..self.height)
            .cartesian_product(0..self.width)
            .progress_count(self.width * self.height)
            .map(|(y, x)| {
                let mut samples = Vec::new();
                for _ in 0..self.samples_per_pixel {
                    let ray_direction = self.sample_location(x, y) - self.camera_center;
                    let ray = Ray::new(self.camera_center, ray_direction);
                    samples.push(self.color(&ray, self.max_depth));
                }
                Color::average(&samples).unwrap()
            })
            .collect();
        println!("  Done in {:?}", start.elapsed());
        colors
    }

    fn sample_location(&self, x: u64, y: u64) -> Vec3 {
        let mut rng = rand::rng();
        let rand_x = rng.random_range(-0.5..=0.5);
        let rand_y = rng.random_range(-0.5..=0.5);
        self.pixel00_loc
            + (self.pixel_delta_u * (x as f64 + rand_x))
            + (self.pixel_delta_v * (y as f64 + rand_y))
    }

    pub fn color(&self, r: &Ray, depth: u64) -> Color {
        if depth == 0 {
            return Color::black();
        }

        let interval = 0.0..f64::MAX;
        if let Some(hit_record) = self.world.hit(r, &interval) {
            let dir = hit_record.normal.random_in_hemisphere();
            let ray = Ray::new(hit_record.point, dir);
            self.color(&ray, depth - 1).mul(0.5).unwrap()
        } else {
            Self::background(r)
        }
    }

    pub fn background(r: &Ray) -> Color {
        let unit_direction = r.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        Color::white().lerp(&Color::blue(), t).unwrap()
    }
}

pub struct CameraBuilder {
    world: World,
    width: u64,
    aspect_ratio: f64,
    viewport_height: f64,
    focal_length: f64,
    camera_center: Vec3,
    samples_per_pixel: u64,
    max_depth: u64,
}

impl CameraBuilder {
    pub fn new(world: World) -> Self {
        Self {
            world,
            width: 400,
            aspect_ratio: 16.0 / 9.0,
            viewport_height: 2.0,
            focal_length: 1.0,
            camera_center: vec3::ORIGIN,
            samples_per_pixel: 10,
            max_depth: 50,
        }
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

    pub fn viewport_height(mut self, viewport_height: f64) -> Self {
        self.viewport_height = viewport_height;
        self
    }

    pub fn focal_length(mut self, focal_length: f64) -> Self {
        self.focal_length = focal_length;
        self
    }

    pub fn camera_center(mut self, camera_center: Vec3) -> Self {
        self.camera_center = camera_center;
        self
    }

    pub fn build(self) -> Camera {
        let height = (self.width as f64 / self.aspect_ratio) as u64;
        let viewport_width = self.viewport_height * (self.width as f64 / height as f64);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -self.viewport_height, 0.0);

        let pixel_delta_u = viewport_u / self.width as f64;
        let pixel_delta_v = viewport_v / height as f64;

        let viewport_upper_left = self.camera_center
            - Vec3::new(0.0, 0.0, self.focal_length)
            - (viewport_u + viewport_v) * 0.5;

        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        Camera {
            world: self.world,
            width: self.width,
            height,
            camera_center: self.camera_center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel: self.samples_per_pixel,
            max_depth: self.max_depth,
        }
    }
}
