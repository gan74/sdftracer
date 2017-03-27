use object::{Object};
use vec::{Vec3};
use ray::{Ray};

use utils::*;


pub struct Scene {
	pub objects: Vec<Object>
}

impl Scene {
	pub fn new() -> Scene {
		Scene {
			objects: Vec::new()
		}
	}


	pub fn eval(&self, mut ray: Ray, far: f32) -> Option<Vec3> {
		let mut total = 0.0;
		
		let epsilon = 0.0001;
		loop {
			let closest = self.objects.iter().min_by_key(|obj| Float::from(obj.eval(ray))).unwrap();
			let dist = closest.eval(ray);

			if dist <= 0.0 {
				return Some(ray.origin);
			}

			if total >= far {
				return None
			}

			total += dist;
			ray.origin = ray.origin + ray.direction * (dist + epsilon);
		}
	}
}