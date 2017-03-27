
use std::cmp::*;


#[derive(Debug, Clone, Copy)]
pub struct Float {
	pub value: f32
}

impl From<f32> for Float {
	fn from(v: f32) -> Float {
		Float {
			value: v
		}
	}
}


impl PartialEq for Float {
	fn eq(&self, o: &Float) -> bool {
		self.value == o.value
	}
}

impl PartialOrd for Float {
	fn partial_cmp(&self, o: &Float) -> Option<Ordering> {
		self.value.partial_cmp(&o.value)
	}
}


impl Eq for Float {
}

impl Ord for Float {
	fn cmp(&self, o: &Float) -> Ordering {
		self.partial_cmp(o).unwrap_or(Ordering::Equal)
	}
}