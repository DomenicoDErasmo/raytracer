use raytracer::{logger::log, ray::Ray, color::Color, vec3::{Point3, Vec3}};

fn ray_color(ray: &Ray) -> Color {
    let unit_direction = ray.direction.unit_vector();
    let alpha = 0.5 * (unit_direction.y + 1.0);
    (1.0 - alpha) * Color {x: 1.0, y: 1.0, z: 1.0} + alpha * Color{x: 0.5, y: 0.7, z: 1.0}
}

fn main() {
    let mut logger = raytracer::logger::Logger {
        stdout: std::io::stdout().lock(),
        stderr: std::io::stderr().lock(),
    };

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = ((image_width as f32 / aspect_ratio) as i32).max(1);

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * image_width as f32 / image_height as f32;
    let camera_center = Point3 {x: 0.0, y: 0.0, z: 0.0};

    // Calculate the vectors across the horizontal and down the vertical viewport edges
    let viewport_u = Vec3 {x: viewport_width, y: 0.0, z: 0.0};
    let viewport_v = Vec3 {x: 0.0, y: -viewport_height, z: 0.0};

    // Calculate the horizontal and vertical delta vectors from pixel to pixel
    let pixel_delta_u = viewport_u / image_width as _;
    let pixel_delta_v = viewport_v / image_height as _;

    // Calculate the location of the upper left pixel
    let viewport_upper_left = camera_center 
        - Vec3 {x: 0.0, y: 0.0, z: focal_length} 
        - viewport_u / 2.0 - viewport_v / 2.0;

    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    log(&mut logger.stdout, format!("P3\n{image_width} {image_height}\n255\n"));
    
    for row in 0..image_height {
        log(
            &mut logger.stderr, 
            format!("\rScanlines remaining: {}", image_height - row),
        );
        for col in 0..image_width {
            let pixel_center = pixel00_loc 
                + (col as f32 * pixel_delta_u) 
                + (row as f32 * pixel_delta_v);
            let ray = Ray { origin: camera_center, direction: pixel_center - camera_center };
            

            let pixel_color = ray_color(&ray);

            raytracer::color::write_color(&mut logger.stdout, pixel_color);
        }
    }
}
