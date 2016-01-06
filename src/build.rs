#[cfg(feature="pkg-config")]
extern crate pkg_config;
extern crate gcc;

#[cfg(not(feature="pkg-config"))]
fn build_pkgconfig() -> bool {
    false
}

#[cfg(feature="pkg-config")]
fn build_pkgconfig() -> bool {
    if pkg_config::find_library("sdl2").is_err() {
        panic!("Could not find SDL2 via pkgconfig");
    }
    true
}

fn main() {
    println!("cargo:rustc-flags=-l GL");
    if !build_pkgconfig() {
        println!("cargo:rustc-flags=-l SDL2");
    }
    
    gcc::Config::new()
        .cpp(true)
        .include("src/cimgui/imgui/")
        .file("src/cimgui/imgui/imgui.cpp")
        .file("src/cimgui/imgui/imgui_draw.cpp")
        .file("src/cimgui/imgui/imgui_demo.cpp")
        .compile("libimgui.a");

    gcc::Config::new()
        .file("src/cimgui/cimgui/cimgui.cpp")
        .file("src/cimgui/cimgui/fontAtlas.cpp")
        .file("src/cimgui/cimgui/drawList.cpp")
        .compile("libcimgui.a");

    gcc::Config::new()
        .include("src/cimgui/imgui/")
        .include("src/cimgui/imgui/examples/libs/gl3w/")
        .file("src/cimgui/imgui/examples/libs/gl3w/GL/gl3w.c")
        .file("src/sdlgl3/imgui_impl_sdl_gl3.cpp")
        .compile("libimgui_sdl_gl3.a");
}
