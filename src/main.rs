use weekend_ray_tracing::camera::CameraBuilder;
use weekend_ray_tracing::sphere::Sphere;
use weekend_ray_tracing::vec3::Vec3;
use weekend_ray_tracing::world::World;

fn main() {
    let mut world = World::default();
    world.objects.push(Box::new(
        Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5).unwrap(),
    ));
    world.objects.push(Box::new(
        Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0).unwrap(),
    ));

    let camera = CameraBuilder::new(world).width(600).build();
    camera.render_to_file("out.ppm");
}
