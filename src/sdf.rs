
use ray::{Ray};

pub trait SDF {
	fn eval(&self, _: Ray) -> f32; 
}