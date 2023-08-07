use crate::vec3::Vec3;
use crate::ray::Ray;

type Point3 = Vec3;

pub struct HitRecord {
	p: Point3,
	normal: Vec3,
	t: f64,
}

impl HitRecord {

	pub fn new() -> HitRecord {
		HitRecord {p : Vec3::new(0.0, 0.0, 0.0), normal: Vec3::new(0.0, 0.0, 0.0), t: 0.0}
	}
}

pub trait Hittable {
	fn hit(ray: Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool;
}