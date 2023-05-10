use winit::event::KeyboardInput;

use crate::camera::Camera;

pub fn handle_keypress(input: KeyboardInput, camera: &mut Camera) {
	//println!("key pressed : {}", input.scancode);
	match input.scancode {
		17 => {move_front(5.0, camera);}
		31 => {move_front(-5.0, camera);}
		32 => {move_side(5.0, camera);}
		30 => {move_side(-5.0, camera);}
		_ => {}
	}
	println!("camera origin : {}, {}, {}", camera.get_origin().x(), camera.get_origin().y(), camera.get_origin().z())
}

fn move_front(distance: f64, camera: &mut Camera) {
	camera.set_origin(camera.get_origin() + distance * camera.get_direction());
}

fn move_side(distance: f64, camera: &mut Camera) {
	camera.set_origin(camera.get_origin() + distance * camera.get_width());
}