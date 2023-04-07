use crate::vec3::Vec3;

type Color = Vec3;

pub fn background_color(v: Vec3) -> Color {
    let mut pixel_color: Color = Color::new(0.0, 0.0, 0.0);

    if v.x().abs() >= v.y().abs() && v.x().abs() >= v.z().abs() {
        pixel_color.set((v.x() + 1.0) / 2.0, 0.0, 0.0);
    }
    if v.y().abs() >= v.x().abs() && v.y().abs() >= v.z().abs() {
        pixel_color.set(0.0, (v.y() + 1.0) / 2.0, 0.0);
    }
    if v.z().abs() >= v.x().abs() && v.z().abs() >= v.y().abs() {
        pixel_color.set(0.0, 0.0, (1.0 * v.z() + 1.0) / 2.0);
    }
    pixel_color
}

pub fn into_color(color: Color) -> u32 {
	(color.b() * 255.0) as u32 | ((color.g() * 255.0) as u32) << 8 | ((color.r() * 255.0) as u32) << 16
}
