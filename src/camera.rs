// depedency
use crate::vec3::Vec3;
use crate::matrix::Matrix;

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
	x_angle: f64,
	y_angle: f64,
	z_angle: f64,
	omnimatrice: Matrix<f64>,
}

impl Camera {

	/**
	 * create and return a new camera
	 */
	pub fn new (heigth: usize, width: usize, fov: f64, origin: Point3) -> Self {
		let mut camera = Camera { ray: Vec::with_capacity(width * heigth), origin, heigth, width, fov,
			depth: (width as f64 / 2.0) / (fov.to_radians() / 2.0).tan(),
			depth_unit_vector: Vec3::new(1.0, 0.0, 0.0),
			width_unit_vector: Vec3::new(0.0, 0.0, 1.0),
			heigth_unit_vector: Vec3::new(0.0, 1.0, 0.0),
			x_angle : 0.0,
			y_angle : 0.0,
			z_angle : 0.0,
			omnimatrice : Matrix::<f64>::new_identity(3, 3) };
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

	pub fn get_direction(&self) -> Vec3 {
		self.depth_unit_vector
	}

	pub fn get_width(&self) -> Vec3 {
		self.width_unit_vector
	}

	pub fn get_heigth(&self) -> Vec3 {
		self.heigth_unit_vector
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
		if Matrix::<f64>::new_identity(3, 3) == self.omnimatrice.clone() {
			for h in 0..self.heigth {
				for w in 0..self.width {
					//eprintln!("correct {}", (w * h + w) as usize);
					self.ray.push((Vec3::new(self.depth, ((self.heigth as f64 / 2.0) as f64 - h as f64) as f64, (w as f64 - (self.width / 2) as f64) as f64) - self.origin).normalized());
				}
			}
		}
		else {
			for h in 0..self.heigth {
				for w in 0..self.width {
					//eprintln!("correct {}", (w * h + w) as usize);
					self.ray.push(self.omnimatrice.clone() * ((Vec3::new(self.depth, ((self.heigth as f64 / 2.0) as f64 - h as f64) as f64, (w as f64 - (self.width / 2) as f64) as f64) - self.origin).normalized()));
				}
			}
		}
	}

	pub fn fov(&self) -> f64 {
		self.fov
	}

	pub fn get_origin(&self) -> Vec3 {
		self.origin
	}

	pub fn set_origin(&mut self, new: Vec3) {
		self.origin = new;
	}

	pub fn add_to_x_angle(&mut self, angle: f64) {
		self.x_angle += angle;
		println!("adding to x angle now angle equal {}", self.x_angle);
	}

	pub fn add_to_y_angle(&mut self, angle: f64) {
		self.y_angle += angle;
		println!("adding to y angle now angle equal {}", self.y_angle);
	}

	pub fn add_to_z_angle(&mut self, angle: f64) {
		self.z_angle += angle;
		println!("adding to z angle now angle equal {}", self.z_angle);
	}

	pub fn get_x_angle(&self) -> f64 {
		self.x_angle
	}

	pub fn get_y_angle(&self) -> f64 {
		self.y_angle
	}

	pub fn get_z_angle(&self) -> f64 {
		self.z_angle
	}

	pub fn set_omni(&mut self, new: Matrix::<f64>) {
		self.omnimatrice = self.omnimatrice.clone() * new;
		println!("getting omnimatrice {:?}", self.omnimatrice);
	}

	pub fn get_omni(&self) -> Matrix<f64> {
		self.omnimatrice.clone()
	}

}