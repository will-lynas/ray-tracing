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
    world.add(
        Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0).unwrap(),
        Lambertian::new(Color::new(0.48, 0.73, 0.2).unwrap()),
    );
    world.add(
        Sphere::new(Vec3::new(0.0, 0.0, -1.2), 0.5).unwrap(),
        Lambertian::new(Color::new(0.1, 0.2, 0.5).unwrap()),
    );
    world.add(
        Sphere::new(Vec3::new(-1.0, 0.0, -0.85), 0.5).unwrap(),
        Dielectric::new(1.5),
    );
    world.add(
        Sphere::new(Vec3::new(-1.0, 0.0, -0.85), 0.4).unwrap(),
        Dielectric::new(1.0 / 1.5),
    );
    world.add(
        Sphere::new(Vec3::new(0.65, -0.25, -0.7), 0.25).unwrap(),
        Metal::new(Color::new(0.95, 0.95, 0.95).unwrap(), 0.0).unwrap(),
    );
    world.add(
        Sphere::new(Vec3::new(-0.5, -0.25, -2.0), 0.25).unwrap(),
        Metal::new(Color::new(0.95, 0.6, 0.6).unwrap(), 0.0).unwrap(),
    );
    world.add(
        Sphere::new(Vec3::new(-0.1, -0.15, -0.35), 0.35).unwrap(),
        Dielectric::new(1.7),
    );
    world.add(
        Sphere::new(Vec3::new(-0.1, -0.15, -0.4), 0.30).unwrap(),
        Dielectric::new(1.0 / 1.7),
    );
    world.add(
        Sphere::new(Vec3::new(-0.6, -0.35, -0.4), 0.15).unwrap(),
        Lambertian::new(Color::new(0.5, 0.0, 0.0).unwrap()),
    );

    let camera = Builder::new(world)
        .look_from(Vec3::new(-2.0, 2.0, 1.0))
        .look_at(Vec3::new(0.0, 0.0, -1.0))
        .vertical_fov(40.0)
        // .draft()
        .build();
    camera.render_to_file("out.ppm");
}
