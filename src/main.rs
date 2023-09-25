use raytracer::{vec::Point3, sphere::Sphere, hittable_list::HittableList, camera::Camera};


fn main() {
    let mut logger = raytracer::logger::Logger {
        stdout: std::io::stdout().lock(),
        stderr: std::io::stderr().lock(),
    };

    let mut world = HittableList {objects: vec![],};
    world.add(
        Box::<Sphere>::new(Sphere {
            center: Point3 {x: 0.0, y: 0.0, z: -1.0},
            radius: 0.5
        })
    );
    world.add(
        Box::<Sphere>::new(Sphere {
            center: Point3 {x: 0.0, y: -100.5, z: -1.0},
            radius: 100.0,
        })
    );

    let mut camera = Camera::default();
    camera.render(&mut logger, &mut world);
}
