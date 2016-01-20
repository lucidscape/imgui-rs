extern crate libc;
extern crate gl;
extern crate sdl2;
extern crate imgui_rs;

use std::os;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::mouse::Mouse;
use sdl2::keyboard::Keycode;
use imgui_rs::imgui::*;
use imgui_rs::renderer::UI;

pub fn main() {
    let mut state_mouse = [0u8, 0u8, 0u8];
    let mut state_wheel = 0.0;

    // Initialize SDL video
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // Initialize SDL window
    let window = video_subsystem.window("imgui-rs demo", 1280, 720)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    // Load OGL function pointers
    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const os::raw::c_void);

    // Initialize SDL renderer
    let mut renderer = window.renderer().build().unwrap();
    renderer.set_draw_color(Color::RGB(0, 0, 100));
    renderer.clear();
    renderer.present();

    // Initialize imgui renderer
    let mut ui = UI::new();

    // Set keyboard mapping, ImGui will use those indices to peek into the io.KeyDown[] array.
    unsafe {
        (*ui.io).KeyMap[ImGuiKey_Tab as usize] = 0;
        (*ui.io).KeyMap[ImGuiKey_LeftArrow as usize] = 1;
        (*ui.io).KeyMap[ImGuiKey_RightArrow as usize] = 2;
        (*ui.io).KeyMap[ImGuiKey_UpArrow as usize] = 3;
        (*ui.io).KeyMap[ImGuiKey_DownArrow as usize] = 4;
        (*ui.io).KeyMap[ImGuiKey_PageUp as usize] = 5;
        (*ui.io).KeyMap[ImGuiKey_PageDown as usize] = 6;
        (*ui.io).KeyMap[ImGuiKey_Home as usize] = 7;
        (*ui.io).KeyMap[ImGuiKey_End as usize] = 8;
        (*ui.io).KeyMap[ImGuiKey_Delete as usize] = 9;
        (*ui.io).KeyMap[ImGuiKey_Backspace as usize] = 10;
        (*ui.io).KeyMap[ImGuiKey_Enter as usize] = 11;
        (*ui.io).KeyMap[ImGuiKey_Escape as usize] = 12;
        (*ui.io).KeyMap[ImGuiKey_A as usize] = 13;
        (*ui.io).KeyMap[ImGuiKey_C as usize] = 14;
        (*ui.io).KeyMap[ImGuiKey_V as usize] = 15;
        (*ui.io).KeyMap[ImGuiKey_X as usize] = 16;
        (*ui.io).KeyMap[ImGuiKey_Y as usize] = 17;
        (*ui.io).KeyMap[ImGuiKey_Z as usize] = 18;
    }

    // Enter main loop
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        // Handle incoming events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                }

                Event::MouseWheel { y, .. } => {
                    if y > 0 {
                        state_wheel = 1.0;
                    } else if y < 0 {
                        state_wheel = -1.0;
                    };
                }

                Event::MouseButtonDown { mouse_btn, .. } => {
                    match mouse_btn {
                        Mouse::Left => state_mouse[0] = 1,
                        Mouse::Middle => state_mouse[1] = 1,
                        Mouse::Right => state_mouse[2] = 1,
                        _ => ()
                    }
                }

                Event::TextInput { text, .. } => {
                    unsafe {
                        ImGuiIO_AddInputCharactersUTF8(text.as_ptr() as *const i8);
                    }
                }

                Event::KeyDown { keycode: Some(keycode), keymod, .. }  => {
                    unsafe {
                        let k = keycode as i32 & !(1<<30);
                        (*ui.io).KeysDown[k as usize] = 1;
                        (*ui.io).KeyShift = if keymod.bits() & (0x0001 | 0x0002) > 0 { 1 } else { 0 };
                        (*ui.io).KeyCtrl = if keymod.bits() & (0x0040 | 0x0080) > 0 { 1 } else { 0 };
                        (*ui.io).KeyAlt = if keymod.bits() & (0x0100 | 0x0200) > 0 { 1 } else { 0 };
                    }
                }

                Event::KeyUp { keycode: Some(keycode), keymod, .. }  => {
                    unsafe {
                        let k = keycode as i32 & !(1<<30);
                        (*ui.io).KeysDown[k as usize] = 0;
                        (*ui.io).KeyShift = if keymod.bits() & (0x0001 | 0x0002) > 0 { 1 } else { 0 };
                        (*ui.io).KeyCtrl = if keymod.bits() & (0x0040 | 0x0080) > 0 { 1 } else { 0 };
                        (*ui.io).KeyAlt = if keymod.bits() & (0x0100 | 0x0200) > 0 { 1 } else { 0 };
                    }
                }

                _ => ()
            }
        }

        unsafe {
            // Clear the frame buffer
            renderer.set_draw_color(Color::RGB(0, 0, 100));
            renderer.clear();

            // Pass mouse state to ImGui
            (*ui.io).MouseDown[0] = state_mouse[0];
            (*ui.io).MouseDown[1] = state_mouse[1];
            (*ui.io).MouseDown[2] = state_mouse[2];
            (*ui.io).MouseWheel = state_wheel;
            state_mouse[0] = 0;
            state_mouse[1] = 0;
            state_mouse[2] = 0;
            state_wheel = 0.0;

            // Update ImGui
            let (w, h) = renderer.window().expect("window").size();
            let (_, x, y) = sdl_context.mouse().mouse_state();
            ui.begin_frame(w, h, x, y, 1.0 / 60.0);

            // Assemble a test window containing several test widgets
            igShowTestWindow(&mut 1);

            // Render IMGUI frame
            ui.end_frame();

            // Swap buffers
            renderer.present();
        }
    }
}
