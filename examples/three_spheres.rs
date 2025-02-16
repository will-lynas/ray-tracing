use weekend_ray_tracing::{
    camera::Builder,
    color::{
        Color,
        BLUE,
        PURPLE,
    },
    hittable::Sphere,
    material::{
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
        Lambertian::new(PURPLE.mul(0.5).unwrap()),
    );

    // Center
    world.add(
        Sphere::new(Vec3::new(0.0, 0.0, -1.2), 0.5).unwrap(),
        Lambertian::new(BLUE.mul(0.5).unwrap()),
    );

    // Left
    world.add(
        Sphere::new(Vec3::new(-1.0, 0.0, -0.85), 0.5).unwrap(),
        Metal::new(Color::new(0.7, 1.0, 0.7).unwrap(), 0.0).unwrap(),
    );

    // Right
    world.add(
        Sphere::new(Vec3::new(0.65, -0.25, -0.7), 0.25).unwrap(),
        Metal::new(Color::new(0.9, 0.9, 0.9).unwrap(), 0.0).unwrap(),
    );

    let camera = Builder::new(world).build();
    camera.render_to_file("out.ppm");
}
