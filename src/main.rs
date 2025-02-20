use weekend_ray_tracing::{
    camera::Builder,
    color::Color,
    hittable::Sphere,
    material::{
        Dielectric,
        Lambertian,
        Metal,
    },
    vec3::Vec3,
    world::World,
};

fn main() {
    let mut world = World::default();

    // Ground
    world.add(
        Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0).unwrap(),
        Lambertian::new(Color::new(0.48, 0.73, 0.2).unwrap()),
    );

    // Center - Solid Blue
    world.add(
        Sphere::new(Vec3::new(0.0, 0.0, -1.2), 0.5).unwrap(),
        Lambertian::new(Color::new(0.1, 0.2, 0.5).unwrap()),
    );

    // Left - Glass sphere
    world.add(
        Sphere::new(Vec3::new(-1.0, 0.0, -0.85), 0.5).unwrap(),
        Dielectric::new(1.5),
    );

    // Left - Inside of glass sphere
    world.add(
        Sphere::new(Vec3::new(-1.0, 0.0, -0.85), 0.4).unwrap(),
        Dielectric::new(1.0 / 1.5),
    );

    // Right - Metal
    world.add(
        Sphere::new(Vec3::new(0.65, -0.25, -0.7), 0.25).unwrap(),
        Metal::new(Color::new(0.95, 0.95, 0.95).unwrap(), 0.0).unwrap(),
    );

    let camera = Builder::new(world)
        .look_from(Vec3::new(-2.0, 2.0, 1.0))
        .look_at(Vec3::new(0.0, 0.0, -1.0))
        .vertical_fov(30.0)
        .defocus_angle(0.5)
        .focus_dist(3.6)
        // .draft()
        .build();
    camera.render_to_file("out.ppm");
}
