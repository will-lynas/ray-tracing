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
        Dielectric::new(1.52),
    );
    let right = Object::new(
        Sphere::new(Vec3::new(0.65, -0.25, -0.7), 0.25).unwrap(),
        Metal::new(Color::new(0.9, 0.9, 0.9).unwrap(), 0.0).unwrap(),
    );

    world.objects.push(ground);
    world.objects.push(center);
    world.objects.push(left);
    world.objects.push(right);

    let camera = Builder::new(world)
        // ---
        // .draft()
        .build();
    camera.render_to_file("out.ppm");
}
