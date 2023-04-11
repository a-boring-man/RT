use crate::vec3::Vec3;

type Point3 = Vec3;

pub struct Camera {
	ray: [Vec3],
	origin: Point3,
	heigth: u16,
	width: u16,
	fov: f64,
	depth_unit_vector: Vec3,
	width_unit_vector: Vec3,
	heigth_unit_vector: Vec3,
}

impl Camera {

	pub fn new (heigth: u16, width: u16, fov: f64, origin: Point3) -> Self {
		let mut camera = Camera { ray: [], origin, heigth, width, fov,
			depth_unit_vector: Vec3::new(1, 0, 0), width_unit_vector: Vec3::new(0, 0, 1),
			heigth_unit_vector: Vec3::new(0, 1, 0) };
		let depth: f64 = (width / 2.0) / (fov / 2.0).tan();
		for h in 0..heigth {
			for w in 0..width {
				let 
			}
		}
	}
}