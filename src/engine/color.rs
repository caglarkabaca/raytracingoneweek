use super::utils::Interval;
use glam::*;

pub type Color = Vec3;

pub fn write_color(color: Color, sample_per_pixel: i32) -> String {
    let interval = Interval::with(0.0, 0.999);

    let scale = 1.0 / sample_per_pixel as f32;

    let r: f32 = interval.clamp(color.x * scale) * 256.0;
    let b: f32 = interval.clamp(color.z * scale) * 256.0;
    let g: f32 = interval.clamp(color.y * scale) * 256.0;
    format!("{} {} {} ", r as i32, g as i32, b as i32)
}
