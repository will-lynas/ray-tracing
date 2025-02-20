use std::f64::consts::PI;

use weekend_ray_tracing::{
    camera::Builder,
    color::Color,
    hittable::Sphere,
    material::Lambertian,
    vec3::Vec3,
    world::World,
};

fn main() {
    let r = (PI / 4.0).cos();
    let mut world = World::default();
    world.add(
        Sphere::new(Vec3::new(-r, 0.0, -1.0), r).unwrap(),
        Lambertian::new(Color::new(0.0, 0.0, 1.0).unwrap()),
    );
    world.add(
        Sphere::new(Vec3::new(r, 0.0, -1.0), r).unwrap(),
        Lambertian::new(Color::new(1.0, 0.0, 0.0).unwrap()),
    );

    let camera = Builder::new(world)
        // ---
        .draft()
        .build();
    camera.render_to_file("out.ppm");
}
