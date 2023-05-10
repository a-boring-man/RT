use winit::event::KeyboardInput;

use crate::camera::Camera;

pub fn handle_keypress(input: KeyboardInput, mut camera: &Camera) {
	println!("key pressed : {}", input.scancode);
	match input.scancode {
		17 => {move_front(5, camera);}
		_ => {}
	}
}

fn move_front(distance: i16, mut camera: &Camera) {
	
}