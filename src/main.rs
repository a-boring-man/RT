#![allow(non_snake_case)]
use winit::event_loop::EventLoop; // create the nessessary context to create a windows
use winit::window::{WindowBuilder}; // creater of windows
use winit::dpi::PhysicalSize; // use to define the size of the window at its creation
use winit::event::{Event, WindowEvent}; // capture event and window event like key press or resize respectivly
use rayon::prelude::*; // use for multithreading
// list all other file needed
mod vec3;
mod camera;
mod color;
mod matrix;
mod keynput;

use crate::vec3::Vec3; // my vec3 class use for geometry arithmetic
use crate::color::{background_color}; // the background debug color function
use crate::camera::Camera; // the camera class
use crate::keynput::handle_keypress;

// global constante
const IMAGE_WIDTH: usize = 850;
const IMAGE_HEIGHT: usize = 600;
const FOV: f64 = 145.0;

// aliassing
type Color = Vec3;

fn main() {
    let backgroun_color = Color::new(0.0, 0.0, 0.0);
    
    //initialazing all the windows stuff
    let mut window_size = (IMAGE_WIDTH, IMAGE_HEIGHT);
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()

        .with_title("RT")
        .with_inner_size(PhysicalSize::new(window_size.0 as u32, window_size.1 as u32))
        .build(&event_loop).unwrap();
    let mut graphics_context = unsafe {softbuffer::GraphicsContext::new(&window, &window)}.unwrap();

    // fill the buffer for the first time with black pixel
    let mut buffer = vec![backgroun_color.into_color(); window_size.0 * window_size.1]; // call the from impl for the type of data in background color
    // then creating the camera and create all of it's vector
    let mut camera = Camera::new(IMAGE_HEIGHT as usize, IMAGE_WIDTH as usize, FOV, Vec3::new(0.0, 0.0, 0.0));

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll(); // use set_wait
        match event { // in case of an event do the following
            Event::RedrawRequested(_window) => { // redraw if window has changed or if requested
                let t = std::time::Instant::now(); // use for performance measuring
                let len = buffer.len();
                let thread_count: usize = std::thread::available_parallelism().unwrap().into(); // use to know the number of thread
                let chunk_size = len / thread_count; // use to slice the work by the number of thread

                // main loop remove "par_" in par_chuncks_mut to remove multithreading
                buffer.par_chunks_mut(chunk_size).enumerate().for_each(|(chunk_index, chunk)| {
                    chunk.iter_mut().enumerate().for_each(|(pixel_index, pixel)| {
                        let global_pixel_index = pixel_index + chunk_index * chunk_size; // the global pixel id
                        *pixel = background_color(camera.get_ray(global_pixel_index)).into_color(); // change the contente of the pixel in the buffer
                    });
                });

                // display the changed buffer
                graphics_context.set_buffer(&buffer, window_size.0 as u16, window_size.1 as u16);
                println!("FPS: {}", 1.0/t.elapsed().as_secs_f64());
            }
            Event::WindowEvent {event, ..} => { // in case of an event
                match event {
                    WindowEvent::CloseRequested => {
                        control_flow.set_exit();
                    },
                    WindowEvent::Resized(mut physical_size) => { // if window is resize
                        if physical_size.height as usize <= 0 {
                            physical_size.height = 1;}
                        window_size = (physical_size.width as usize, physical_size.height as usize); // new size
                        camera.update_size(window_size.1, window_size.0, camera.fov()); // update the camera class and all the ray
                        buffer.resize(window_size.0 * window_size.1, backgroun_color.into_color()); // resize the buffer
                    }
                    WindowEvent::KeyboardInput { device_id:_, input, is_synthetic:_ } => {
                        match input.scancode {
                            1 => {control_flow.set_exit();}
                            _ => {handle_keypress(input);}
                        }
                        println!("key pressed : {}", input.scancode);
                    }
                    _ => {},
                }
            }
            _ => {},
        }
    })
}
