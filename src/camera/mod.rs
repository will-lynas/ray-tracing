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

use crate::{
    color::{
        Color,
        BLACK,
        LIGHT_BLUE,
        WHITE,
    },
    itertools::Itertools,
    progress_bar::{
        ProgressBar,
        ProgressBarIter,
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

    fn pixel_color(&self, x: u64, y: u64) -> Color {
        let samples: Vec<_> = (0..self.samples_per_pixel)
            .map(|_| {
                let ray_direction = self.sample_location(x, y) - self.camera_center;
                let ray = Ray::new(self.camera_center, ray_direction);
                self.color(&ray, self.max_depth)
            })
            .collect();
        Color::average(&samples).unwrap()
    }

    pub fn render(&self) -> Vec<Color> {
        println!("Rendering...");
        let start = Instant::now();
        let mut bar = ProgressBar::new(self.height * self.width);
        let colors = (0..self.height)
            .cartesian_product(0..self.width)
            .map(|(y, x)| {
                let color = self.pixel_color(x, y);
                bar.increment();
                color
            })
            .collect();
        println!("  Done in {:?}", start.elapsed());
        colors
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
