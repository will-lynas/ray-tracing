use glam::Vec3A as Vec3;
use weekend_ray_tracing::{
    camera::Builder,
    color::Color,
    hittable::Sphere,
    material::{
        Dielectric,
        Lambertian,
        Metal,
    },
    world::World,
};

fn main() {
    let mut world = World::default();
    world.add(
        Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0),
        Lambertian::new(Color::new(0.48, 0.73, 0.2)),
    );
    world.add(
        Sphere::new(Vec3::new(0.0, 0.0, -1.2), 0.5),
        Lambertian::new(Color::new(0.1, 0.2, 0.5)),
    );
    world.add(
        Sphere::new(Vec3::new(-1.0, 0.0, -0.85), 0.5),
        Dielectric::new(1.5),
    );
    world.add(
        Sphere::new(Vec3::new(-1.0, 0.0, -0.85), 0.4),
        Dielectric::new(1.0 / 1.5),
    );
    world.add(
        Sphere::new(Vec3::new(0.65, -0.25, -0.7), 0.25),
        Metal::new(Color::new(0.95, 0.95, 0.95), 0.0),
    );
    world.add(
        Sphere::new(Vec3::new(-0.5, -0.25, -2.0), 0.25),
        Metal::new(Color::new(0.95, 0.6, 0.6), 0.0),
    );
    world.add(
        Sphere::new(Vec3::new(-0.1, -0.15, -0.35), 0.35),
        Dielectric::new(1.7),
    );
    world.add(
        Sphere::new(Vec3::new(-0.1, -0.15, -0.4), 0.30),
        Dielectric::new(1.0 / 1.7),
    );
    world.add(
        Sphere::new(Vec3::new(-0.6, -0.35, -0.4), 0.15),
        Lambertian::new(Color::new(0.5, 0.0, 0.0)),
    );

    let camera = Builder::new(world)
        .look_from(Vec3::new(-2.0, 2.0, 1.0))
        .look_at(Vec3::new(0.0, 0.0, -1.0))
        .vertical_fov(40.0)
        .draft()
        .build();
    camera.render_to_file();
}
