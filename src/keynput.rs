use winit::event::KeyboardInput;



pub fn handle_keypress(input: KeyboardInput) {
	println!("key pressed : {}", input.scancode);
}