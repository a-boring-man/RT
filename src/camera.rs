use crate::vec3::Vec3;

type Point3 = Vec3;

pub struct Camera<const size: usize> {
	ray: [Vec3; size],
	origin: Point3,
	heigth: u16,
	width: u16,
	fov: f64,
	depth_unit_vector: Vec3,
	width_unit_vector: Vec3,
	heigth_unit_vector: Vec3,
}

impl<const size: usize> Camera<size> {

	pub fn new (heigth: u16, width: u16, fov: f64, origin: Point3) -> Self {
		let mut camera = Camera::<size> { ray: [Vec3::default(); size], origin, heigth, width, fov,
			depth_unit_vector: Vec3::new(1.0, 0.0, 0.0), width_unit_vector: Vec3::new(0.0, 0.0, 1.0),
			heigth_unit_vector: Vec3::new(0.0, 1.0, 0.0) };
		let depth: f64 = (width as f64 / 2.0) / (fov / 2.0).tan();
		for h in 0..heigth {
			for w in 0..width {
				camera.ray[w * h + w] = ;
			}
		}
	}
}