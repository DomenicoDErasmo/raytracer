use crate::interval::Interval;

pub use super::vec::Vec3 as Color;

fn color_to_u8(float: f32, interval: &Interval) -> u8 {
    (((std::u8::MAX as f32 + 1.0) - std::f32::EPSILON) * interval.clamp(float)) as _
}

pub fn write_color(stream: &mut impl std::io::Write, pixel_color: Color, samples_per_pixel: i32) {
    let mut red = pixel_color.x;
    let mut green = pixel_color.y;
    let mut blue = pixel_color.z;

    let scale = 1.0 / samples_per_pixel as f32;
    red *= scale;
    green *= scale;
    blue *= scale;

    let intensity = Interval {min: 0.0, max: 1.0 - f32::EPSILON};

    super::logger::log(
        stream, 
        format!("{} {} {}\n", 
            color_to_u8(red, &intensity), 
            color_to_u8(green, &intensity), 
            color_to_u8(blue, &intensity)
        )
    );
}