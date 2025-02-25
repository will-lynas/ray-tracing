mod builder;

use std::{
    fs::File,
    io::{
        BufWriter,
        Write,
    },
    sync::mpsc,
    thread::{
        self,
        available_parallelism,
    },
    time::Instant,
};

pub use builder::Builder;
use indicatif::{
    ProgressBar,
    ProgressIterator,
};
use itertools::Itertools;

use crate::{
    color::{
        Color,
        BLACK,
        LIGHT_BLUE,
        WHITE,
    },
    ray::Ray,
    rng::ThreadRng,
    vec3::Vec3,
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
        println!("Rendering...");
        let start = Instant::now();

        let num_threads = available_parallelism().unwrap().get();
        let mut batches = vec![Vec::new(); num_threads];
        for (y, x) in (0..self.height).cartesian_product(0..self.width) {
            let i = x + y * self.width;
            let batch_index = i as usize % num_threads;
            batches[batch_index].push((y, x));
        }

        let (tx, rx) = mpsc::channel();
        let handles: Vec<_> = batches
            .into_iter()
            .map(|batch| {
                let tx = tx.clone();
                let camera = self.clone();
                thread::spawn(move || {
                    for (y, x) in batch {
                        let color = camera.pixel_color(x, y);
                        tx.send(((y, x), color)).unwrap();
                    }
                })
            })
            .collect();
        drop(tx);

        let bar = ProgressBar::new(self.height * self.width);
        let mut result = vec![Color::default(); (self.height * self.width) as usize];

        for ((y, x), color) in rx {
            result[(y * self.width + x) as usize] = color;
            bar.inc(1);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        bar.finish_and_clear();
        println!("  Done in {:?}", start.elapsed());
        result
    }

    fn sample_location(&self, x: u64, y: u64) -> Vec3 {
        let rand_x = ThreadRng::random_range(-0.5..0.5);
        let rand_y = ThreadRng::random_range(-0.5..0.5);
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
        WHITE.lerp(&LIGHT_BLUE, t).unwrap()
    }
}
