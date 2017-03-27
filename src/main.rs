
extern crate image;
extern crate rand;

mod vec;
mod sdf;
mod primitives;
mod object;
mod scene;
mod ray;
mod utils;


use vec::*;
use ray::*;
use scene::*;
use object::*;
use primitives::*;


use std::fs::{File};
use std::path::{Path};

use image::{ImageBuffer};



fn main() {
	let mut scene = Scene::new();

	for _ in 0..4 {

		let mut sp = Object::from_sdf(Box::new(Sphere::new(1.0)));
		sp.position = Vec3::new(rand() * 5.0 + 5.0 , nrand() * 4.0, rand() * 2.0);
		scene.objects.push(sp);
	}
	{
		let mut pl = Object::from_sdf(Box::new(Plane::new(Vec3::new(0.0, 0.0, 1.0))));
		pl.position = Vec3::new(0.0, 0.0, -1.0);
		scene.objects.push(pl);
	}

	let orig = Vec3::zero();
	let near = 0.5;
	let far = 10.0;
	let image_res = 768;
	let f_res = image_res as f32 - 1.0;

	let img = ImageBuffer::from_fn(image_res, image_res, |x, y| {
		let world_y = renorm((x as f32) / f_res);
		let world_z = -renorm((y as f32) / f_res);
		let p = Vec3::new(near, world_y, world_z);

		let intersect = scene.eval(Ray::new(orig, p - orig), far);

		let dist = intersect.map(|i| orig.distance(i) / far).unwrap_or(1.0);

		image::Luma([(dist * 255.0) as u8])
	});

    let ref mut out = File::create(&Path::new("./out.png")).unwrap();
    image::ImageLuma8(img).save(out, image::PNG).unwrap();
}


fn renorm(x: f32) -> f32 {
	(x - 0.5) * 0.5
}

fn rand() -> f32 {
	rand::random::<f32>()
}

fn nrand() -> f32 {
	renorm(rand())
}