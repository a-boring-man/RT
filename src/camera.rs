use crate::vec3::Vec3;

type Point3 = Vec3;

pub struct Camera<const SIZE: usize> {
	ray: [Vec3; SIZE],
	origin: Point3,
	heigth: usize,
	width: usize,
	fov: f64,
	depth_unit_vector: Vec3,
	width_unit_vector: Vec3,
	heigth_unit_vector: Vec3,
}

impl<const SIZE: usize> Camera<SIZE> {

	pub fn new (heigth: usize, width: usize, fov: f64, origin: Point3) -> Self {
		let mut camera = Camera::<SIZE> { ray: [Vec3::default(); SIZE], origin, heigth, width, fov,
			depth_unit_vector: Vec3::new(1.0, 0.0, 0.0), width_unit_vector: Vec3::new(0.0, 0.0, 1.0),
			heigth_unit_vector: Vec3::new(0.0, 1.0, 0.0) };
		let depth: f64 = (width as f64 / 2.0) / (fov / 2.0).tan();
		for h in 0..heigth {
			for w in 0..width {
				// eprintln!("correct {}", (w * h + w) as usize);
				camera.ray[(w * h + w) as usize] = (Vec3::new(depth, ((heigth as f64 / 2.0) as f64 - h as f64) as f64, (w as f64 - (width / 2) as f64) as f64) - origin).normalized();
			}
		}
		camera
	}

	pub fn get_ray(&self, index: usize) -> Vec3 {
		// eprintln!("inside {}", index);
		self.ray[index].clone()
	}
}