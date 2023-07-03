use winit::event::KeyboardInput;

use crate::camera::Camera;
use crate::matrix::Matrix;
use crate::vec3::Vec3;

pub fn handle_keypress(input: KeyboardInput, camera: &mut Camera) {
	//println!("key pressed : {}", input.scancode);
	let mut work_to_be_done = true;
	let mut translation = false;
	let mut rotation_matrix = Matrix::<f64>::new_identity(3, 3);
	let mut translation_vector = Vec3::new(0.0, 0.0, 0.0);
	match input.scancode {
		17 => {translation_vector = camera.translation_x.clone();
				translation = true;}
		31 => {translation_vector = camera.translation_x_neg.clone();
			translation = true;}
		32 => {translation_vector = camera.translation_z.clone();
			translation = true;}
		30 => {translation_vector = camera.translation_z_neg.clone();
			translation = true;}
		57 => {translation_vector = camera.translation_y.clone();
			translation = true;}
		42 => {translation_vector = camera.translation_y_neg.clone();
			translation = true;}
		57416 => {rotation_matrix = camera.rot_z.clone();}
		57424 => {rotation_matrix = camera.rot_z_neg.clone();}
		57421 => {rotation_matrix = camera.rot_y_neg.clone();}
		57419 => {rotation_matrix = camera.rot_y.clone();}
		18 => {rotation_matrix = camera.rot_x.clone();}
		16 => {rotation_matrix = camera.rot_x_neg.clone();}
		_ => {work_to_be_done = false}
	}
	if !work_to_be_done {
		return;
	}
	if translation {
		println!("work : {:?}", rotation_matrix.clone());
		camera.set_origin(translation_vector.clone() + camera.get_origin());
	}
	camera.set_direction(rotation_matrix.clone() * camera.get_direction());
	camera.set_width(rotation_matrix.clone() * camera.get_width());
	camera.set_heigth(rotation_matrix.clone() * camera.get_heigth());
	camera.update_ray();
}
