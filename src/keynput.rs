use winit::event::KeyboardInput;

use crate::camera::Camera;
use crate::matrix::Matrix;

pub fn handle_keypress(input: KeyboardInput, camera: &mut Camera) {
	//println!("key pressed : {}", input.scancode);
	let mut work_to_be_done = true;
	let mut translation = false;
	let mut transform_matrice = Matrix::<f64>::new_identity(3, 3); 
	match input.scancode {
		17 => {transform_matrice = camera.translation_x.clone();
				translation = true;}
		31 => {transform_matrice = camera.translation_x_neg.clone();
			translation = true;}
		32 => {transform_matrice = camera.translation_z.clone();
			translation = true;}
		30 => {transform_matrice = camera.translation_z_neg.clone();
			translation = true;}
		57 => {transform_matrice = camera.translation_y.clone();
			translation = true;}
		42 => {transform_matrice = camera.translation_y_neg.clone();
			translation = true;}
		57416 => {transform_matrice = camera.rot_z.clone();}
		57424 => {transform_matrice = camera.rot_z_neg.clone();}
		57421 => {transform_matrice = camera.rot_y_neg.clone();}
		57419 => {transform_matrice = camera.rot_y.clone();}
		18 => {transform_matrice = camera.rot_x.clone();}
		16 => {transform_matrice = camera.rot_x_neg.clone();}
		_ => {work_to_be_done = false}
	}
	if !work_to_be_done {
		return;
	}
	camera.update_ray();
}
