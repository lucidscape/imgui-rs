use std::ptr;
use std::os::raw;
use std::mem::{size_of, transmute};
use std::ffi::CString;
use libc::{c_void, c_int, c_uchar};
use gl;
use imgui::*;

static VERTEX_SHADER: &'static str = "
    #version 330
    uniform mat4 ProjMtx;
    in vec2 Position;
    in vec2 UV;
    in vec4 Color;
    out vec2 Frag_UV;
    out vec4 Frag_Color;
    void main()
    {
        Frag_UV = UV;
        Frag_Color = Color;
        gl_Position = ProjMtx * vec4(Position.xy, 0, 1);
    }
";

static FRAGMENT_SHADER: &'static str = "
    #version 330
    uniform sampler2D Texture;
    in vec2 Frag_UV;
    in vec4 Frag_Color;
    out vec4 Out_Color;
    void main()
    {
        Out_Color = Frag_Color * texture(Texture, Frag_UV.st);
    }
";

#[derive(Debug)]
pub struct Renderer {
    pub io:*mut ImGuiIO,
    program_handle: u32,
    vertex_shader_handle: u32,
    fragment_shader_handle: u32,
    vao_handle: u32,
    vbo_handle: u32,
    elements_handle: u32,
    font_texture_handle: u32,
    idx_tex: u32,
    idx_projection: u32,
    idx_position: u32,
    idx_uv: u32,
    idx_color: u32
}

fn cstr(input:&str) -> *const i8 {
    CString::new(input.as_bytes()).unwrap().as_ptr() as *const i8
}

impl Renderer {
    pub fn new() -> Renderer {
        unsafe {
            // Backup GL state
            let (mut last_texture, mut last_array_buffer, mut last_vertex_array) = (0, 0, 0);
            gl::GetIntegerv(gl::TEXTURE_BINDING_2D, &mut last_texture);
            gl::GetIntegerv(gl::ARRAY_BUFFER_BINDING, &mut last_array_buffer);
            gl::GetIntegerv(gl::VERTEX_ARRAY_BINDING, &mut last_vertex_array);

            // Compile and link shaders
            let program_handle = gl::CreateProgram();
            let vertex_shader_handle = gl::CreateShader(gl::VERTEX_SHADER);
            let fragment_shader_handle = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(vertex_shader_handle, 1, &cstr(VERTEX_SHADER), ptr::null());
            gl::ShaderSource(fragment_shader_handle, 1, &cstr(FRAGMENT_SHADER), ptr::null());
            gl::CompileShader(vertex_shader_handle);
            gl::CompileShader(fragment_shader_handle);
            gl::AttachShader(program_handle, vertex_shader_handle);
            gl::AttachShader(program_handle, fragment_shader_handle);
            gl::LinkProgram(program_handle);
            let idx_tex = gl::GetUniformLocation(program_handle, cstr("Texture"));
        	let idx_projection = gl::GetUniformLocation(program_handle, cstr("ProjMtx"));
        	let idx_position = gl::GetAttribLocation(program_handle, cstr("Position"));
        	let idx_uv = gl::GetAttribLocation(program_handle, cstr("UV"));
        	let idx_color = gl::GetAttribLocation(program_handle, cstr("Color"));

            // Initialize draw element buffer
            let mut elements_handle = 0;
            gl::GenBuffers(1, &mut elements_handle);

            // Initialize VBO
            let mut vbo_handle = 0;
            gl::GenBuffers(1, &mut vbo_handle);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo_handle);

            // Initialize vertex attributes
            let mut vao_handle = 0;
            let stride = size_of::<ImDrawVert>() as i32;
            let v2size = size_of::<ImVec2>();
            gl::GenVertexArrays(1, &mut vao_handle);
        	gl::BindVertexArray(vao_handle);
        	gl::EnableVertexAttribArray(idx_position as u32);
        	gl::EnableVertexAttribArray(idx_uv as u32);
        	gl::EnableVertexAttribArray(idx_color as u32);
            gl::VertexAttribPointer(idx_position as u32, 2, gl::FLOAT, gl::FALSE, stride, 0 as *const raw::c_void);
            gl::VertexAttribPointer(idx_uv as u32, 2, gl::FLOAT, gl::FALSE, stride, v2size as *const raw::c_void);
            gl::VertexAttribPointer(idx_color as u32, 4, gl::UNSIGNED_BYTE, gl::TRUE, stride, (v2size as isize * 2) as *const raw::c_void);

            // Build font texture atlas
            let mut io = igGetIO();
            let mut pixels: *mut c_uchar = ptr::null_mut();
            let mut width: c_int = 0;
            let mut height: c_int = 0;
            let mut bytes_per_pixel: c_int = 0;
            ImFontAtlas_GetTexDataAsRGBA32((*io).Fonts, &mut pixels, &mut width, &mut height, &mut bytes_per_pixel);

            // Upload font texture
            let mut font_texture_handle = 0;
          	gl::GenTextures(1, &mut font_texture_handle);
          	gl::BindTexture(gl::TEXTURE_2D, font_texture_handle);
          	gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
          	gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
          	gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, width, height, 0, gl::RGBA, gl::UNSIGNED_BYTE, pixels as *const raw::c_void);
            ImFontAtlas_SetTexID((*io).Fonts, font_texture_handle as *mut c_void);

            // Restore modified GL state
            gl::BindTexture(gl::TEXTURE_2D, last_texture as u32);
            gl::BindBuffer(gl::ARRAY_BUFFER, last_array_buffer as u32);
            gl::BindVertexArray(last_vertex_array as u32);

            // Disable automatic rendering, instead call ImGui::GetDrawData() to acquire draw lists
            (*io).RenderDrawListsFn = None;

            Renderer {
                io: io,
                program_handle: program_handle,
                vertex_shader_handle: vertex_shader_handle,
                fragment_shader_handle: fragment_shader_handle,
                vao_handle: vao_handle,
                vbo_handle: vbo_handle,
                elements_handle: elements_handle,
                font_texture_handle: font_texture_handle,
                idx_tex: idx_tex as u32,
                idx_projection: idx_projection as u32,
                idx_position: idx_position as u32,
                idx_uv: idx_uv as u32,
                idx_color: idx_color as u32
            }
        }
    }

    pub fn begin_frame(&mut self, w:f32, h:f32, mouse_x:f32, mouse_y:f32, dt:f32) {
        unsafe {
            (*self.io).DeltaTime = dt;
        	(*self.io).DisplaySize = ImVec2 { x: w, y: h };
        	(*self.io).DisplayFramebufferScale = ImVec2 { x: 1.0, y: 1.0 };
            (*self.io).MousePos = ImVec2 { x: mouse_x, y: mouse_y };
            igNewFrame();
        }
    }

    pub fn end_frame(&mut self) {
        unsafe {
            // Backup GL state
            let mut last_program = 0;
            let mut last_texture = 0;
            let mut last_array_buffer = 0;
            let mut last_element_array_buffer = 0;
            let mut last_vertex_array = 0;
            let mut last_blend_src = 0;
            let mut last_blend_dst = 0;
            let mut last_blend_equation_rgb = 0;
            let mut last_blend_equation_alpha = 0;
            let mut last_viewport = [0i32, 0i32, 0i32, 0i32];
            let last_enable_blend = gl::IsEnabled(gl::BLEND);
            let last_enable_cull_face = gl::IsEnabled(gl::CULL_FACE);
            let last_enable_depth_test = gl::IsEnabled(gl::DEPTH_TEST);
            let last_enable_scissor_test = gl::IsEnabled(gl::SCISSOR_TEST);
            gl::GetIntegerv(gl::CURRENT_PROGRAM, &mut last_program);
            gl::GetIntegerv(gl::TEXTURE_BINDING_2D, &mut last_texture);
            gl::GetIntegerv(gl::ARRAY_BUFFER_BINDING, &mut last_array_buffer);
            gl::GetIntegerv(gl::ELEMENT_ARRAY_BUFFER_BINDING, &mut last_element_array_buffer);
            gl::GetIntegerv(gl::VERTEX_ARRAY_BINDING, &mut last_vertex_array);
            gl::GetIntegerv(gl::BLEND_SRC, &mut last_blend_src);
            gl::GetIntegerv(gl::BLEND_DST, &mut last_blend_dst);
            gl::GetIntegerv(gl::BLEND_EQUATION_RGB, &mut last_blend_equation_rgb);
            gl::GetIntegerv(gl::BLEND_EQUATION_ALPHA, &mut last_blend_equation_alpha);
            gl::GetIntegerv(gl::VIEWPORT, transmute(&mut last_viewport));

            // Setup render state: alpha-blending enabled, no face culling, no depth testing, scissor enabled
            gl::Enable(gl::BLEND);
            gl::BlendEquation(gl::FUNC_ADD);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Disable(gl::CULL_FACE);
            gl::Disable(gl::DEPTH_TEST);
            gl::Enable(gl::SCISSOR_TEST);
            gl::ActiveTexture(gl::TEXTURE0);

            // Setup orthographic projection matrix
            let (w, h) = ((*self.io).DisplaySize.x, (*self.io).DisplaySize.y);
            gl::Viewport(0, 0, w as i32, h as i32);
            let ortho_projection = [
                2.0/w, 0.0, 0.0, 0.0,
                0.0, 2.0/-h, 0.0, 0.0,
                0.0, 0.0, -1.0, 0.0,
                -1.0, 1.0, 0.0, 1.0,
            ];
            gl::UseProgram(self.program_handle);
            gl::Uniform1i(self.idx_tex as i32, 0);
            gl::UniformMatrix4fv(self.idx_projection as i32, 1, gl::FALSE, &ortho_projection[0]);
            gl::BindVertexArray(self.vao_handle);

            // Generate render commands
            igRender();

            // Execute render commands
            let draw_data = igGetDrawData();
            for i in 0..(*draw_data).CmdListsCount {
                let cmd_list = (*draw_data).CmdLists.offset(i as isize);
                gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo_handle);
                gl::BufferData(gl::ARRAY_BUFFER,
                    ImDrawList_GetVertexBufferSize(*cmd_list) as isize * size_of::<ImDrawVert>() as isize,
                    ImDrawList_GetVertexPtr(*cmd_list, 0) as *const raw::c_void,
                    gl::STREAM_DRAW
                );
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.elements_handle);
                gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                    ImDrawList_GetIndexBufferSize(*cmd_list) as isize * size_of::<ImDrawIdx>() as isize,
                    ImDrawList_GetIndexPtr(*cmd_list, 0) as *const raw::c_void,
                    gl::STREAM_DRAW
                );

                // Handle commands
                let mut ptr: *const u16 = ptr::null_mut();
                for j in 0..ImDrawList_GetCmdSize(*cmd_list) {
                    let cmd = ImDrawList_GetCmdPtr(*cmd_list, j);
                    let elements = (*cmd).ElemCount as i32;

                    // Handle user-defined callback
                    match (*cmd).UserCallback {
                        Some(callback) => callback(*cmd_list, cmd),
                        None => {
                            gl::BindTexture(gl::TEXTURE_2D, (*cmd).TextureId as u32);
                            gl::Scissor((*cmd).ClipRect.x as i32, (h - (*cmd).ClipRect.w) as i32, ((*cmd).ClipRect.z - (*cmd).ClipRect.x) as i32, ((*cmd).ClipRect.w - (*cmd).ClipRect.y) as i32);
                            gl::DrawElements(gl::TRIANGLES, elements, gl::UNSIGNED_SHORT, ptr as *const _);
                        }
                    }
                    ptr = ptr.offset(elements as isize)
                }
            }

            // Restore modified GL state
            gl::UseProgram(last_program as u32);
            gl::BindTexture(gl::TEXTURE_2D, last_texture as u32);
            gl::BindBuffer(gl::ARRAY_BUFFER, last_array_buffer as u32);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, last_element_array_buffer as u32);
            gl::BindVertexArray(last_vertex_array as u32);
            gl::BlendEquationSeparate(last_blend_equation_rgb as u32, last_blend_equation_alpha as u32);
            gl::BlendFunc(last_blend_src as u32, last_blend_dst as u32);
            if last_enable_blend == 1 { gl::Enable(gl::BLEND); } else { gl::Disable(gl::BLEND) };
            if last_enable_cull_face == 1 { gl::Enable(gl::CULL_FACE); } else { gl::Disable(gl::CULL_FACE) };
            if last_enable_depth_test == 1 { gl::Enable(gl::DEPTH_TEST); } else { gl::Disable(gl::DEPTH_TEST) };
            if last_enable_scissor_test == 1 { gl::Enable(gl::SCISSOR_TEST); } else { gl::Disable(gl::SCISSOR_TEST) };
            gl::Viewport(last_viewport[0], last_viewport[1], last_viewport[2], last_viewport[3]);

        }
    }
}
