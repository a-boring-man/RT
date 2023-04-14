use crate::vec3::Vec3;

type Point3 = Vec3;

pub struct Camera {
	ray: Vec<Vec3>,
	origin: Point3,
	heigth: usize,
	width: usize,
	depth: f64;
	fov: f64,
	depth_unit_vector: Vec3,
	width_unit_vector: Vec3,
	heigth_unit_vector: Vec3,
}

impl Camera {

	pub fn new (heigth: usize, width: usize, fov: f64, origin: Point3) -> Self {
		let mut camera = Camera { ray: Vec::with_capacity(width * heigth), origin, heigth, width, fov,
			depth_unit_vector: Vec3::new(1.0, 0.0, 0.0), width_unit_vector: Vec3::new(0.0, 0.0, 1.0),
			heigth_unit_vector: Vec3::new(0.0, 1.0, 0.0) };
		depth: f64 = (width as f64 / 2.0) / (fov / 2.0).tan();
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

	pub fn update_size(&mut self, new_heigth: usize, new_width: usize) {
		if new_heigth == self.heigth && new_width == self.width {return;}
		self.heigth = new_heigth;
		self.width = new_width;
	}
}