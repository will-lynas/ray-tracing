use clap::Parser;
use glam::Vec3A as Vec3;
use ray_tracing::{
    bvh_node::BvhNode,
    camera::Builder,
    color::Color,
    extension_traits::Vec3Ext,
    hittable::{
        HittableList,
        Sphere,
    },
    material::{
        Dielectric,
        Lambertian,
        Metal,
    },
    rng::ThreadRng,
};

#[derive(Parser)]
struct Args {
    /// Enable draft mode for faster rendering
    #[arg(short, long)]
    draft: bool,
}

fn main() {
    let args = Args::parse();

    let mut world = HittableList::default();

    let ground_material = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    world.add(Sphere::new_static(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    for a in -11..11 {
        for b in -11..11 {
            let center = Vec3::new(
                a as f32 + 0.9 * ThreadRng::random(),
                0.2,
                b as f32 + 0.9 * ThreadRng::random(),
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() <= 0.9 {
                continue;
            }

            let radius = 0.2;
            let choose_mat = ThreadRng::random();
            let end = if ThreadRng::random() < 0.5 {
                center + Vec3::Y * ThreadRng::random_range(&(0.0..0.5))
            } else {
                center
            };
            if choose_mat < 0.70 {
                let albedo = Color(Vec3::random()) * Color(Vec3::random());
                let sphere_material = Lambertian::new(albedo);
                world.add(Sphere::new_start_end(center, end, radius, sphere_material));
            } else if choose_mat < 0.85 {
                let albedo = Color(Vec3::random_range(&(0.5..1.0)));
                let fuzz = ThreadRng::random_range(&(0.0..0.5));
                let sphere_material = Metal::new(albedo, fuzz);
                world.add(Sphere::new_start_end(center, end, radius, sphere_material));
            } else {
                let sphere_material = Dielectric::new(1.5);
                world.add(Sphere::new_start_end(center, end, radius, sphere_material));
            };
        }
    }

    let material1 = Dielectric::new(1.5);
    world.add(Sphere::new_static(Vec3::new(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = Lambertian::new(Color::new(0.0, 0.9, 0.2));
    world.add(Sphere::new_static(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    ));

    let material3 = Metal::new(Color::new(0.90, 0.90, 1.0), 0.0);
    world.add(Sphere::new_static(Vec3::new(4.0, 1.0, 0.0), 1.0, material3));

    let bvh = BvhNode::from_list(world);

    let mut builder = Builder::new(bvh)
        .width(2000)
        .samples_per_pixel(500)
        .max_depth(50)
        .vertical_fov(20.0)
        .look_from(Vec3::new(13.0, 2.0, 3.0))
        .look_at(Vec3::new(0.0, 0.0, 0.0))
        .vup(Vec3::Y)
        .defocus_angle(0.6)
        .focus_dist(10.0);

    if args.draft {
        builder = builder.draft();
    }

    let camera = builder.build();
    camera.render_to_file();
}
