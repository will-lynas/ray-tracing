use weekend_ray_tracing::{
    camera::Builder,
    color::Color,
    hittable::Sphere,
    material::{
        Lambertian,
        Metal,
    },
    vec3::Vec3,
    world::{
        Object,
        World,
    },
};

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
