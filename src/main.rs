use clap::Parser;
use glam::Vec3A as Vec3;
use ray_tracing::{
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

#[derive(Parser)]
struct Args {
    /// Enable draft mode for faster rendering
    #[arg(short, long)]
    draft: bool,
}

fn main() {
    let args = Args::parse();

    let mut world = World::default();

    // Ground
    world.add(
        Sphere::new_static(Vec3::new(0.0, -100.5, -1.0), 100.0),
        Lambertian::new(Color::new(0.48, 0.73, 0.2)),
    );

    // Sphere solid center blue
    let big_sphere_radius = 0.5;
    let big_sphere_start = Vec3::new(0.0, 0.0, -1.2);
    let end = big_sphere_start + Vec3::Y * big_sphere_radius * 0.6;
    world.add(
        Sphere::new_start_end(big_sphere_start, end, big_sphere_radius),
        Lambertian::new(Color::new(0.1, 0.2, 0.5)),
    );

    // Sphere glass left
    // Outer
    world.add(
        Sphere::new_static(Vec3::new(-1.0, 0.0, -0.85), 0.5),
        Dielectric::new(1.5),
    );
    // Inner
    world.add(
        Sphere::new_static(Vec3::new(-1.0, 0.0, -0.85), 0.4),
        Dielectric::new(1.0 / 1.5),
    );

    // Sphere metal right
    world.add(
        Sphere::new_static(Vec3::new(0.65, -0.25, -0.7), 0.25),
        Metal::new(Color::new(0.95, 0.95, 0.95), 0.0),
    );

    // Sphere metal left red tint
    world.add(
        Sphere::new_static(Vec3::new(-0.5, -0.25, -2.0), 0.25),
        Metal::new(Color::new(0.95, 0.6, 0.6), 0.0),
    );

    // Sphere glass left
    // Outer
    world.add(
        Sphere::new_static(Vec3::new(-0.1, -0.15, -0.35), 0.35),
        Dielectric::new(1.7),
    );
    // Inner
    world.add(
        Sphere::new_static(Vec3::new(-0.1, -0.15, -0.4), 0.30),
        Dielectric::new(1.0 / 1.7),
    );

    // Sphere solid red
    let radius = 0.15;
    let start = Vec3::new(-0.6, -0.35, -0.4);
    let end = start + Vec3::Y * radius * 0.6;
    world.add(
        Sphere::new_start_end(start, end, radius),
        Lambertian::new(Color::new(0.5, 0.0, 0.0)),
    );

    let mut builder = Builder::new(world);
    builder
        .look_from(Vec3::new(-2.0, 2.0, 1.0))
        .look_at(big_sphere_start - Vec3::Y * big_sphere_radius)
        .vertical_fov(30.0);

    if args.draft {
        builder.draft();
    }

    let camera = builder.build();
    camera.render_to_file();
}
