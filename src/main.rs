use indicatif::ProgressIterator;
use itertools::Itertools;
use std::fs::File;
use std::io::Write;

fn main() {
    let width = 256;
    let height = 256;
    let mut file = File::create("out.ppm").unwrap();

    writeln!(file, "P3").unwrap();
    writeln!(file, "{} {}", width, height).unwrap();
    writeln!(file, "255").unwrap();

    (0..width)
        .cartesian_product(0..height)
        .progress_count(width * height)
        .for_each(|(x, y)| writeln!(file, "{} {} {}", x, y, 128).unwrap());
}
