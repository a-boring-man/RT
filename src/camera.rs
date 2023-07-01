// depedency
use crate::vec3::Vec3;
use crate::matrix::Matrix;

// aliassing
type Point3 = Vec3;

pub struct Camera {
	pub ray: Vec<Vec3>,
	origin: Point3,
	heigth: usize,
	width: usize,
	depth: f64,
	fov: f64,
	depth_unit_vector: Vec3,
	width_unit_vector: Vec3,
	heigth_unit_vector: Vec3,
	omnimatrice: Matrix<f64>,
	pub translation_x: Vec3,
	pub translation_y: Vec3,
	pub translation_z: Vec3,
	pub translation_x_neg: Vec3,
	pub translation_y_neg: Vec3,
	pub translation_z_neg: Vec3,
	pub rot_x: Matrix<f64>,
	pub rot_y: Matrix<f64>,
	pub rot_z: Matrix<f64>,
	pub rot_x_neg: Matrix<f64>,
	pub rot_y_neg: Matrix<f64>,
	pub rot_z_neg: Matrix<f64>,
}

impl Camera {

	/**
	 * create and return a new camera
	 */
	pub fn new (heigth: usize, width: usize, fov: f64, origin: Point3, rotate_speed: f64, translation_speed: f64) -> Self {
		let mut camera = Camera { ray: Vec::with_capacity(width * heigth), origin, heigth, width, fov,
			depth: (width as f64 / 2.0) / (fov.to_radians() / 2.0).tan(),
			depth_unit_vector: Vec3::new(1.0, 0.0, 0.0),
			width_unit_vector: Vec3::new(0.0, 0.0, 1.0),
			heigth_unit_vector: Vec3::new(0.0, 1.0, 0.0),
			omnimatrice : Matrix::<f64>::new_identity(3, 3),
			translation_x : Vec3::new(translation_speed, 0.0, 0.0),
			translation_y : Vec3::new(0.0, translation_speed, 0.0),
			translation_z : Vec3::new(0.0, 0.0, translation_speed),
			translation_x_neg : Vec3::new(-translation_speed, 0.0, 0.0),
			translation_y_neg : Vec3::new(0.0, -translation_speed, 0.0),
			translation_z_neg : Vec3::new(0.0, 0.0, -translation_speed),
			rot_x : Matrix::<f64>::new_rot_by_x(rotate_speed),
			rot_y : Matrix::<f64>::new_rot_by_y(rotate_speed),
			rot_z : Matrix::<f64>::new_rot_by_z(rotate_speed),
			rot_x_neg : Matrix::<f64>::new_rot_by_x(-rotate_speed),
			rot_y_neg : Matrix::<f64>::new_rot_by_y(-rotate_speed),
			rot_z_neg : Matrix::<f64>::new_rot_by_z(-rotate_speed), };
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

	pub fn set_direction(&mut self, new: Vec3) {
		self.depth_unit_vector = new;
	}

	pub fn get_width(&self) -> Vec3 {
		self.width_unit_vector
	}

	pub fn set_width(&mut self, new: Vec3) {
		self.width_unit_vector = new;
	}

	pub fn get_heigth(&self) -> Vec3 {
		self.heigth_unit_vector
	}

	pub fn set_heigth(&mut self, new: Vec3) {
		self.heigth_unit_vector = new;
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
				self.ray.push(((self.depth * self.depth_unit_vector) + (w as f64 - self.width as f64 / 2.0) * self.width_unit_vector + ((self.heigth as f64 / 2.0) - h as f64) * self.heigth_unit_vector).normalized());
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

	pub fn set_omni(&mut self, new: Matrix::<f64>) {
		self.omnimatrice = self.omnimatrice.clone() * new;
		println!("getting omnimatrice {:?}", self.omnimatrice);
	}

	pub fn get_omni(&self) -> Matrix<f64> {
		self.omnimatrice.clone()
	}

}