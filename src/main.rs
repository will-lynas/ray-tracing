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
    world::{
        Object,
        World,
    },
};

fn main() {
    let mut world = World::default();

    let ground = Object::new(
        Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0).unwrap(),
        Lambertian::new(PURPLE),
    );
    let center = Object::new(
        Sphere::new(Vec3::new(0.0, 0.0, -1.2), 0.5).unwrap(),
        Lambertian::new(BLUE.mul(0.5).unwrap()),
    );
    let left = Object::new(
        Sphere::new(Vec3::new(-1.0, 0.0, -0.85), 0.5).unwrap(),
        Metal::new(Color::new(0.7, 1.0, 0.7).unwrap(), 0.0).unwrap(),
    );
    let right = Object::new(
        Sphere::new(Vec3::new(0.65, -0.25, -0.7), 0.25).unwrap(),
        Metal::new(Color::new(0.95, 0.95, 0.95).unwrap(), 0.0).unwrap(),
    );
    let front = Object::new(
        Sphere::new(Vec3::new(-0.2, -0.35, -0.6), 0.15).unwrap(),
        Dielectric::new(1.52),
    );
    let front_right = Object::new(
        Sphere::new(Vec3::new(0.35, -0.4, -0.9), 0.1).unwrap(),
        Dielectric::new(1.8),
    );

    world.objects.push(ground);
    world.objects.push(center);
    world.objects.push(left);
    world.objects.push(right);
    world.objects.push(front);
    world.objects.push(front_right);

    let camera = Builder::new(world)
        // ---
        // .draft()
        .build();
    camera.render_to_file("out.ppm");
}
