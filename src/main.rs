use weekend_ray_tracing::camera::CameraBuilder;
use weekend_ray_tracing::color::GREY;
use weekend_ray_tracing::hittable::{Hittable, Sphere};
use weekend_ray_tracing::material::{Lambertian, Material, Uniform};
use weekend_ray_tracing::vec3::Vec3;
use weekend_ray_tracing::world::{Object, World};

fn main() {
    let mut world = World::default();
    world.objects.push(Object {
        hittable: Hittable::Sphere(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5).unwrap()),
        material: Material::Uniform(Uniform::new(GREY)),
    });
    world.objects.push(Object {
        hittable: Hittable::Sphere(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0).unwrap()),
        material: Material::Lambertian(Lambertian::new(GREY)),
    });

    let camera = CameraBuilder::new(world)
        .width(600)
        .samples_per_pixel(100)
        .build();
    camera.render_to_file("out.ppm");
}
