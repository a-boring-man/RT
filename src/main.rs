use winit::event_loop::EventLoop; // create the nessessary context to create a windows
use winit::window::WindowBuilder; // creater of windows
use winit::dpi::PhysicalSize; // use to define the size of the window at its creation
use winit::event::{Event, WindowEvent}; // capture event and window event like key press or resize respectivly
use rayon::prelude::*; // use for multithreading

#[warn(non_snake_case)]
mod vec3;

use crate::vec3::Vec3; // my vec3 class use for geometry arithmetic

// global constante
const IMAGE_WIDTH: usize = 400;
const ASPECT_RATIO: f64 = (16 / 9) as f64;
const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 * ASPECT_RATIO) as usize;

// aliassing
type Color = Vec3;

impl From<Color> for u32 {
    fn from(color: Color) -> Self {
        (color.b() * 255.0) as u32 | ((color.g() * 255.0) as u32) << 8 | ((color.r() * 255.0) as u32) << 16
    }
}

fn main() {
    let mut window_size = (IMAGE_WIDTH, IMAGE_HEIGHT);
    let background_color = Color::new(0.2, 0.0, 0.5);

    //initialazing all the windows stuff
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
            .with_title("RT")
            .with_inner_size(PhysicalSize::new(window_size.0 as u32, window_size.1 as u32))
            .build(&event_loop).unwrap();

    let mut graphics_context = unsafe {softbuffer::GraphicsContext::new(&window, &window)}.unwrap();
    let mut buffer = vec![background_color.into(); window_size.0 * window_size.1]; // call the from impl for the type of data in background color

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll(); // use set_wait
        match event { // in case of an event do the following
            Event::MainEventsCleared => { // after all the event have been handle do this (use to run the loop indefinitly in case of no event)
                let t = std::time::Instant::now(); // use for performance measuring
                // render
                let len = buffer.len();
                let thread_count: usize = std::thread::available_parallelism().unwrap().into();
                let chunk_size = len / thread_count;

                buffer.par_chunks_mut(chunk_size).enumerate().for_each(|(chunk_index, chunk)| {
                    chunk.iter_mut().enumerate().for_each(|(pixel_index, pixel)| {
                        let global_pixel_index = pixel_index + chunk_index * chunk_size;
                        let abs_x = global_pixel_index % window_size.0;
                        let abs_y = global_pixel_index / window_size.0;
                        *pixel = Color::new(abs_x as f64 / window_size.0 as f64, abs_y as f64 / window_size.1 as f64, 0.0).into();
                    });
                });
                graphics_context.set_buffer(&buffer, window_size.0 as u16, window_size.1 as u16);
                println!("FPS: {}", 1.0/t.elapsed().as_secs_f64());
            }
            Event::WindowEvent {event, ..} => {
                match event {
                    WindowEvent::CloseRequested => {
                        control_flow.set_exit();
                    },
                    WindowEvent::Resized(physical_size) => {
                        window_size = (physical_size.width as usize, physical_size.height as usize);
                        buffer.resize(window_size.0 * window_size.1, background_color.into());
                    }
                    _ => {},
                }
            }
            _ => {},
        }
    })
}
