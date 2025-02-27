use criterion::{
    criterion_group,
    criterion_main,
    Criterion,
};
use glam::Vec3A as Vec3;
use ray_tracing::{
    bvh_node::BvhNode,
    camera::{
        Builder,
        Camera,
    },
    color::Color,
    hittable::{
        HittableList,
        Sphere,
    },
    material::Lambertian,
};

fn gen_camera() -> Camera {
    let mut world = HittableList::default();
    world.add(Box::new(Sphere::new_static(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Lambertian::new(Color::new(0.48, 0.73, 0.2)),
    )));
    world.add(Box::new(Sphere::new_static(
        Vec3::new(0.0, 0.0, -1.2),
        0.5,
        Lambertian::new(Color::new(0.1, 0.2, 0.5)),
    )));

    let bvh = BvhNode::from_list(world);
    Builder::new(bvh)
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
