extern crate libc;
extern crate sdl2;
extern crate imgui_rs;

use std::mem;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use imgui_rs::imgui::*;

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

    // Initialize IMGUI-SDL binding
    unsafe {
        // This example currently relies on the IMGUI_ImplSdlGL3 sample code
        // which in turn relies on gl3w. Neither is a requirement for using
        // IMGUI
        gl3wInit();
        ImGui_ImplSdlGL3_Init(mem::transmute(window.raw()));
    }

    // Initialize SDL renderer
    let mut renderer = window.renderer().build().unwrap();
    renderer.set_draw_color(Color::RGB(0, 0, 100));
    renderer.clear();
    renderer.present();

    // Initialize SDL event pump
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        // Handle incoming events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        unsafe {
            // Assemble test window containing several test widgets
            ImGui_ImplSdlGL3_NewFrame();
            igShowTestWindow(&mut 1);

            // Clear the frame buffer
            renderer.set_draw_color(Color::RGB(0, 0, 100));
            renderer.clear();

            // Render IMGUI frame
            igRender();

            // Swap buffers
            renderer.present();
        }
    }
}
