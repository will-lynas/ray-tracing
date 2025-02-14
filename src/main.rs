fn main() {
    let width = 256;
    let height = 256;
    println!("P3");
    println!("{} {}", width, height);
    println!("255");
    for x in 0..width {
        for y in 0..height {
            println!("{} {} {}", x, y, 128);
        }
    }
}
