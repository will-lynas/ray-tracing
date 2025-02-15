use crate::color::Color;
use crate::ray::Ray;
use crate::vec3;
use crate::vec3::Vec3;
use crate::world::World;
use indicatif::ProgressIterator;
use itertools::Itertools;
use std::fs::File;
use std::io::Write;

pub struct Camera {
    world: World,
}

impl Camera {
    pub fn new(world: World) -> Self {
        Self { world }
    }

    pub fn render(&self, file_name: &str) {
        let aspect_ratio = 16.0 / 9.0;
        let width = 400;
        let height = (width as f64 / aspect_ratio) as u64;

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (width as f64 / height as f64);
        let camera_center = vec3::ORIGIN;

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / width as f64;
        let pixel_delta_v = viewport_v / height as f64;

        let viewport_upper_left =
            camera_center - Vec3::new(0.0, 0.0, focal_length) - (viewport_u + viewport_v) * 0.5;

        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        let mut file = File::create(file_name).unwrap();
        writeln!(file, "P3").unwrap();
        writeln!(file, "{} {}", width, height).unwrap();
        writeln!(file, "255").unwrap();

        (0..height)
            .cartesian_product(0..width)
            .progress_count(width * height)
            .map(|(y, x)| {
                let pixel_center =
                    pixel00_loc + (pixel_delta_u * x as f64) + (pixel_delta_v * y as f64);
                let ray_direction = pixel_center - camera_center;
                let ray = Ray::new(camera_center, ray_direction);
                self.color(&ray)
            })
            .for_each(|color| writeln!(file, "{}", color).unwrap());
    }

    pub fn color(&self, r: &Ray) -> Color {
        let interval = 0.0..f64::MAX;
        if let Some(hit_record) = self.world.hit(r, &interval) {
            Color::from_unit_vector(hit_record.normal).unwrap()
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
