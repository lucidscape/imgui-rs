extern crate libc;
extern crate gl;
extern crate sdl2;
extern crate imgui_rs;

use std::os;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use imgui_rs::imgui::*;
use imgui_rs::sdl_renderer::UI;

pub fn main() {
    // Initialize SDL video
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // Initialize SDL window
    let window = video_subsystem.window("imgui-rs demo", 1280, 720)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const os::raw::c_void);

    // Initialize SDL renderer
    let mut renderer = window.renderer().build().unwrap();
    renderer.set_draw_color(Color::RGB(0, 0, 100));
    renderer.clear();
    renderer.present();

    // Initialize imgui renderer
    let mut ui = UI::new();

    // Initialize SDL event pump
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        // Handle incoming events
        for event in event_pump.poll_iter() {
            match ui.handle_event(&event) {
                true => (),
                false => {
                    match event {
                        Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                            break 'running
                        },
                        _ => {}
                    }
                }
            }
        }

        unsafe {
            // Clear the frame buffer
            renderer.set_draw_color(Color::RGB(0, 0, 100));
            renderer.clear();

            let (w, h) = renderer.window().expect("window").size();
            let (_, x, y) = sdl_context.mouse().mouse_state();
            ui.begin_frame(w, h, x, y, 0.01);

            // Assemble a test window containing several test widgets
            igShowTestWindow(&mut 1);

            // Render IMGUI frame
            ui.end_frame();

            // Swap buffers
            renderer.present();
        }
    }
}
