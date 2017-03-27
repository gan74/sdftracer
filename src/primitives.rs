
use sdf::{SDF};
use vec::{Vec3};
use ray::{Ray};

pub struct Sphere {
	radius: f32
}

impl Sphere {
	pub fn new(r: f32) -> Sphere {
		Sphere {
			radius: r
		}
	}
}

impl SDF for Sphere {
	fn eval(&self, r: Ray) -> f32 {
		r.origin.length() - self.radius
	}
}





pub struct Plane {
	normal: Vec3
}

impl Plane {
	pub fn new(normal: Vec3) -> Plane {
		Plane {
			normal: normal
		}
	}
}

impl SDF for Plane {
	fn eval(&self, r: Ray) -> f32 {
		self.normal.dot(&r.origin)
	}
}

