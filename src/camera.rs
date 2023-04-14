// depedency
use crate::vec3::Vec3;

// aliassing
type Point3 = Vec3;

pub struct Camera {
	ray: Vec<Vec3>,
	origin: Point3,
	heigth: usize,
	width: usize,
	depth: f64,
	fov: f64,
	depth_unit_vector: Vec3,
	width_unit_vector: Vec3,
	heigth_unit_vector: Vec3,
}

impl Camera {

	/**
	 * create and return a new camera
	 */
	pub fn new (heigth: usize, width: usize, fov: f64, origin: Point3) -> Self {
		let mut camera = Camera { ray: Vec::with_capacity(width * heigth), origin, heigth, width, fov,
			depth_unit_vector: Vec3::new(1.0, 0.0, 0.0), width_unit_vector: Vec3::new(0.0, 0.0, 1.0),
			heigth_unit_vector: Vec3::new(0.0, 1.0, 0.0), depth: (width as f64 / 2.0) / (fov.to_radians() / 2.0).tan() };
		camera.update_ray();
		camera
	}

	/**
	 * return a copy of the ray at index index of camera
	 */
	pub fn get_ray(&self, index: usize) -> Vec3 {
		// eprintln!("inside {}", index);
		self.ray[index]
	}

	/**
	 * update camera size and ray
	 */
	pub fn update_size(&mut self, new_heigth: usize, new_width: usize, new_fov: f64) {
		if new_heigth == self.heigth && new_width == self.width {return;}
		self.heigth = new_heigth;
		self.width = new_width;
		self.fov = new_fov;
		self.depth = (self.width as f64 / 2.0) / (self.fov.to_radians() / 2.0).tan();
		self.update_ray();
	}

	/**
	 * update camera ray
	 */
	pub fn update_ray(&mut self) {
		self.ray.clear();
		for h in 0..self.heigth {
			for w in 0..self.width {
				//eprintln!("correct {}", (w * h + w) as usize);
				self.ray.push((Vec3::new(self.depth, ((self.heigth as f64 / 2.0) as f64 - h as f64) as f64, (w as f64 - (self.width / 2) as f64) as f64) - self.origin).normalized());
			}
		}
	}

	pub fn fov(&self) -> f64 {
		self.fov
	}
}