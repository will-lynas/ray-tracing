use indicatif::ProgressIterator;
use itertools::Itertools;
use std::fs::File;
use std::io::Write;

use weekend_ray_tracing::color::Color;

fn main() {
    let width = 256;
    let height = 256;
    let mut file = File::create("out.ppm").unwrap();

    writeln!(file, "P3").unwrap();
    writeln!(file, "{} {}", width, height).unwrap();
    writeln!(file, "255").unwrap();

    (0..height)
        .cartesian_product(0..width)
        .progress_count(width * height)
        .map(|(y, x)| Color::new(x as f64 / width as f64, y as f64 / height as f64, 0.5).unwrap())
        .for_each(|color| writeln!(file, "{}", color).unwrap());
}
