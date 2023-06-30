use winit::event::KeyboardInput;

use crate::camera::Camera;
use crate::matrix::Matrix;

pub fn handle_keypress(input: KeyboardInput, camera: &mut Camera) {
	//println!("key pressed : {}", input.scancode);
	let mut work_to_be_done = true;
	let mut transform_matrice = Matrix::<f64>::new_identity; 
	match input.scancode {
		17 => {transform_matrice = camera.translation_x;}
		31 => {transform_matrice = camera.translation_x_neg;}
		32 => {transform_matrice = camera.translation_z;}
		30 => {transform_matrice = camera.translation_z_neg;}
		57 => {transform_matrice = camera.translation_y;}
		42 => {transform_matrice = camera.translation_y_neg;}
		57416 => {camera.rot_z}
		57424 => {camera.rot_z_neg}
		57421 => {camera.rot_y_neg}
		57419 => {camera.rot_y}
		18 => {camera.rot_x;}
		16 => {camera.rot_x_neg}
		_ => {work_to_be_done = false}
	}
	if !work_to_be_done {
		return;
	}
	camera.update_ray();
}
