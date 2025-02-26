mod builder;

use std::{
    fs::File,
    io::{
        BufWriter,
        Write,
    },
    time::Instant,
};

pub use builder::Builder;
use glam::Vec3A as Vec3;
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
    ray::Ray,
    rng::ThreadRng,
    vec3_ext::Vec3Ext,
    world::World,
};

#[derive(Clone)]
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
    defocus_dist_u: Vec3,
    defocus_dist_v: Vec3,
    quiet: bool,
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

        colors
            .into_iter()
            .progress_count(self.width * self.height)
            .for_each(|color| {
                writeln!(writer, "{color}").unwrap();
            });
    }

    fn sample_ray_origin(&self) -> Vec3 {
        let p = Vec3::random_in_unit_disk();
        self.camera_center + self.defocus_dist_u * p.x + self.defocus_dist_v * p.y
    }

    fn pixel_color(&self, x: u64, y: u64) -> Color {
        let samples: Vec<_> = (0..self.samples_per_pixel)
            .map(|_| {
                let ray_origin = self.sample_ray_origin();
                let ray_direction = self.sample_location(x, y) - ray_origin;
                let ray = Ray::new(ray_origin, ray_direction);
                self.color(&ray, self.max_depth)
            })
            .collect();
        Color::average(&samples).unwrap()
    }

    pub fn render(&self) -> Vec<Color> {
        if !self.quiet {
            println!("Rendering...");
        }

        let progress_bar = if self.quiet {
            ProgressBar::hidden()
        } else {
            ProgressBar::new(self.height * self.width)
        };

        let start = Instant::now();
        let pixels: Vec<_> = (0..self.height)
            .cartesian_product(0..self.width)
            .progress_with(progress_bar)
            .par_bridge()
            .map(|(y, x)| (x as usize, y as usize, self.pixel_color(x, y)))
            .collect();

        if !self.quiet {
            println!("  Done in {:?}", start.elapsed());
        }

        let mut result = vec![Color::default(); self.height as usize * self.width as usize];
        for (x, y, color) in pixels {
            result[y * self.width as usize + x] = color;
        }
        result
    }

    fn sample_location(&self, x: u64, y: u64) -> Vec3 {
        let rand_x = ThreadRng::random_range(-0.5..0.5);
        let rand_y = ThreadRng::random_range(-0.5..0.5);
        self.pixel00_loc
            + (self.pixel_delta_u * (x as f32 + rand_x))
            + (self.pixel_delta_v * (y as f32 + rand_y))
    }

    pub fn color(&self, r: &Ray, depth: u64) -> Color {
        if depth == 0 {
            return BLACK;
        }

        let interval = 0.001..f32::MAX;
        if let Some((scattered, attenuation)) = self.world.bounce(r, &interval) {
            attenuation * self.color(&scattered, depth - 1)
        } else {
            Self::background(r)
        }
    }

    pub fn background(r: &Ray) -> Color {
        let unit_direction = r.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        WHITE.lerp(&LIGHT_BLUE, t).unwrap()
    }
}
