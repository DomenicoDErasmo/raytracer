use std::collections::VecDeque;

use raytracer::{
    vec::{Point3, Vec3}, 
    sphere::Sphere, 
    hittable_list::HittableList, 
    camera::Camera, 
    lambertian::Lambertian,
    color::Color, 
    metal::Metal,
    dielectric::Dielectric, util::random_float, aabb::AABB, bvh::BVHNode,
};


fn main() {
    let mut logger = raytracer::logger::Logger {
        stdout: std::io::stdout().lock(),
        stderr: std::io::stderr().lock(),
    };

    let mut world = HittableList {objects: vec![], bounding_box: AABB::default()};

    let ground_material = Box::<_>::new(
        Lambertian {albedo: Color {x: 0.5, y: 0.5, z: 0.5,}}
    );
    let ground_sphere = Sphere::make_stationary_sphere(
        Point3 {x: 0.0, y: -1000.0, z: 0.0}, 
        -1000.0, 
        ground_material,
    );
    world.add(Box::<_>::new(ground_sphere));

    for a in -11..11 {
        for b in -11..11 {
            let material_rng = random_float(None, None);
            let center = Point3 {
                x: a as f32 + 0.9 * random_float(None, None),
                y: 0.2,
                z: b as f32 + 0.9 * random_float(None, None),
            };

            if (center - Point3 {x: 4.0, y: 0.2, z: 0.0}).length() > 0.9 {
                choose_material_from_rng(&mut world, material_rng, &center);
            }
        }
    }

    let big_dielectric_sphere_material = Box::<_>::new(Dielectric {ir: 1.5});
    let big_dielectric_sphere = Sphere::make_stationary_sphere(
        Point3 {x: 0.0, y: 1.0, z: 0.0}, 
        1.0, 
        big_dielectric_sphere_material
    );
    world.add(Box::<_>::new(big_dielectric_sphere));

    let big_lambertian_sphere_material = Box::<_>::new(
        Lambertian {albedo: Color {x: 0.4, y: 0.2, z: 0.1}}
    );
    let big_lambertian_sphere = Sphere::make_stationary_sphere(
        Point3 {x: -4.0, y: 1.0, z: 0.0},
        1.0, 
        big_lambertian_sphere_material
    );
    world.add(Box::<_>::new(big_lambertian_sphere));

    let big_metal_sphere_material = Box::<_>::new(
        Metal {albedo: Color {x: 0.7, y: 0.6, z: 0.5}, fuzz: 0.0}
    );
    let big_metal_sphere = Sphere::make_stationary_sphere(
        Point3 {x: 4.0, y: 1.0, z: 0.0}, 
        1.0, 
        big_metal_sphere_material
    );
    world.add(Box::<_>::new(big_metal_sphere));
    let mut world_vec_deque = VecDeque::from(world.objects);
    let length = world_vec_deque.len();
    let bvh_objects = BVHNode::from_objects_and_times(
        &mut world_vec_deque, 
        0, 
        length,
    );
    let bounding_box = bvh_objects.bounding_box;
    world = HittableList {
        objects: vec![Box::<_>::new(bvh_objects)],
        bounding_box,
    };

    let mut camera = Camera::default();
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;
    camera.vertical_field_of_view = 20.0;
    camera.look_from = Point3 {x: 13.0, y: 2.0, z: 3.0};
    camera.look_at = Point3 {x: 0.0, y: 0.0, z: 0.0};
    camera.defocus_angle = 0.6;
    camera.focus_distance = 10.0;
    camera.render(&mut logger, &mut world);
}

fn choose_material_from_rng(world: &mut HittableList, material_rng: f32, center: &Point3) {
    if material_rng < 0.8 {
        let albedo = Color::random(None, None);
        let sphere_material = Box::<_>::new(Lambertian {albedo});
        let moving_lambertian_sphere = Sphere::make_moving_sphere(
            *center, 
            *center + Vec3 {
                x: 0.0,
                y: random_float(Some(0.0), Some(0.5)),
                z: 0.0,
            }, 
            0.2, 
            sphere_material
        );
        world.add(Box::<_>::new(moving_lambertian_sphere));
    } else if material_rng < 0.95 {
        let albedo = Color::random(Some(0.5), Some(1.0));
        let fuzz = random_float(Some(0.0), Some(0.5));
        let sphere_material = Box::<_>::new(Metal {albedo, fuzz});
        let stationary_metal_sphere = Sphere::make_stationary_sphere(
            *center, 
            0.2, 
            sphere_material,
        );
        world.add(Box::<_>::new(stationary_metal_sphere));
    } else {
        let sphere_material = Box::<_>::new(Dielectric {ir: 1.5});
        let stationary_dielectric_sphere = Sphere::make_stationary_sphere(
            *center, 
            0.2, 
            sphere_material,
        );
        world.add(Box::<_>::new(stationary_dielectric_sphere));
    }
}
