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
use indicatif::ProgressIterator;
use itertools::Itertools;
use rand::Rng;

use crate::{
    color::{
        Color,
        BLACK,
        BLUE,
        WHITE,
    },
    ray::Ray,
    vec3::Vec3,
    world::World,
};

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
            writeln!(writer, "{color}").unwrap();
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
            return BLACK;
        }

        let interval = 0.001..f64::MAX;
        if let Some((scattered, attenuation)) = self.world.bounce(r, &interval) {
            attenuation * self.color(&scattered, depth - 1)
        } else {
            Self::background(r)
        }
    }

    pub fn background(r: &Ray) -> Color {
        let unit_direction = r.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        WHITE.lerp(&BLUE, t).unwrap()
    }
}
