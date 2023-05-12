use winit::event::KeyboardInput;

use crate::camera::Camera;
use crate::matrix::Matrix;

pub fn handle_keypress(input: KeyboardInput, camera: &mut Camera) {
	//println!("key pressed : {}", input.scancode);
	let mut matrice_has_been_changed = false;
	match input.scancode {
		17 => {move_front(5.0, camera);}
		31 => {move_front(-5.0, camera);}
		32 => {move_right(5.0, camera);}
		30 => {move_right(-5.0, camera);}
		57 => {move_up(5.0, camera);}
		42 => {move_up(-5.0, camera);}
		57416 => {camera.add_to_z_angle(5.0);}
		57424 => {camera.add_to_z_angle(-5.0);}
		57421 => {camera.add_to_y_angle(-5.0);}
		57419 => {camera.add_to_y_angle(5.0);}
		18 => {camera.add_to_y_angle(5.0);}
		16 => {camera.add_to_y_angle(-5.0);}
		_ => {}
	}
	if camera.get_x_angle() - 0.0 > 0.0001 || camera.get_x_angle() - 0.0 < -0.0001 {
		camera.set_omni(Matrix::<f64>::new_rot_by_x(camera.get_x_angle()));
		matrice_has_been_changed = true;
	}
	if camera.get_y_angle() - 0.0 > 0.0001 || camera.get_y_angle() - 0.0 < -0.0001 {
		camera.set_omni(Matrix::<f64>::new_rot_by_x(camera.get_y_angle()));
		matrice_has_been_changed = true;
	}
	if camera.get_z_angle() - 0.0 > 0.0001 || camera.get_z_angle() - 0.0 < -0.0001 {
		camera.set_omni(Matrix::<f64>::new_rot_by_x(camera.get_z_angle()));
		matrice_has_been_changed = true;
	}
	if matrice_has_been_changed {
		//println!("camera omni : {:?}", camera.get_omni())
		camera.update_ray();
	}
}

fn move_front(distance: f64, camera: &mut Camera) {
	camera.set_origin(camera.get_origin() + distance * camera.get_direction());
}

fn move_right(distance: f64, camera: &mut Camera) {
	camera.set_origin(camera.get_origin() + distance * camera.get_width());
}

fn move_up(distance: f64, camera: &mut Camera) {
	camera.set_origin(camera.get_origin() + distance * camera.get_heigth());
}