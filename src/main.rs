use std::fs::File;
use std::io::Write;

fn main() {
    let width = 256;
    let height = 256;
    let mut file = File::create("out.ppm").unwrap();

    writeln!(file, "P3").unwrap();
    writeln!(file, "{} {}", width, height).unwrap();
    writeln!(file, "255").unwrap();

    for x in 0..width {
        for y in 0..height {
            writeln!(file, "{} {} {}", x, y, 128).unwrap();
        }
    }
}
