use crate::vec3::Vec3;
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

type Point3 = Vec3;

struct Sphere {
	center: Point3,
	radius: f64,
}

impl Sphere {

	pub fn new (center: Point3, radius: f64) -> Self {
		Sphere { center , radius }
	}
}

// impl Hittable for Sphere {

// 	fn hit(ray: Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
// 		ray.org() + t * ray.dir() = radius
// 	}
// }