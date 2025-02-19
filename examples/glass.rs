use weekend_ray_tracing::{
    camera::Builder,
    color::{
        Color,
        BLUE,
        PURPLE,
    },
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
        Lambertian::new(PURPLE),
    );

    // Center - Solid Blue
    world.add(
        Sphere::new(Vec3::new(0.0, 0.0, -1.2), 0.5).unwrap(),
        Lambertian::new(BLUE.mul(0.5).unwrap()),
    );

    // Left - Green Metal
    world.add(
        Sphere::new(Vec3::new(-1.0, 0.0, -0.85), 0.5).unwrap(),
        Metal::new(Color::new(0.7, 1.0, 0.7).unwrap(), 0.0).unwrap(),
    );

    // Right - Metal
    world.add(
        Sphere::new(Vec3::new(0.65, -0.25, -0.7), 0.25).unwrap(),
        Metal::new(Color::new(0.95, 0.95, 0.95).unwrap(), 0.0).unwrap(),
    );

    // Front left - Medium Glass
    world.add(
        Sphere::new(Vec3::new(-0.2, -0.35, -0.6), 0.15).unwrap(),
        Dielectric::new(1.52),
    );

    // Front right - Small Glass
    world.add(
        Sphere::new(Vec3::new(0.35, -0.4, -0.9), 0.1).unwrap(),
        Dielectric::new(1.8),
    );

    let camera = Builder::new(world).build();
    camera.render_to_file("out.ppm");
}
