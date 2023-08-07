use crate::vec3::Vec3;
use crate::RAY_T_MAX;

type Point3 = Vec3;

pub struct Ray {
	origin: Point3,
	direction: Vec3,
	t_max: f64,
}

impl Ray {

	pub fn new(origin: Point3, direction: Vec3) -> Self {
		Ray { origin , direction , t_max: RAY_T_MAX }
	}

	pub fn new_bound(origin: Point3, direction: Vec3, t_max: f64) -> Self {
		Ray { origin , direction , t_max }
	}

	pub fn dir(&self) -> Vec3 {
		self.direction.clone()
	}

	pub fn org(&self) -> Point3 {
		self.origin.clone()
	}

	pub fn at(&self) -> Point3 {
		self.origin + self.direction
	}
}