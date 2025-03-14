use glam::Vec3A as Vec3;
use ray_tracing::{
    camera::Builder,
    color::Color,
    extension_traits::Vec3Ext,
    hittable::{
        BvhNode,
        HittableList,
        Sphere,
    },
    material::{
        Dielectric,
        Lambertian,
        Metal,
    },
    rng::random_range,
    texture::{
        CheckerTexture,
        SolidColor,
    },
};

pub fn many_spheres() -> Builder {
    let mut world = HittableList::default();

    let ground_material = Lambertian::new(CheckerTexture::new(
        SolidColor::new(0.1, 0.01, 0.4),
        SolidColor::new(0.9, 0.9, 0.9),
        0.3,
    ));
    world.add(Sphere::new_static(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    for a in -11..11 {
        for b in -11..11 {
            let center = Vec3::new(
                a as f32 + 0.9 * fastrand::f32(),
                0.2,
                b as f32 + 0.9 * fastrand::f32(),
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() <= 0.9 {
                continue;
            }

            let radius = 0.2;
            let choose_mat = fastrand::f32();
            if choose_mat < 0.50 {
                let albedo = Color(Vec3::random()) * Color(Vec3::random());
                let sphere_material = Lambertian::new(SolidColor::new_from_color(albedo));
                world.add(Sphere::new_static(center, radius, sphere_material));
            } else if choose_mat < 0.70 {
                let albedo = Color(Vec3::random_range(&(0.5..1.0)));
                let fuzz = random_range(&(0.0..0.5));
                let sphere_material = Metal::new(albedo, fuzz);
                world.add(Sphere::new_static(center, radius, sphere_material));
            } else {
                let sphere_material = Dielectric::new(1.5);
                world.add(Sphere::new_static(center, radius, sphere_material));
            };
        }
    }

    let material1 = Dielectric::new(1.5);
    world.add(Sphere::new_static(Vec3::new(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = Lambertian::new(SolidColor::new(0.0, 0.9, 0.2));
    world.add(Sphere::new_static(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    ));

    let material3 = Metal::new(Color::new(0.90, 0.90, 1.0), 0.0);
    world.add(Sphere::new_static(Vec3::new(4.0, 1.0, 0.0), 1.0, material3));

    let bvh = BvhNode::from_list(world);

    Builder::new(bvh)
        .width(2000)
        .samples_per_pixel(500)
        .max_depth(50)
        .vertical_fov(20.0)
        .look_from(Vec3::new(13.0, 2.0, 3.0))
        .look_at(Vec3::new(0.0, 0.0, 0.0))
        .vup(Vec3::Y)
        .defocus_angle(0.6)
        .focus_dist(10.0)
}

pub fn many_bouncing_spheres() -> Builder {
    let mut world = HittableList::default();

    let ground_material = Lambertian::new(CheckerTexture::new(
        SolidColor::new(0.1, 0.01, 0.4),
        SolidColor::new(0.9, 0.9, 0.9),
        0.3,
    ));
    world.add(Sphere::new_static(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    for a in -11..11 {
        for b in -11..11 {
            let center = Vec3::new(
                a as f32 + 0.9 * fastrand::f32(),
                0.2,
                b as f32 + 0.9 * fastrand::f32(),
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() <= 0.9 {
                continue;
            }
            let end = if fastrand::f32() < 0.5 {
                center + Vec3::Y * random_range(&(0.0..0.5))
            } else {
                center
            };

            let radius = 0.2;
            let choose_mat = fastrand::f32();
            if choose_mat < 0.50 {
                let albedo = Color(Vec3::random()) * Color(Vec3::random());
                let sphere_material = Lambertian::new(SolidColor::new_from_color(albedo));
                world.add(Sphere::new_start_end(center, end, radius, sphere_material));
            } else if choose_mat < 0.70 {
                let albedo = Color(Vec3::random_range(&(0.5..1.0)));
                let fuzz = random_range(&(0.0..0.5));
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

    let material2 = Lambertian::new(SolidColor::new(0.0, 0.9, 0.2));
    world.add(Sphere::new_static(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    ));

    let material3 = Metal::new(Color::new(0.90, 0.90, 1.0), 0.0);
    world.add(Sphere::new_static(Vec3::new(4.0, 1.0, 0.0), 1.0, material3));

    let bvh = BvhNode::from_list(world);

    Builder::new(bvh)
        .width(2000)
        .samples_per_pixel(500)
        .max_depth(50)
        .vertical_fov(20.0)
        .look_from(Vec3::new(13.0, 2.0, 3.0))
        .look_at(Vec3::new(0.0, 0.0, 0.0))
        .vup(Vec3::Y)
        .defocus_angle(0.6)
        .focus_dist(10.0)
}

pub fn checkered_spheres() -> Builder {
    let mut world = HittableList::default();
    // TODO: we should hold references to textures, instead of duplicating them
    // (Can't even do Clone, because of the dyn)
    let checker = CheckerTexture::new(
        SolidColor::new(0.1, 0.01, 0.4),
        SolidColor::new(0.9, 0.9, 0.9),
        0.3,
    );
    world.add(Sphere::new_static(
        Vec3::new(0.0, -10.0, 0.0),
        10.0,
        Lambertian::new(checker),
    ));
    let checker = CheckerTexture::new(
        SolidColor::new(0.1, 0.01, 0.4),
        SolidColor::new(0.9, 0.9, 0.9),
        0.3,
    );
    world.add(Sphere::new_static(
        Vec3::new(0.0, 10.0, 0.0),
        10.0,
        Lambertian::new(checker),
    ));

    let bvh = BvhNode::from_list(world);

    Builder::new(bvh)
        .width(2000)
        .samples_per_pixel(500)
        .max_depth(50)
        .vertical_fov(20.0)
        .look_from(Vec3::new(13.0, 2.0, 3.0))
        .look_at(Vec3::new(0.0, 0.0, 0.0))
        .vup(Vec3::Y)
        .defocus_angle(0.0)
        .focus_dist(10.0)
}
