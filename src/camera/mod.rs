mod builder;

use std::{
    fs::{
        self,
        create_dir,
    },
    ops::Range,
    time::{
        Instant,
        SystemTime,
    },
};

pub use builder::Builder;
use chrono::DateTime;
use glam::Vec3A as Vec3;
use image::{
    save_buffer,
    ColorType,
};
use indicatif::{
    ProgressBar,
    ProgressIterator,
};
use itertools::Itertools;
use rayon::prelude::*;

use crate::{
    color::{
        Color,
        BLACK,
        LIGHT_BLUE,
        WHITE,
    },
    extension_traits::Vec3Ext,
    hittable::Hittable,
    rng::ThreadRng,
    timed_ray::TimedRay,
};

pub struct Camera {
    world: Box<dyn Hittable>,
    width: usize,
    height: usize,
    camera_center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    samples_per_pixel: usize,
    max_depth: usize,
    defocus_dist_u: Vec3,
    defocus_dist_v: Vec3,
    quiet: bool,
}

impl Camera {
    fn get_filename() -> String {
        let now = SystemTime::now();
        let datetime = DateTime::<chrono::Local>::from(now);
        let timestamp = datetime.format("%y%m%d_%H%M%S").to_string();
        format!("{timestamp}.png")
    }

    pub fn render_to_file(&self) {
        let prev_filename = "last_run.png";
        let buf: Vec<_> = self.render().iter().flat_map(Color::bytes).collect();
        save_buffer(
            prev_filename,
            &buf,
            self.width as u32,
            self.height as u32,
            ColorType::Rgb8,
        )
        .unwrap();

        let dir = std::path::Path::new("out");
        if !dir.exists() {
            create_dir(dir).unwrap();
        }
        let filename = Self::get_filename();
        let path = dir.join(filename);
        fs::copy(prev_filename, path).unwrap();
    }

    fn sample_ray_origin(&self) -> Vec3 {
        let p = Vec3::random_in_unit_disk();
        self.camera_center + self.defocus_dist_u * p.x + self.defocus_dist_v * p.y
    }

    fn pixel_color(&self, x: usize, y: usize) -> Color {
        let samples: Vec<_> = (0..self.samples_per_pixel)
            .map(|_| {
                let ray_origin = self.sample_ray_origin();
                let ray_direction = self.sample_location(x, y) - ray_origin;
                let ray_time = ThreadRng::random();
                let ray = TimedRay::new(ray_origin, ray_direction, ray_time);
                self.color(&ray, self.max_depth)
            })
            .collect();
        Color::average(&samples)
    }

    pub fn render(&self) -> Vec<Color> {
        if !self.quiet {
            println!("Rendering...");
        }

        let progress_bar = if self.quiet {
            ProgressBar::hidden()
        } else {
            ProgressBar::new((self.height * self.width) as u64)
        };

        let start = Instant::now();
        let pixels: Vec<_> = (0..self.height)
            .cartesian_product(0..self.width)
            .progress_with(progress_bar)
            .par_bridge()
            .map(|(y, x)| (x, y, self.pixel_color(x, y)))
            .collect();

        if !self.quiet {
            println!("Done in {:?}", start.elapsed());
        }

        let mut result = vec![Color::default(); self.height * self.width];
        for (x, y, color) in pixels {
            result[y * self.width + x] = color;
        }
        result
    }

    fn sample_location(&self, x: usize, y: usize) -> Vec3 {
        let rand_x = ThreadRng::random_range(-0.5..0.5);
        let rand_y = ThreadRng::random_range(-0.5..0.5);
        self.pixel00_loc
            + (self.pixel_delta_u * (x as f32 + rand_x))
            + (self.pixel_delta_v * (y as f32 + rand_y))
    }

    pub fn bounce(&self, r: &TimedRay, interval: &Range<f32>) -> Option<(TimedRay, Color)> {
        let hit_record = self.world.hit(r, interval)?;
        hit_record.material.scatter(&hit_record)
    }

    pub fn color(&self, r: &TimedRay, depth: usize) -> Color {
        if depth == 0 {
            return BLACK;
        }

        let interval = 0.001..f32::MAX;
        if let Some((scattered, attenuation)) = self.bounce(r, &interval) {
            attenuation * self.color(&scattered, depth - 1)
        } else {
            Self::background(r)
        }
    }

    pub fn background(r: &TimedRay) -> Color {
        let unit_direction = r.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        WHITE.lerp(&LIGHT_BLUE, t)
    }
}
