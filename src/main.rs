use weekend_ray_tracing::camera::Builder;
use weekend_ray_tracing::color::Color;
use weekend_ray_tracing::hittable::Sphere;
use weekend_ray_tracing::material::{Lambertian, Metal};
use weekend_ray_tracing::vec3::Vec3;
use weekend_ray_tracing::world::{Object, World};

fn main() {
    let mut world = World::default();

    let sphere = Object::new(
        Sphere::new(Vec3::new(0.0, 0.1, -1.0), 0.5).unwrap(),
        Metal::new(Color::new(1.0, 0.6, 0.6).unwrap()),
    );
    let ground = Object::new(
        Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0).unwrap(),
        Lambertian::new(Color::new(0.5, 0.5, 0.5).unwrap()),
    );

    world.objects.push(sphere);
    world.objects.push(ground);
    let camera = Builder::new(world).build();

    camera.render_to_file("out.ppm");
}
