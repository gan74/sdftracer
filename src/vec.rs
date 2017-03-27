
use std::ops::*;

#[derive(Default, Debug, Clone, Copy)]
pub struct Vec3 {
	pub x: f32,
	pub y: f32,
	pub z: f32
}


impl Vec3 {
	pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
		Vec3 {
			x: x,
			y: y,
			z: z
		}
	}

	pub fn zero() -> Vec3 {
		Vec3::new(0.0, 0.0, 0.0)
	}

	pub fn dot(&self, p: &Vec3) -> f32 {
		self.x * p.x + self.y * p.y + self.z * p.z
	}

	pub fn distance2(&self, p: Vec3) -> f32 {
		self.sub(p).length2()
	}

	pub fn length2(&self) -> f32 {
		self.dot(self)
	}

	pub fn distance(&self, p: Vec3) -> f32 {
		self.distance2(p).sqrt()
	}

	pub fn length(&self) -> f32 {
		self.length2().sqrt()
	}

	pub fn normalized(&self) -> Vec3 {
		self.clone() / self.length()
	}
}










impl From<f32> for Vec3 {
	fn from(x: f32) -> Vec3 {
		Vec3::new(x, x, x)
	}
}

impl PartialEq for Vec3 {
	fn eq(&self, o: &Self) -> bool {
		self.x == o.x && self.y == o.y && self.z == o.z
	}
}







impl Add for Vec3 {
	type Output = Vec3;

	fn add(self, o: Vec3) -> Vec3 {
		Vec3 {
			x: self.x + o.x,
			y: self.y + o.y,
			z: self.z + o.z
		}
	}
}

impl Sub for Vec3 {
	type Output = Vec3;

	fn sub(self, o: Vec3) -> Vec3 {
		Vec3 {
			x: self.x - o.x,
			y: self.y - o.y,
			z: self.z - o.z
		}
	}
}

impl Mul for Vec3 {
	type Output = Vec3;

	fn mul(self, o: Vec3) -> Vec3 {
		Vec3 {
			x: self.x * o.x,
			y: self.y * o.y,
			z: self.z * o.z
		}
	}
}

impl Div for Vec3 {
	type Output = Vec3;

	fn div(self, o: Vec3) -> Vec3 {
		Vec3 {
			x: self.x / o.x,
			y: self.y / o.y,
			z: self.z / o.z
		}
	}
}









impl Add<f32> for Vec3 {
	type Output = Vec3;

	fn add(self, o: f32) -> Vec3 {
		self.add(Vec3::from(o))
	}
}

impl Sub<f32> for Vec3 {
	type Output = Vec3;

	fn sub(self, o: f32) -> Vec3 {
		self.sub(Vec3::from(o))
	}
}

impl Mul<f32> for Vec3 {
	type Output = Vec3;

	fn mul(self, o: f32) -> Vec3 {
		self.mul(Vec3::from(o))
	}
}

impl Div<f32> for Vec3 {
	type Output = Vec3;

	fn div(self, o: f32) -> Vec3 {
		self.div(Vec3::from(o))
	}
}




/*
impl AddAssign for Vec3 {
	fn add_assign(&mut self, o: Vec3) {
		*self = *self + o;
	}
}

impl MulAssign for Vec3 {
	fn mul_assign(&mut self, o: Vec3) {
		*self = *self * o;
	}
}

impl AddAssign<f32> for Vec3 {
	fn add_assign(&mut self, o: f32) {
		*self = *self + o;
	}
}

impl MulAssign<f32> for Vec3 {
	fn mul_assign(&mut self, o: f32) {
		*self = *self * o;
	}
}
*/
