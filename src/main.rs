use raytracer::logger::log;

fn main() {
    let image_width = 256;
    let image_height = 256;

    let mut logger = raytracer::logger::Logger {
        stdout: std::io::stdout().lock(),
        stderr: std::io::stderr().lock(),
    };

    log(&mut logger.stdout, format!("P3\n{image_width} {image_height}\n255\n"));
    
    for row in 0..image_height {
        log(
            &mut logger.stderr, 
            format!("\rScanlines remaining: {}", image_height - row),
        );
        for col in 0..image_width {
            let red = col as f32 / (image_width - 1) as f32;
            let green = row as f32 / (image_height - 1) as f32;
            let blue = 0.0;

            raytracer::color::write_color(
                &mut logger.stdout, 
                raytracer::vec3::Color {x: red, y: green, z: blue});
        }
    }
}
