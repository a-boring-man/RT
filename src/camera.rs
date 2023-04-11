use crate::vec3::Vec3;

type Point3 = Vec3;

pub struct Camera {
	ray: [Vec3],
	origin: Point3,
	aspect_ratio: f64,
	heigth: u16,
	width: u16,
	fov: f64,
	depth_unit_vector: Vec3,
	width_unit_vector: Vec3,
	heigth_unit_vector: Vec3,
}

impl Camera {

	pub fn new (heigth: u16, aspect_ratio: f64, fov: f64, origin: Point3) -> Self {
		let mut camera = Camera { ray: [], origin, aspect_ratio, heigth, width: (aspect_ratio * heigth as f64).into(), fov, depth_unit_vector: Vec3::new(1, 0, 0) };
		
	}
}