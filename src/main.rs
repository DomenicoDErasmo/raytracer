use std::f32::consts::PI;

use raytracer::{
    vec::Point3, 
    sphere::Sphere, 
    hittable_list::HittableList, 
    camera::Camera, 
    lambertian::Lambertian,
    color::Color, 
    metal::Metal,
    dielectric::Dielectric,
};


fn main() {
    let mut logger = raytracer::logger::Logger {
        stdout: std::io::stdout().lock(),
        stderr: std::io::stderr().lock(),
    };

    // let mut world = HittableList {objects: vec![],};

    // // Materials
    // let ground = Box::<_>::new(
    //     Lambertian { albedo: Color {x: 0.8, y: 0.8, z: 0.0} }
    // );
    // let center = Box::<_>::new(
    //     Lambertian { albedo: Color {x: 0.1, y: 0.2, z: 0.5} }
    // );
    // let left = Box::<_>::new(
    //     Dielectric {ir: 1.5},
    // );
    // let right = Box::<_>::new(
    //     Metal::new(&Color {x: 0.8, y: 0.6, z: 0.2}, 0.0)
    // );

    // world.add(Box::<Sphere>::new(Sphere {
    //         center: Point3 {x: 0.0, y: -100.5, z: -1.0},
    //         radius: 100.0,
    //         material: ground.clone(),
    //     })
    // );
    // world.add(Box::<Sphere>::new(Sphere {
    //         center: Point3 {x: 0.0, y: 0.0, z: -1.0},
    //         radius: 0.5,
    //         material: center.clone(),
    //     })
    // );
    // world.add(Box::<Sphere>::new(Sphere {
    //         center: Point3 {x: -1.0, y: 0.0, z: -1.0},
    //         radius: 0.5,
    //         material: left.clone(),
    //     })
    // );
    // world.add(Box::<Sphere>::new(Sphere {
    //         center: Point3 {x: -1.0, y: 0.0, z: -1.0},
    //         radius: -0.4,
    //         material: left.clone(),
    //     })
    // );
    // world.add(Box::<Sphere>::new(Sphere {
    //         center: Point3 {x: 1.0, y: 0.0, z: -1.0},
    //         radius: 0.5,
    //         material: right.clone(),
    //     })
    // );

    let R = (PI / 4.0).cos();
    let mut world = HittableList {objects: vec![]};
    let material_left = Box::<_>::new(Lambertian {albedo: 
        Color {x: 0.0, y: 0.0, z: 1.0}}
    );
    let material_right = Box::<_>::new(Lambertian {albedo: 
        Color {x: 1.0, y: 0.0, z: 0.0}}
    );

    world.add(Box::<_>::new(
        Sphere {center: Point3 {x: -R, y: 0.0, z: -1.0}, radius: R, material: material_left}
    ));
    world.add(Box::<_>::new(
        Sphere {center: Point3 {x: R, y: 0.0, z: -1.0}, radius: R, material: material_right}
    ));

    let mut camera = Camera::default();
    camera.render(&mut logger, &mut world);
}
