use weekend_ray_tracing::camera::Builder;
use weekend_ray_tracing::color::{Color, BLUE, PURPLE};
use weekend_ray_tracing::hittable::Sphere;
use weekend_ray_tracing::material::{Lambertian, Metal};
use weekend_ray_tracing::vec3::Vec3;
use weekend_ray_tracing::world::{Object, World};

fn main() {
    let mut world = World::default();

    let ground = Object::new(
        Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0).unwrap(),
        Lambertian::new(PURPLE.mul(0.5).unwrap()),
    );
    let center = Object::new(
        Sphere::new(Vec3::new(0.0, 0.0, -1.2), 0.5).unwrap(),
        Lambertian::new(BLUE.mul(0.5).unwrap()),
    );
    let left = Object::new(
        Sphere::new(Vec3::new(-1.0, 0.0, -0.85), 0.5).unwrap(),
        Metal::new(Color::new(0.7, 1.0, 0.7).unwrap()),
    );
    let right = Object::new(
        Sphere::new(Vec3::new(0.65, -0.25, -0.7), 0.25).unwrap(),
        Metal::new(Color::new(0.9, 0.9, 0.9).unwrap()),
    );

    world.objects.push(ground);
    world.objects.push(center);
    world.objects.push(left);
    world.objects.push(right);

    let camera = Builder::new(world)
        .width(1000)
        .samples_per_pixel(100)
        .max_depth(100)
        .build();

    camera.render_to_file("out.ppm");
}
