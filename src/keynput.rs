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
	let mut new_omni = camera.get_omni();
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
		57421 => {rotation_matrix = camera.rot_y.clone();}
		57419 => {rotation_matrix = camera.rot_y_neg.clone();}
		18 => {rotation_matrix = camera.rot_x.clone();}
		16 => {rotation_matrix = camera.rot_x_neg.clone();}
		_ => {work_to_be_done = false}
	}
	if !work_to_be_done {
		return;
	}
	if translation {
		println!("work : {:?}", rotation_matrix.clone());
		camera.set_origin(camera.get_omni() * translation_vector.clone() + camera.get_origin());
	}
	new_omni = camera.get_omni() * rotation_matrix.clone();
	camera.set_direction(new_omni.clone() * Vec3::new(1.0, 0.0, 0.0));
	camera.set_width(new_omni.clone() * Vec3::new(0.0, 0.0, 1.0));
	camera.set_heigth(new_omni.clone() * Vec3::new(0.0, 1.0, 0.0));
	// camera.set_direction(new_omni.clone() * camera.get_inverted_omni() * camera.get_direction());
	// camera.set_width(new_omni.clone() * camera.get_inverted_omni() * camera.get_width());
	// camera.set_heigth(new_omni.clone() * camera.get_inverted_omni() * camera.get_heigth());
	if rotation_matrix == camera.rot_z.clone() {
		camera.set_inverted_omni(camera.rot_z_neg.clone() * camera.get_inverted_omni())
	}
	if rotation_matrix == camera.rot_z_neg.clone() {
		camera.set_inverted_omni(camera.rot_z.clone() * camera.get_inverted_omni())
	}
	if rotation_matrix == camera.rot_y.clone() {
		camera.set_inverted_omni(camera.rot_y_neg.clone() * camera.get_inverted_omni())
	}
	if rotation_matrix == camera.rot_y_neg.clone() {
		camera.set_inverted_omni(camera.rot_y.clone() * camera.get_inverted_omni())
	}
	if rotation_matrix == camera.rot_x.clone() {
		camera.set_inverted_omni(camera.rot_x_neg.clone() * camera.get_inverted_omni())
	}
	if rotation_matrix == camera.rot_x_neg.clone() {
		camera.set_inverted_omni(camera.rot_x.clone() * camera.get_inverted_omni())
	}
	camera.set_omni(new_omni);
	camera.update_ray();
}
