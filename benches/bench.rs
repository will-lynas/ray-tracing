use criterion::{
    criterion_group,
    criterion_main,
    Criterion,
};
use glam::Vec3A as Vec3;
use ray_tracing::{
    camera::{
        Builder,
        Camera,
    },
    color::Color,
    hittable::Sphere,
    material::Lambertian,
    world::World,
};

fn gen_camera() -> Camera {
    let mut world = World::default();
    world.add(
        Sphere::new_static(Vec3::new(0.0, -100.5, -1.0), 100.0),
        Lambertian::new(Color::new(0.48, 0.73, 0.2)),
    );
    world.add(
        Sphere::new_static(Vec3::new(0.0, 0.0, -1.2), 0.5),
        Lambertian::new(Color::new(0.1, 0.2, 0.5)),
    );
    Builder::new(world)
        .samples_per_pixel(10)
        .width(100)
        .quiet(true)
        .build()
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let camera = gen_camera();
    c.bench_function("render", |b| b.iter(|| camera.render()));
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
