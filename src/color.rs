// dependency list
use crate::vec3::Vec3;

// aliassing
type Color = Vec3;

/**
 * the debuging background color function super usefull to check camera changes
 */
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
