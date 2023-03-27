use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use winit::dpi::PhysicalSize;
use winit::event::{Event, WindowEvent};

const IMAGE_WIDTH: usize = 256;
const IMAGE_HEIGHT: usize = 256;

#[derive(Clone, Copy)]
struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    fn new(r: f64, g: f64, b: f64) -> Self {
        Color{r, g, b}
    }
}

impl From<Color> for u32 {
    fn from(color: Color) -> Self {
        (color.b * 255.0) as u32 | ((color.g * 255.0) as u32) << 8 | ((color.r * 255.0) as u32) << 16
    }
}

fn main() {
    let mut window_size = (IMAGE_WIDTH, IMAGE_HEIGHT);
    let background_color = Color::new(0.1, 0.1, 0.1);

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
            .with_title("RT")
            .with_inner_size(PhysicalSize::new(window_size.0 as u32, window_size.1 as u32))
            .build(&event_loop).unwrap();

    let mut graphics_context = unsafe {softbuffer::GraphicsContext::new(&window, &window)}.unwrap();
    let mut buffer = vec![background_color.into(); window_size.0 * window_size.1];

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll(); // use set_wait
        match event {
            Event::MainEventsCleared => {
                // render
                let len = buffer.len();
                buffer.iter_mut().enumerate().for_each(|(pixel_index, pixel)| {
                    *pixel = Color::new(pixel_index as f64/len as f64, 0.0, 0.0).into();
                });
                graphics_context.set_buffer(&buffer, window_size.0 as u16, window_size.1 as u16);
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
