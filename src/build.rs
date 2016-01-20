extern crate gcc;

fn main() {
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
}
