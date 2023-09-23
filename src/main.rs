use raytracer::logger::log;

fn color_to_u8(float: f32) -> u8 {
    (((std::u8::MAX as f32 + 1.0) - std::f32::EPSILON) * float) as _
}

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

            let int_red = color_to_u8(red);
            let int_green = color_to_u8(green);
            let int_blue = color_to_u8(blue);

            log (
                &mut logger.stdout,
                format!("{int_red} {int_green} {int_blue}\n"),
            );
        }
    }
}
