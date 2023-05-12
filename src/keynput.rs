use winit::event::KeyboardInput;

use crate::camera::Camera;

pub fn handle_keypress(input: KeyboardInput, camera: &mut Camera) {
	//println!("key pressed : {}", input.scancode);
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
		_ => {}
	}
	println!("camera origin : {}, {}, {}", camera.get_origin().x(), camera.get_origin().y(), camera.get_origin().z())
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