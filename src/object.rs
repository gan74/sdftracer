

use sdf::{SDF};
use vec::{Vec3};
use ray::{Ray};

pub struct Object {
	pub position: Vec3,
	pub sdf: Box<SDF>
}

impl Object {
	pub fn from_sdf(sdf: Box<SDF>) -> Object {
		Object {
			position: Vec3::zero(),
			sdf: sdf
		}
	}

	pub fn eval(&self, r: Ray) -> f32 {
		self.sdf.eval(self.to_local(r))
	}


	pub fn to_local(&self, r: Ray) -> Ray {
		Ray {
			origin: r.origin - self.position,
			direction: r.direction
		}
	}
}