pub use super::vec::Vec3 as Color;

fn color_to_u8(float: f32) -> u8 {
    (((std::u8::MAX as f32 + 1.0) - std::f32::EPSILON) * float) as _
}

pub fn write_color(stream: &mut impl std::io::Write, pixel_color: Color) {
    super::logger::log(
        stream, 
        format!("{} {} {}\n", 
            color_to_u8(pixel_color.x), 
            color_to_u8(pixel_color.y), 
            color_to_u8(pixel_color.z)
        )
    );
}