use raytracer::{
    vec::Point3, 
    sphere::Sphere, 
    hittable_list::HittableList, 
    camera::Camera, 
    lambertian::Lambertian,
    color::Color, metal::Metal,
};


fn main() {
    let mut logger = raytracer::logger::Logger {
        stdout: std::io::stdout().lock(),
        stderr: std::io::stderr().lock(),
    };

    let mut world = HittableList {objects: vec![],};

    // Materials
    let ground = Box::<_>::new(
        Lambertian { albedo: Color {x: 0.8, y: 0.8, z: 0.0} }
    );
    let center = Box::<_>::new(
        Lambertian { albedo: Color {x: 0.7, y: 0.3, z: 0.3} }
    );
    let left = Box::<_>::new(
        Metal { albedo: Color {x: 0.8, y: 0.8, z: 0.8 } }
    );
    let right = Box::<_>::new(
        Metal { albedo: Color {x: 0.8, y: 0.6, z: 0.2 } }
    );

    world.add(Box::<Sphere>::new(Sphere {
            center: Point3 {x: 0.0, y: -100.5, z: -1.0},
            radius: 100.0,
            material: ground,
        })
    );
    world.add(Box::<Sphere>::new(Sphere {
            center: Point3 {x: 0.0, y: 0.0, z: -1.0},
            radius: 0.5,
            material: center,
        })
    );
    world.add(Box::<Sphere>::new(Sphere {
            center: Point3 {x: -1.0, y: 0.0, z: -1.0},
            radius: 0.5,
            material: left,
        })
    );
    world.add(Box::<Sphere>::new(Sphere {
            center: Point3 {x: 1.0, y: 0.0, z: -1.0},
            radius: 0.5,
            material: right,
        })
    );

    let mut camera = Camera::default();
    camera.render(&mut logger, &mut world);
}
