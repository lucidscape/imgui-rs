IMGUI-RS
========

IMGUI-RS provides Rust bindings for IMGUI, a bloat-free intermediate mode GUI library for C/C++.

IMGUI outputs vertex buffers that you can render in your 3D-pipeline enabled application. It is fast, portable, renderer agnostic and self-contained.

IMGUI does away with state synchronization by requiring the application to explicitly pass all state required in real-time. The user interface only retains the minimal amount of state required to facilitate the functionality required by each type of widget supported by the system.

```toml
[dependencies]
imgui-rs = { git = "https://github.com/lucidscape/imgui-rs.git" }
```

Gallery
-------

![screenshot 1](https://raw.githubusercontent.com/wiki/ocornut/imgui/web/v143/examples_04.png)
![screenshot 2](https://raw.githubusercontent.com/wiki/ocornut/imgui/web/v143/test_window_01.png)
![screenshot 3](https://raw.githubusercontent.com/wiki/ocornut/imgui/web/v143/test_window_02.png)
![screenshot 4](https://raw.githubusercontent.com/wiki/ocornut/imgui/web/v143/test_window_03.png)
![screenshot 5](https://raw.githubusercontent.com/wiki/ocornut/imgui/web/v140/test_window_05_menus.png)
![screenshot 6](https://raw.githubusercontent.com/wiki/ocornut/imgui/web/v143/skinning_sample_02.png)
![screenshot 7](https://cloud.githubusercontent.com/assets/8225057/7903336/96f0fb7c-07d0-11e5-95d6-41c6a1595e5a.png)
