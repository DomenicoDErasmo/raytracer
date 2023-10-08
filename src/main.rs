use raytracer::{
    vec::Point3, 
    sphere::Sphere, 
    hittable_list::HittableList, 
    camera::Camera, 
    lambertian::Lambertian,
    color::Color, 
    metal::Metal,
    dielectric::Dielectric, util::random_double,
};


fn main() {
    let mut logger = raytracer::logger::Logger {
        stdout: std::io::stdout().lock(),
        stderr: std::io::stderr().lock(),
    };

    let mut world = HittableList {objects: vec![],};

    let ground_material = Box::<_>::new(
        Lambertian {albedo: Color {x: 0.5, y: 0.5, z: 0.5,}}
    );
    world.add(Box::<_>::new(Sphere {
        center: Point3 {x: 0.0, y: -1000.0, z: 0.0},
        radius: 1000.0,
        material: ground_material,
    }));

    for a in -11..11 {
        for b in -11..11 {
            let material_rng = random_double(None, None);
            let center = Point3 {
                x: a as f32 + 0.9 * random_double(None, None),
                y: 0.2,
                z: b as f32 + 0.9 * random_double(None, None),
            };

            if (center - Point3 {x: 4.0, y: 0.2, z: 0.0}).length() > 0.9 {
                choose_material_from_rng(&mut world, material_rng, &center);
            }
        }
    }

    let big_dielectric_sphere_material = Box::<_>::new(Dielectric {ir: 1.5});
    world.add(Box::<_>::new(Sphere {
        center: Point3 {x: 0.0, y: 1.0, z: 0.0}, 
        radius: 1.0, 
        material: big_dielectric_sphere_material
    }));

    let big_lambertian_sphere_material = Box::<_>::new(
        Lambertian {albedo: Color {x: 0.4, y: 0.2, z: 0.1}}
    );
    world.add(Box::<_>::new(Sphere {
        center: Point3 {x: -4.0, y: 1.0, z: 0.0}, 
        radius: 1.0, 
        material: big_lambertian_sphere_material
    }));

    let big_metal_sphere_material = Box::<_>::new(
        Metal {albedo: Color {x: 0.7, y: 0.6, z: 0.5}, fuzz: 0.0}
    );
    world.add(Box::<_>::new(Sphere {
        center: Point3 {x: 4.0, y: 1.0, z: 0.0}, 
        radius: 1.0, 
        material: big_metal_sphere_material
    }));

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
        world.add(Box::<_>::new(Sphere {
            center: *center, 
            radius: 0.2, 
            material: sphere_material
        }));
    } else if material_rng < 0.95 {
        let albedo = Color::random(Some(0.5), Some(1.0));
        let fuzz = random_double(Some(0.0), Some(0.5));
        let sphere_material = Box::<_>::new(Metal {albedo, fuzz});
        world.add(Box::<_>::new(Sphere {
            center: *center, 
            radius: 0.2, 
            material: sphere_material
        }));
    } else {
        let sphere_material = Box::<_>::new(Dielectric {ir: 1.5});
        world.add(Box::<_>::new(Sphere {
            center: *center, 
            radius: 0.2, 
            material: sphere_material
        }));
    }
}
