use super::utils::Interval;
use glam::*;

pub type Color = Vec3;

fn linear_to_gamma(linear_component: f32) -> f32 {
    f32::sqrt(linear_component)
}

pub fn write_color(color: Color, sample_per_pixel: i32) -> String {
    let interval = Interval::with(0.0, 0.999);

    let scale = 1.0 / sample_per_pixel as f32;
    let mut color = color;

    color.x = linear_to_gamma(color.x * scale);
    color.y = linear_to_gamma(color.y * scale);
    color.z = linear_to_gamma(color.z * scale);

    let r: f32 = interval.clamp(color.x) * 256.0;
    let b: f32 = interval.clamp(color.z) * 256.0;
    let g: f32 = interval.clamp(color.y) * 256.0;
    format!("{} {} {} ", r as i32, g as i32, b as i32)
}
