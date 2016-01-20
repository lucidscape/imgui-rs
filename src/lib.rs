extern crate libc;

pub mod imgui;

#[cfg(feature = "sdl2")]
extern crate gl;

#[cfg(feature = "sdl2")]
extern crate sdl2_sys;

#[cfg(feature = "sdl2")]
extern crate sdl2;

#[cfg(feature = "sdl2")]
pub mod sdl_renderer;
