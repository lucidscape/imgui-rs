#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
use libc::{c_void, c_char, c_int, c_uint, c_ushort, c_float, c_uchar, c_long, size_t};
use std::mem;

pub type ptrdiff_t = c_long;
pub type ImU32 = c_uint;
pub type ImWchar = c_ushort;
pub type ImTextureID = *mut c_void;
pub type ImGuiID = ImU32;
pub type ImGuiCol = c_int;
pub type ImGuiStyleVar = c_int;
pub type ImGuiKey = c_int;
pub type ImGuiAlign = c_int;
pub type ImGuiColorEditMode = c_int;
pub type ImGuiMouseCursor = c_int;
pub type ImGuiWindowFlags = c_int;
pub type ImGuiSetCond = c_int;
pub type ImGuiInputTextFlags = c_int;
pub type ImGuiSelectableFlags = c_int;
pub type ImGuiTextEditCallback = Option<unsafe extern "C" fn(data: *mut ImGuiTextEditCallbackData) -> c_int>;

pub type Enum_ImGuiWindowFlags_ = c_uint;
pub const ImGuiWindowFlags_NoTitleBar: c_uint = 1;
pub const ImGuiWindowFlags_NoResize: c_uint = 2;
pub const ImGuiWindowFlags_NoMove: c_uint = 4;
pub const ImGuiWindowFlags_NoScrollbar: c_uint = 8;
pub const ImGuiWindowFlags_NoScrollWithMouse: c_uint = 16;
pub const ImGuiWindowFlags_NoCollapse: c_uint = 32;
pub const ImGuiWindowFlags_AlwaysAutoResize: c_uint = 64;
pub const ImGuiWindowFlags_ShowBorders: c_uint = 128;
pub const ImGuiWindowFlags_NoSavedSettings: c_uint = 256;
pub const ImGuiWindowFlags_NoInputs: c_uint = 512;
pub const ImGuiWindowFlags_MenuBar: c_uint = 1024;
pub const ImGuiWindowFlags_HorizontalScrollbar: c_uint = 2048;
pub const ImGuiWindowFlags_NoFocusOnAppearing: c_uint = 4096;
pub const ImGuiWindowFlags_NoBringToFrontOnFocus: c_uint = 8192;
pub const ImGuiWindowFlags_ChildWindow: c_uint = 1048576;
pub const ImGuiWindowFlags_ChildWindowAutoFitX: c_uint = 2097152;
pub const ImGuiWindowFlags_ChildWindowAutoFitY: c_uint = 4194304;
pub const ImGuiWindowFlags_ComboBox: c_uint = 8388608;
pub const ImGuiWindowFlags_Tooltip: c_uint = 16777216;
pub const ImGuiWindowFlags_Popup: c_uint = 33554432;
pub const ImGuiWindowFlags_Modal: c_uint = 67108864;
pub const ImGuiWindowFlags_ChildMenu: c_uint = 134217728;

pub type Enum_ImGuiInputTextFlags_ = c_uint;
pub const ImGuiInputTextFlags_CharsDecimal: c_uint = 1;
pub const ImGuiInputTextFlags_CharsHexadecimal: c_uint = 2;
pub const ImGuiInputTextFlags_CharsUppercase: c_uint = 4;
pub const ImGuiInputTextFlags_CharsNoBlank: c_uint = 8;
pub const ImGuiInputTextFlags_AutoSelectAll: c_uint = 16;
pub const ImGuiInputTextFlags_EnterReturnsTrue: c_uint = 32;
pub const ImGuiInputTextFlags_CallbackCompletion: c_uint = 64;
pub const ImGuiInputTextFlags_CallbackHistory: c_uint = 128;
pub const ImGuiInputTextFlags_CallbackAlways: c_uint = 256;
pub const ImGuiInputTextFlags_CallbackCharFilter: c_uint = 512;
pub const ImGuiInputTextFlags_AllowTabInput: c_uint = 1024;
pub const ImGuiInputTextFlags_CtrlEnterForNewLine: c_uint = 2048;
pub const ImGuiInputTextFlags_NoHorizontalScroll: c_uint = 4096;
pub const ImGuiInputTextFlags_AlwaysInsertMode: c_uint = 8192;
pub const ImGuiInputTextFlags_ReadOnly: c_uint = 16384;
pub const ImGuiInputTextFlags_Password: c_uint = 32768;
pub const ImGuiInputTextFlags_Multiline: c_uint = 1048576;

pub type Enum_ImGuiSelectableFlags_ = c_uint;
pub const ImGuiSelectableFlags_DontClosePopups: c_uint = 1;
pub const ImGuiSelectableFlags_SpanAllColumns: c_uint = 2;

pub type Enum_ImGuiKey_ = c_uint;
pub const ImGuiKey_Tab: c_uint = 0;
pub const ImGuiKey_LeftArrow: c_uint = 1;
pub const ImGuiKey_RightArrow: c_uint = 2;
pub const ImGuiKey_UpArrow: c_uint = 3;
pub const ImGuiKey_DownArrow: c_uint = 4;
pub const ImGuiKey_PageUp: c_uint = 5;
pub const ImGuiKey_PageDown: c_uint = 6;
pub const ImGuiKey_Home: c_uint = 7;
pub const ImGuiKey_End: c_uint = 8;
pub const ImGuiKey_Delete: c_uint = 9;
pub const ImGuiKey_Backspace: c_uint = 10;
pub const ImGuiKey_Enter: c_uint = 11;
pub const ImGuiKey_Escape: c_uint = 12;
pub const ImGuiKey_A: c_uint = 13;
pub const ImGuiKey_C: c_uint = 14;
pub const ImGuiKey_V: c_uint = 15;
pub const ImGuiKey_X: c_uint = 16;
pub const ImGuiKey_Y: c_uint = 17;
pub const ImGuiKey_Z: c_uint = 18;
pub const ImGuiKey_COUNT: c_uint = 19;

pub type Enum_ImGuiCol_ = c_uint;
pub const ImGuiCol_Text: c_uint = 0;
pub const ImGuiCol_TextDisabled: c_uint = 1;
pub const ImGuiCol_WindowBg: c_uint = 2;
pub const ImGuiCol_ChildWindowBg: c_uint = 3;
pub const ImGuiCol_Border: c_uint = 4;
pub const ImGuiCol_BorderShadow: c_uint = 5;
pub const ImGuiCol_FrameBg: c_uint = 6;
pub const ImGuiCol_FrameBgHovered: c_uint = 7;
pub const ImGuiCol_FrameBgActive: c_uint = 8;
pub const ImGuiCol_TitleBg: c_uint = 9;
pub const ImGuiCol_TitleBgCollapsed: c_uint = 10;
pub const ImGuiCol_TitleBgActive: c_uint = 11;
pub const ImGuiCol_MenuBarBg: c_uint = 12;
pub const ImGuiCol_ScrollbarBg: c_uint = 13;
pub const ImGuiCol_ScrollbarGrab: c_uint = 14;
pub const ImGuiCol_ScrollbarGrabHovered: c_uint = 15;
pub const ImGuiCol_ScrollbarGrabActive: c_uint = 16;
pub const ImGuiCol_ComboBg: c_uint = 17;
pub const ImGuiCol_CheckMark: c_uint = 18;
pub const ImGuiCol_SliderGrab: c_uint = 19;
pub const ImGuiCol_SliderGrabActive: c_uint = 20;
pub const ImGuiCol_Button: c_uint = 21;
pub const ImGuiCol_ButtonHovered: c_uint = 22;
pub const ImGuiCol_ButtonActive: c_uint = 23;
pub const ImGuiCol_Header: c_uint = 24;
pub const ImGuiCol_HeaderHovered: c_uint = 25;
pub const ImGuiCol_HeaderActive: c_uint = 26;
pub const ImGuiCol_Column: c_uint = 27;
pub const ImGuiCol_ColumnHovered: c_uint = 28;
pub const ImGuiCol_ColumnActive: c_uint = 29;
pub const ImGuiCol_ResizeGrip: c_uint = 30;
pub const ImGuiCol_ResizeGripHovered: c_uint = 31;
pub const ImGuiCol_ResizeGripActive: c_uint = 32;
pub const ImGuiCol_CloseButton: c_uint = 33;
pub const ImGuiCol_CloseButtonHovered: c_uint = 34;
pub const ImGuiCol_CloseButtonActive: c_uint = 35;
pub const ImGuiCol_PlotLines: c_uint = 36;
pub const ImGuiCol_PlotLinesHovered: c_uint = 37;
pub const ImGuiCol_PlotHistogram: c_uint = 38;
pub const ImGuiCol_PlotHistogramHovered: c_uint = 39;
pub const ImGuiCol_TextSelectedBg: c_uint = 40;
pub const ImGuiCol_TooltipBg: c_uint = 41;
pub const ImGuiCol_ModalWindowDarkening: c_uint = 42;
pub const ImGuiCol_COUNT: c_uint = 43;

pub type Enum_ImGuiStyleVar_ = c_uint;
pub const ImGuiStyleVar_Alpha: c_uint = 0;
pub const ImGuiStyleVar_WindowPadding: c_uint = 1;
pub const ImGuiStyleVar_WindowRounding: c_uint = 2;
pub const ImGuiStyleVar_WindowMinSize: c_uint = 3;
pub const ImGuiStyleVar_ChildWindowRounding: c_uint = 4;
pub const ImGuiStyleVar_FramePadding: c_uint = 5;
pub const ImGuiStyleVar_FrameRounding: c_uint = 6;
pub const ImGuiStyleVar_ItemSpacing: c_uint = 7;
pub const ImGuiStyleVar_ItemInnerSpacing: c_uint = 8;
pub const ImGuiStyleVar_IndentSpacing: c_uint = 9;
pub const ImGuiStyleVar_GrabMinSize: c_uint = 10;

pub type Enum_ImGuiAlign_ = c_uint;
pub const ImGuiAlign_Left: c_uint = 1;
pub const ImGuiAlign_Center: c_uint = 2;
pub const ImGuiAlign_Right: c_uint = 4;
pub const ImGuiAlign_Top: c_uint = 8;
pub const ImGuiAlign_VCenter: c_uint = 16;
pub const ImGuiAlign_Default: c_uint = 9;

pub type Enum_ImGuiColorEditMode_ = c_int;
pub const ImGuiColorEditMode_UserSelect: c_int = -2;
pub const ImGuiColorEditMode_UserSelectShowButton: c_int = -1;
pub const ImGuiColorEditMode_RGB: c_int = 0;
pub const ImGuiColorEditMode_HSV: c_int = 1;
pub const ImGuiColorEditMode_HEX: c_int = 2;

pub type Enum_ImGuiMouseCursor_ = c_uint;
pub const ImGuiMouseCursor_Arrow: c_uint = 0;
pub const ImGuiMouseCursor_TextInput: c_uint = 1;
pub const ImGuiMouseCursor_Move: c_uint = 2;
pub const ImGuiMouseCursor_ResizeNS: c_uint = 3;
pub const ImGuiMouseCursor_ResizeEW: c_uint = 4;
pub const ImGuiMouseCursor_ResizeNESW: c_uint = 5;
pub const ImGuiMouseCursor_ResizeNWSE: c_uint = 6;
pub const ImGuiMouseCursor_Count_: c_uint = 7;

pub type Enum_ImGuiSetCond_ = c_uint;
pub const ImGuiSetCond_Always: c_uint = 1;
pub const ImGuiSetCond_Once: c_uint = 2;
pub const ImGuiSetCond_FirstUseEver: c_uint = 4;
pub const ImGuiSetCond_Appearing: c_uint = 8;

#[repr(C)]
#[derive(Copy)]
pub struct ImVector<T> {
    pub Size: c_int,
    pub Capacity: c_int,
    pub Data: *mut T,
}
impl<T> Clone for ImVector<T> {
    fn clone(&self) -> Self {
        panic!("TODO")
    }
}
impl<T> Default for ImVector<T> {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct ImVec2 {
    pub x: c_float,
    pub y: c_float,
}
impl Clone for ImVec2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for ImVec2 {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct ImVec4 {
    pub x: c_float,
    pub y: c_float,
    pub z: c_float,
    pub w: c_float,
}
impl Clone for ImVec4 {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for ImVec4 {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct ImGuiStyle {
    pub Alpha: c_float,
    pub WindowPadding: ImVec2,
    pub WindowMinSize: ImVec2,
    pub WindowRounding: c_float,
    pub WindowTitleAlign: ImGuiAlign,
    pub ChildWindowRounding: c_float,
    pub FramePadding: ImVec2,
    pub FrameRounding: c_float,
    pub ItemSpacing: ImVec2,
    pub ItemInnerSpacing: ImVec2,
    pub TouchExtraPadding: ImVec2,
    pub WindowFillAlphaDefault: c_float,
    pub IndentSpacing: c_float,
    pub ColumnsMinSpacing: c_float,
    pub ScrollbarSize: c_float,
    pub ScrollbarRounding: c_float,
    pub GrabMinSize: c_float,
    pub GrabRounding: c_float,
    pub DisplayWindowPadding: ImVec2,
    pub DisplaySafeAreaPadding: ImVec2,
    pub AntiAliasedLines: u8,
    pub AntiAliasedShapes: u8,
    pub CurveTessellationTol: c_float,
    pub Colors: [ImVec4; 43usize],
}
impl Clone for ImGuiStyle {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for ImGuiStyle {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct ImGuiIO {
    pub DisplaySize: ImVec2,
    pub DeltaTime: c_float,
    pub IniSavingRate: c_float,
    pub IniFilename: *const c_char,
    pub LogFilename: *const c_char,
    pub MouseDoubleClickTime: c_float,
    pub MouseDoubleClickMaxDist: c_float,
    pub MouseDragThreshold: c_float,
    pub KeyMap: [c_int; 19usize],
    pub KeyRepeatDelay: c_float,
    pub KeyRepeatRate: c_float,
    pub UserData: *mut c_void,
    pub Fonts: *mut ImFontAtlas,
    pub FontGlobalScale: c_float,
    pub FontAllowUserScaling: u8,
    pub DisplayFramebufferScale: ImVec2,
    pub DisplayVisibleMin: ImVec2,
    pub DisplayVisibleMax: ImVec2,
    pub RenderDrawListsFn: Option<unsafe extern "C" fn(data: *mut ImDrawData) -> ()>,
    pub GetClipboardTextFn: Option<extern "C" fn() -> *const c_char>,
    pub SetClipboardTextFn: Option<unsafe extern "C" fn(text: *const c_char) -> ()>,
    pub MemAllocFn: Option<extern "C" fn(sz: size_t) -> *mut c_void>,
    pub MemFreeFn: Option<unsafe extern "C" fn(ptr: *mut c_void) -> ()>,
    pub ImeSetInputScreenPosFn: Option<extern "C" fn(x: c_int, y: c_int) -> ()>,
    pub ImeWindowHandle: *mut c_void,
    pub MousePos: ImVec2,
    pub MouseDown: [u8; 5usize],
    pub MouseWheel: c_float,
    pub MouseDrawCursor: u8,
    pub KeyCtrl: u8,
    pub KeyShift: u8,
    pub KeyAlt: u8,
    pub KeysDown: [u8; 512usize],
    pub InputCharacters: [ImWchar; 17usize],
    pub WantCaptureMouse: u8,
    pub WantCaptureKeyboard: u8,
    pub WantTextInput: u8,
    pub Framerate: c_float,
    pub MetricsAllocs: c_int,
    pub MetricsRenderVertices: c_int,
    pub MetricsRenderIndices: c_int,
    pub MetricsActiveWindows: c_int,
    pub MousePosPrev: ImVec2,
    pub MouseDelta: ImVec2,
    pub MouseClicked: [u8; 5usize],
    pub MouseClickedPos: [ImVec2; 5usize],
    pub MouseClickedTime: [c_float; 5usize],
    pub MouseDoubleClicked: [u8; 5usize],
    pub MouseReleased: [u8; 5usize],
    pub MouseDownOwned: [u8; 5usize],
    pub MouseDownDuration: [c_float; 5usize],
    pub MouseDownDurationPrev: [c_float; 5usize],
    pub MouseDragMaxDistanceSqr: [c_float; 5usize],
    pub KeysDownDuration: [c_float; 512usize],
    pub KeysDownDurationPrev: [c_float; 512usize],
}
impl Clone for ImGuiIO {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for ImGuiIO {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct ImGuiOnceUponAFrame {
    pub RefFrame: c_int,
}
impl Clone for ImGuiOnceUponAFrame {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for ImGuiOnceUponAFrame {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct ImGuiTextFilter {
    pub InputBuf: [c_char; 256usize],
    pub Filters: ImVector<Struct_TextRange>,
    pub CountGrep: c_int,
}
impl Clone for ImGuiTextFilter {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for ImGuiTextFilter {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct Struct_TextRange {
    pub b: *const c_char,
    pub e: *const c_char,
}
impl Clone for Struct_TextRange {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for Struct_TextRange {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct ImGuiTextBuffer {
    pub Buf: ImVector<c_char>,
}
impl Clone for ImGuiTextBuffer {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for ImGuiTextBuffer {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct ImGuiStorage {
    pub Data: ImVector<Struct_Pair>,
}
impl Clone for ImGuiStorage {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for ImGuiStorage {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct Struct_Pair {
    pub key: ImGuiID,
    pub _bindgen_data_1_: [u64; 1usize],
}
impl Struct_Pair {
    pub unsafe fn val_i(&mut self) -> *mut c_int {
        let raw: *mut u8 = mem::transmute(&self._bindgen_data_1_);
        mem::transmute(raw.offset(0))
    }
    pub unsafe fn val_f(&mut self) -> *mut c_float {
        let raw: *mut u8 = mem::transmute(&self._bindgen_data_1_);
        mem::transmute(raw.offset(0))
    }
    pub unsafe fn val_p(&mut self) -> *mut *mut c_void {
        let raw: *mut u8 = mem::transmute(&self._bindgen_data_1_);
        mem::transmute(raw.offset(0))
    }
}
impl Clone for Struct_Pair {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for Struct_Pair {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct ImGuiTextEditCallbackData {
    pub EventFlag: ImGuiInputTextFlags,
    pub Flags: ImGuiInputTextFlags,
    pub UserData: *mut c_void,
    pub ReadOnly: u8,
    pub EventChar: ImWchar,
    pub EventKey: ImGuiKey,
    pub Buf: *mut c_char,
    pub BufSize: c_int,
    pub BufDirty: u8,
    pub CursorPos: c_int,
    pub SelectionStart: c_int,
    pub SelectionEnd: c_int,
}
impl Clone for ImGuiTextEditCallbackData {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for ImGuiTextEditCallbackData {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct ImColor {
    pub Value: ImVec4,
}
impl Clone for ImColor {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for ImColor {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct ImGuiListClipper {
    pub ItemsHeight: c_float,
    pub ItemsCount: c_int,
    pub DisplayStart: c_int,
    pub DisplayEnd: c_int,
}
impl Clone for ImGuiListClipper {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for ImGuiListClipper {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

pub type ImDrawCallback = Option<unsafe extern "C" fn(parent_list: *const ImDrawList, cmd: *const ImDrawCmd) -> ()>;

#[repr(C)]
#[derive(Copy)]
pub struct ImDrawCmd {
    pub ElemCount: c_uint,
    pub ClipRect: ImVec4,
    pub TextureId: ImTextureID,
    pub UserCallback: ImDrawCallback,
    pub UserCallbackData: *mut c_void,
}
impl Clone for ImDrawCmd {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for ImDrawCmd {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

pub type ImDrawIdx = c_ushort;

#[repr(C)]
#[derive(Copy)]
pub struct ImDrawVert {
    pub pos: ImVec2,
    pub uv: ImVec2,
    pub col: ImU32,
}
impl Clone for ImDrawVert {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for ImDrawVert {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct ImDrawChannel {
    pub CmdBuffer: ImVector<ImDrawCmd>,
    pub IdxBuffer: ImVector<ImDrawIdx>,
}
impl Clone for ImDrawChannel {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for ImDrawChannel {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct ImDrawList {
    pub CmdBuffer: ImVector<ImDrawCmd>,
    pub IdxBuffer: ImVector<ImDrawIdx>,
    pub VtxBuffer: ImVector<ImDrawVert>,
    pub _OwnerName: *const c_char,
    pub _VtxCurrentIdx: c_uint,
    pub _VtxWritePtr: *mut ImDrawVert,
    pub _IdxWritePtr: *mut ImDrawIdx,
    pub _ClipRectStack: ImVector<ImVec4>,
    pub _TextureIdStack: ImVector<ImTextureID>,
    pub _Path: ImVector<ImVec2>,
    pub _ChannelsCurrent: c_int,
    pub _ChannelsCount: c_int,
    pub _Channels: ImVector<ImDrawChannel>,
}
impl Clone for ImDrawList {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for ImDrawList {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct ImDrawData {
    pub Valid: u8,
    pub CmdLists: *mut *mut ImDrawList,
    pub CmdListsCount: c_int,
    pub TotalVtxCount: c_int,
    pub TotalIdxCount: c_int,
}
impl Clone for ImDrawData {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for ImDrawData {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct ImFontConfig {
    pub FontData: *mut c_void,
    pub FontDataSize: c_int,
    pub FontDataOwnedByAtlas: u8,
    pub FontNo: c_int,
    pub SizePixels: c_float,
    pub OversampleH: c_int,
    pub OversampleV: c_int,
    pub PixelSnapH: u8,
    pub GlyphExtraSpacing: ImVec2,
    pub GlyphRanges: *const ImWchar,
    pub MergeMode: u8,
    pub MergeGlyphCenterV: u8,
    pub Name: [c_char; 32usize],
    pub DstFont: *mut ImFont,
}
impl Clone for ImFontConfig {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for ImFontConfig {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct ImFontAtlas {
    pub TexID: *mut c_void,
    pub TexPixelsAlpha8: *mut c_uchar,
    pub TexPixelsRGBA32: *mut c_uint,
    pub TexWidth: c_int,
    pub TexHeight: c_int,
    pub TexDesiredWidth: c_int,
    pub TexUvWhitePixel: ImVec2,
    pub Fonts: ImVector<*const ImFont>,
    pub ConfigData: ImVector<ImFontConfig>,
}
impl Clone for ImFontAtlas {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for ImFontAtlas {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct ImFont {
    pub FontSize: c_float,
    pub Scale: c_float,
    pub DisplayOffset: ImVec2,
    pub FallbackChar: ImWchar,
    pub ConfigData: *mut ImFontConfig,
    pub ConfigDataCount: c_int,
    pub Ascent: c_float,
    pub Descent: c_float,
    pub ContainerAtlas: *mut ImFontAtlas,
    pub Glyphs: ImVector<Struct_Glyph>,
    pub FallbackGlyph: *const Struct_Glyph,
    pub FallbackXAdvance: c_float,
    pub IndexXAdvance: ImVector<c_float>,
    pub IndexLookup: ImVector<c_int>,
}
impl Clone for ImFont {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for ImFont {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct Struct_Glyph {
    pub Codepoint: ImWchar,
    pub XAdvance: c_float,
    pub X0: c_float,
    pub Y0: c_float,
    pub X1: c_float,
    pub Y1: c_float,
    pub U0: c_float,
    pub V0: c_float,
    pub U1: c_float,
    pub V1: c_float,
}
impl Clone for Struct_Glyph {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for Struct_Glyph {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[link(name = "imgui")]
extern "C" {
    pub fn igGetIO() -> *mut ImGuiIO;
    pub fn igGetStyle() -> *mut ImGuiStyle;
    pub fn igGetDrawData() -> *mut ImDrawData;
    pub fn igNewFrame() -> ();
    pub fn igRender() -> ();
    pub fn igShutdown() -> ();
    pub fn igShowUserGuide() -> ();
    pub fn igShowStyleEditor(_ref: *mut ImGuiStyle) -> ();
    pub fn igShowTestWindow(opened: *mut u8) -> ();
    pub fn igShowMetricsWindow(opened: *mut u8) -> ();
    pub fn igBegin(name: *const c_char, p_opened: *mut u8, flags: ImGuiWindowFlags) -> u8;
    pub fn igBegin2(name: *const c_char, p_opened: *mut u8, size_on_first_use: ImVec2, bg_alpha: c_float, flags: ImGuiWindowFlags) -> u8;
    pub fn igEnd() -> ();
    pub fn igBeginChild(str_id: *const c_char, size: ImVec2, border: u8, extra_flags: ImGuiWindowFlags) -> u8;
    pub fn igBeginChildEx(id: ImGuiID, size: ImVec2, border: u8, extra_flags: ImGuiWindowFlags) -> u8;
    pub fn igEndChild() -> ();
    pub fn igGetContentRegionMax(out: *mut ImVec2) -> ();
    pub fn igGetContentRegionAvail(out: *mut ImVec2) -> ();
    pub fn igGetContentRegionAvailWidth() -> c_float;
    pub fn igGetWindowContentRegionMin(out: *mut ImVec2) -> ();
    pub fn igGetWindowContentRegionMax(out: *mut ImVec2) -> ();
    pub fn igGetWindowContentRegionWidth() -> c_float;
    pub fn igGetWindowDrawList() -> *mut ImDrawList;
    pub fn igGetWindowFont() -> *mut ImFont;
    pub fn igGetWindowFontSize() -> c_float;
    pub fn igSetWindowFontScale(scale: c_float) -> ();
    pub fn igGetWindowPos(out: *mut ImVec2) -> ();
    pub fn igGetWindowSize(out: *mut ImVec2) -> ();
    pub fn igGetWindowWidth() -> c_float;
    pub fn igGetWindowHeight() -> c_float;
    pub fn igIsWindowCollapsed() -> u8;
    pub fn igSetNextWindowPos(pos: ImVec2, cond: ImGuiSetCond) -> ();
    pub fn igSetNextWindowPosCenter(cond: ImGuiSetCond) -> ();
    pub fn igSetNextWindowSize(size: ImVec2, cond: ImGuiSetCond) -> ();
    pub fn igSetNextWindowContentSize(size: ImVec2) -> ();
    pub fn igSetNextWindowContentWidth(width: c_float) -> ();
    pub fn igSetNextWindowCollapsed(collapsed: u8, cond: ImGuiSetCond) -> ();
    pub fn igSetNextWindowFocus() -> ();
    pub fn igSetWindowPos(pos: ImVec2, cond: ImGuiSetCond) -> ();
    pub fn igSetWindowSize(size: ImVec2, cond: ImGuiSetCond) -> ();
    pub fn igSetWindowCollapsed(collapsed: u8, cond: ImGuiSetCond) -> ();
    pub fn igSetWindowFocus() -> ();
    pub fn igSetWindowPosByName(name: *const c_char, pos: ImVec2, cond: ImGuiSetCond) -> ();
    pub fn igSetWindowSize2(name: *const c_char, size: ImVec2, cond: ImGuiSetCond) -> ();
    pub fn igSetWindowCollapsed2(name: *const c_char, collapsed: u8, cond: ImGuiSetCond) -> ();
    pub fn igSetWindowFocus2(name: *const c_char) -> ();
    pub fn igGetScrollX() -> c_float;
    pub fn igGetScrollY() -> c_float;
    pub fn igGetScrollMaxX() -> c_float;
    pub fn igGetScrollMaxY() -> c_float;
    pub fn igSetScrollX(scroll_x: c_float) -> ();
    pub fn igSetScrollY(scroll_y: c_float) -> ();
    pub fn igSetScrollHere(center_y_ratio: c_float) -> ();
    pub fn igSetScrollFromPosY(pos_y: c_float, center_y_ratio: c_float) -> ();
    pub fn igSetKeyboardFocusHere(offset: c_int) -> ();
    pub fn igSetStateStorage(tree: *mut ImGuiStorage) -> ();
    pub fn igGetStateStorage() -> *mut ImGuiStorage;
    pub fn igPushFont(font: *mut ImFont) -> ();
    pub fn igPopFont() -> ();
    pub fn igPushStyleColor(idx: ImGuiCol, col: ImVec4) -> ();
    pub fn igPopStyleColor(count: c_int) -> ();
    pub fn igPushStyleVar(idx: ImGuiStyleVar, val: c_float) -> ();
    pub fn igPushStyleVarVec(idx: ImGuiStyleVar, val: ImVec2) -> ();
    pub fn igPopStyleVar(count: c_int) -> ();
    pub fn igGetColorU32(idx: ImGuiCol, alpha_mul: c_float) -> ImU32;
    pub fn igGetColorU32Vec(col: *const ImVec4) -> ImU32;
    pub fn igPushItemWidth(item_width: c_float) -> ();
    pub fn igPopItemWidth() -> ();
    pub fn igCalcItemWidth() -> c_float;
    pub fn igPushTextWrapPos(wrap_pos_x: c_float) -> ();
    pub fn igPopTextWrapPos() -> ();
    pub fn igPushAllowKeyboardFocus(v: u8) -> ();
    pub fn igPopAllowKeyboardFocus() -> ();
    pub fn igPushButtonRepeat(repeat: u8) -> ();
    pub fn igPopButtonRepeat() -> ();
    pub fn igBeginGroup() -> ();
    pub fn igEndGroup() -> ();
    pub fn igSeparator() -> ();
    pub fn igSameLine(local_pos_x: c_float, spacing_w: c_float) -> ();
    pub fn igSpacing() -> ();
    pub fn igDummy(size: *const ImVec2) -> ();
    pub fn igIndent() -> ();
    pub fn igUnindent() -> ();
    pub fn igColumns(count: c_int, id: *const c_char, border: u8) -> ();
    pub fn igNextColumn() -> ();
    pub fn igGetColumnIndex() -> c_int;
    pub fn igGetColumnOffset(column_index: c_int) -> c_float;
    pub fn igSetColumnOffset(column_index: c_int, offset_x: c_float) -> ();
    pub fn igGetColumnWidth(column_index: c_int) -> c_float;
    pub fn igGetColumnsCount() -> c_int;
    pub fn igGetCursorPos(pOut: *mut ImVec2) -> ();
    pub fn igGetCursorPosX() -> c_float;
    pub fn igGetCursorPosY() -> c_float;
    pub fn igSetCursorPos(local_pos: ImVec2) -> ();
    pub fn igSetCursorPosX(x: c_float) -> ();
    pub fn igSetCursorPosY(y: c_float) -> ();
    pub fn igGetCursorStartPos(pOut: *mut ImVec2) -> ();
    pub fn igGetCursorScreenPos(pOut: *mut ImVec2) -> ();
    pub fn igSetCursorScreenPos(pos: ImVec2) -> ();
    pub fn igAlignFirstTextHeightToWidgets() -> ();
    pub fn igGetTextLineHeight() -> c_float;
    pub fn igGetTextLineHeightWithSpacing() -> c_float;
    pub fn igGetItemsLineHeightWithSpacing() -> c_float;
    pub fn igPushIdStr(str_id: *const c_char) -> ();
    pub fn igPushIdStrRange(str_begin: *const c_char, str_end: *const c_char) -> ();
    pub fn igPushIdPtr(ptr_id: *const c_void) -> ();
    pub fn igPushIdInt(int_id: c_int) -> ();
    pub fn igPopId() -> ();
    pub fn igGetIdStr(str_id: *const c_char) -> ImGuiID;
    pub fn igGetIdStrRange(str_begin: *const c_char, str_end: *const c_char) -> ImGuiID;
    pub fn igGetIdPtr(ptr_id: *const c_void) -> ImGuiID;
    pub fn igText(fmt: *const c_char, ...) -> ();
    pub fn igTextColored(col: ImVec4, fmt: *const c_char, ...) -> ();
    pub fn igTextDisabled(fmt: *const c_char, ...) -> ();
    pub fn igTextWrapped(fmt: *const c_char, ...) -> ();
    pub fn igTextUnformatted(text: *const c_char, text_end: *const c_char) -> ();
    pub fn igLabelText(label: *const c_char, fmt: *const c_char, ...) -> ();
    pub fn igBullet() -> ();
    pub fn igBulletText(fmt: *const c_char, ...) -> ();
    pub fn igButton(label: *const c_char, size: ImVec2) -> u8;
    pub fn igSmallButton(label: *const c_char) -> u8;
    pub fn igInvisibleButton(str_id: *const c_char, size: ImVec2) -> u8;
    pub fn igImage(user_texture_id: ImTextureID, size: ImVec2, uv0: ImVec2, uv1: ImVec2, tint_col: ImVec4, border_col: ImVec4) -> ();
    pub fn igImageButton(user_texture_id: ImTextureID, size: ImVec2, uv0: ImVec2, uv1: ImVec2, frame_padding: c_int, bg_col: ImVec4, tint_col: ImVec4) -> u8;
    pub fn igCollapsingHeader(label: *const c_char, str_id: *const c_char, display_frame: u8, default_open: u8) -> u8;
    pub fn igCheckbox(label: *const c_char, v: *mut u8) -> u8;
    pub fn igCheckboxFlags(label: *const c_char, flags: *mut c_uint, flags_value: c_uint) -> u8;
    pub fn igRadioButtonBool(label: *const c_char, active: u8) -> u8;
    pub fn igRadioButton(label: *const c_char, v: *mut c_int, v_button: c_int) -> u8;
    pub fn igCombo(label: *const c_char, current_item: *mut c_int, items: *mut *const c_char, items_count: c_int, height_in_items: c_int) -> u8;
    pub fn igCombo2(label: *const c_char, current_item: *mut c_int, items_separated_by_zeros: *const c_char, height_in_items: c_int) -> u8;
    pub fn igCombo3(label: *const c_char, current_item: *mut c_int, items_getter: Option<unsafe extern "C" fn(data: *mut c_void, idx: c_int, out_text: *mut *const c_char) -> u8>, data: *mut c_void, items_count: c_int, height_in_items: c_int) -> u8;
    pub fn igColorButton(col: ImVec4, small_height: u8, outline_border: u8) -> u8;
    pub fn igColorEdit3(label: *const c_char, col: *mut c_float) -> u8;
    pub fn igColorEdit4(label: *const c_char, col: *mut c_float, show_alpha: u8) -> u8;
    pub fn igColorEditMode(mode: ImGuiColorEditMode) -> ();
    pub fn igPlotLines(label: *const c_char, values: *const c_float, values_count: c_int, values_offset: c_int, overlay_text: *const c_char, scale_min: c_float, scale_max: c_float, graph_size: ImVec2, stride: c_int) -> ();
    pub fn igPlotLines2(label: *const c_char, values_getter: Option<unsafe extern "C" fn(data: *mut c_void, idx: c_int) -> c_float>, data: *mut c_void, values_count: c_int, values_offset: c_int, overlay_text: *const c_char, scale_min: c_float, scale_max: c_float, graph_size: ImVec2) -> ();
    pub fn igPlotHistogram(label: *const c_char, values: *const c_float, values_count: c_int, values_offset: c_int, overlay_text: *const c_char, scale_min: c_float, scale_max: c_float, graph_size: ImVec2, stride: c_int) -> ();
    pub fn igPlotHistogram2(label: *const c_char, values_getter: Option<unsafe extern "C" fn(data: *mut c_void, idx: c_int) -> c_float>, data: *mut c_void, values_count: c_int, values_offset: c_int, overlay_text: *const c_char, scale_min: c_float, scale_max: c_float, graph_size: ImVec2) -> ();
    pub fn igProgressBar(fraction: c_float, size_arg: *const ImVec2, overlay: *const c_char) -> ();
    pub fn igSliderFloat(label: *const c_char, v: *mut c_float, v_min: c_float, v_max: c_float, display_format: *const c_char, power: c_float) -> u8;
    pub fn igSliderFloat2(label: *const c_char, v: *mut c_float, v_min: c_float, v_max: c_float, display_format: *const c_char, power: c_float) -> u8;
    pub fn igSliderFloat3(label: *const c_char, v: *mut c_float, v_min: c_float, v_max: c_float, display_format: *const c_char, power: c_float) -> u8;
    pub fn igSliderFloat4(label: *const c_char, v: *mut c_float, v_min: c_float, v_max: c_float, display_format: *const c_char, power: c_float) -> u8;
    pub fn igSliderAngle(label: *const c_char, v_rad: *mut c_float, v_degrees_min: c_float, v_degrees_max: c_float) -> u8;
    pub fn igSliderInt(label: *const c_char, v: *mut c_int, v_min: c_int, v_max: c_int, display_format: *const c_char) -> u8;
    pub fn igSliderInt2(label: *const c_char, v: *mut c_int, v_min: c_int, v_max: c_int, display_format: *const c_char) -> u8;
    pub fn igSliderInt3(label: *const c_char, v: *mut c_int, v_min: c_int, v_max: c_int, display_format: *const c_char) -> u8;
    pub fn igSliderInt4(label: *const c_char, v: *mut c_int, v_min: c_int, v_max: c_int, display_format: *const c_char) -> u8;
    pub fn igVSliderFloat(label: *const c_char, size: ImVec2, v: *mut c_float, v_min: c_float, v_max: c_float, display_format: *const c_char, power: c_float) -> u8;
    pub fn igVSliderInt(label: *const c_char, size: ImVec2, v: *mut c_int, v_min: c_int, v_max: c_int, display_format: *const c_char) -> u8;
    pub fn igDragFloat(label: *const c_char, v: *mut c_float, v_speed: c_float, v_min: c_float, v_max: c_float, display_format: *const c_char, power: c_float) -> u8;
    pub fn igDragFloat2(label: *const c_char, v: *mut c_float, v_speed: c_float, v_min: c_float, v_max: c_float, display_format: *const c_char, power: c_float) -> u8;
    pub fn igDragFloat3(label: *const c_char, v: *mut c_float, v_speed: c_float, v_min: c_float, v_max: c_float, display_format: *const c_char, power: c_float) -> u8;
    pub fn igDragFloat4(label: *const c_char, v: *mut c_float, v_speed: c_float, v_min: c_float, v_max: c_float, display_format: *const c_char, power: c_float) -> u8;
    pub fn igDragFloatRange2(label: *const c_char, v_current_min: *mut c_float, v_current_max: *mut c_float, v_speed: c_float, v_min: c_float, v_max: c_float, display_format: *const c_char, display_format_max: *const c_char, power: c_float) -> u8;
    pub fn igDragInt(label: *const c_char, v: *mut c_int, v_speed: c_float, v_min: c_int, v_max: c_int, display_format: *const c_char) -> u8;
    pub fn igDragInt2(label: *const c_char, v: *mut c_int, v_speed: c_float, v_min: c_int, v_max: c_int, display_format: *const c_char) -> u8;
    pub fn igDragInt3(label: *const c_char, v: *mut c_int, v_speed: c_float, v_min: c_int, v_max: c_int, display_format: *const c_char) -> u8;
    pub fn igDragInt4(label: *const c_char, v: *mut c_int, v_speed: c_float, v_min: c_int, v_max: c_int, display_format: *const c_char) -> u8;
    pub fn igDragIntRange2(label: *const c_char, v_current_min: *mut c_int, v_current_max: *mut c_int, v_speed: c_float, v_min: c_int, v_max: c_int, display_format: *const c_char, display_format_max: *const c_char) -> u8;
    pub fn igInputText(label: *const c_char, buf: *mut c_char, buf_size: size_t, flags: ImGuiInputTextFlags, callback: ImGuiTextEditCallback, user_data: *mut c_void) -> u8;
    pub fn igInputTextMultiline(label: *const c_char, buf: *mut c_char, buf_size: size_t, size: ImVec2, flags: ImGuiInputTextFlags, callback: ImGuiTextEditCallback, user_data: *mut c_void) -> u8;
    pub fn igInputFloat(label: *const c_char, v: *mut c_float, step: c_float, step_fast: c_float, decimal_precision: c_int, extra_flags: ImGuiInputTextFlags) -> u8;
    pub fn igInputFloat2(label: *const c_char, v: *mut c_float, decimal_precision: c_int, extra_flags: ImGuiInputTextFlags) -> u8;
    pub fn igInputFloat3(label: *const c_char, v: *mut c_float, decimal_precision: c_int, extra_flags: ImGuiInputTextFlags) -> u8;
    pub fn igInputFloat4(label: *const c_char, v: *mut c_float, decimal_precision: c_int, extra_flags: ImGuiInputTextFlags) -> u8;
    pub fn igInputInt(label: *const c_char, v: *mut c_int, step: c_int, step_fast: c_int, extra_flags: ImGuiInputTextFlags) -> u8;
    pub fn igInputInt2(label: *const c_char, v: *mut c_int, extra_flags: ImGuiInputTextFlags) -> u8;
    pub fn igInputInt3(label: *const c_char, v: *mut c_int, extra_flags: ImGuiInputTextFlags) -> u8;
    pub fn igInputInt4(label: *const c_char, v: *mut c_int, extra_flags: ImGuiInputTextFlags) -> u8;
    pub fn igTreeNode(str_label_id: *const c_char) -> u8;
    pub fn igTreeNodeStr(str_id: *const c_char, fmt: *const c_char, ...) -> u8;
    pub fn igTreeNodePtr(ptr_id: *const c_void, fmt: *const c_char, ...) -> u8;
    pub fn igTreePushStr(str_id: *const c_char) -> ();
    pub fn igTreePushPtr(ptr_id: *const c_void) -> ();
    pub fn igTreePop() -> ();
    pub fn igSetNextTreeNodeOpened(opened: u8, cond: ImGuiSetCond) -> ();
    pub fn igSelectable(label: *const c_char, selected: u8, flags: ImGuiSelectableFlags, size: ImVec2) -> u8;
    pub fn igSelectableEx(label: *const c_char, p_selected: *mut u8, flags: ImGuiSelectableFlags, size: ImVec2) -> u8;
    pub fn igListBox(label: *const c_char, current_item: *mut c_int, items: *mut *const c_char, items_count: c_int, height_in_items: c_int) -> u8;
    pub fn igListBox2(label: *const c_char, current_item: *mut c_int, items_getter: Option<unsafe extern "C" fn(data: *mut c_void, idx: c_int, out_text: *mut *const c_char) -> u8>, data: *mut c_void, items_count: c_int, height_in_items: c_int) -> u8;
    pub fn igListBoxHeader(label: *const c_char, size: ImVec2) -> u8;
    pub fn igListBoxHeader2(label: *const c_char, items_count: c_int, height_in_items: c_int) -> u8;
    pub fn igListBoxFooter() -> ();
    pub fn igValueBool(prefix: *const c_char, b: u8) -> ();
    pub fn igValueInt(prefix: *const c_char, v: c_int) -> ();
    pub fn igValueUInt(prefix: *const c_char, v: c_uint) -> ();
    pub fn igValueFloat(prefix: *const c_char, v: c_float, float_format: *const c_char) -> ();
    pub fn igValueColor(prefix: *const c_char, v: ImVec4) -> ();
    pub fn igValueColor2(prefix: *const c_char, v: c_uint) -> ();
    pub fn igSetTooltip(fmt: *const c_char, ...) -> ();
    pub fn igBeginTooltip() -> ();
    pub fn igEndTooltip() -> ();
    pub fn igBeginMainMenuBar() -> u8;
    pub fn igEndMainMenuBar() -> ();
    pub fn igBeginMenuBar() -> u8;
    pub fn igEndMenuBar() -> ();
    pub fn igBeginMenu(label: *const c_char, enabled: u8) -> u8;
    pub fn igEndMenu() -> ();
    pub fn igMenuItem(label: *const c_char, shortcut: *const c_char, selected: u8, enabled: u8) -> u8;
    pub fn igMenuItemPtr(label: *const c_char, shortcut: *const c_char, p_selected: *mut u8, enabled: u8) -> u8;
    pub fn igOpenPopup(str_id: *const c_char) -> ();
    pub fn igBeginPopup(str_id: *const c_char) -> u8;
    pub fn igBeginPopupModal(name: *const c_char, p_opened: *mut u8, extra_flags: ImGuiWindowFlags) -> u8;
    pub fn igBeginPopupContextItem(str_id: *const c_char, mouse_button: c_int) -> u8;
    pub fn igBeginPopupContextWindow(also_over_items: u8, str_id: *const c_char, mouse_button: c_int) -> u8;
    pub fn igBeginPopupContextVoid(str_id: *const c_char, mouse_button: c_int) -> u8;
    pub fn igEndPopup() -> ();
    pub fn igCloseCurrentPopup() -> ();
    pub fn igLogToTTY(max_depth: c_int) -> ();
    pub fn igLogToFile(max_depth: c_int, filename: *const c_char) -> ();
    pub fn igLogToClipboard(max_depth: c_int) -> ();
    pub fn igLogFinish() -> ();
    pub fn igLogButtons() -> ();
    pub fn igLogText(fmt: *const c_char, ...) -> ();
    pub fn igIsItemHovered() -> u8;
    pub fn igIsItemHoveredRect() -> u8;
    pub fn igIsItemActive() -> u8;
    pub fn igIsItemVisible() -> u8;
    pub fn igIsAnyItemHovered() -> u8;
    pub fn igIsAnyItemActive() -> u8;
    pub fn igGetItemRectMin(pOut: *mut ImVec2) -> ();
    pub fn igGetItemRectMax(pOut: *mut ImVec2) -> ();
    pub fn igGetItemRectSize(pOut: *mut ImVec2) -> ();
    pub fn igIsWindowHovered() -> u8;
    pub fn igIsWindowFocused() -> u8;
    pub fn igIsRootWindowFocused() -> u8;
    pub fn igIsRootWindowOrAnyChildFocused() -> u8;
    pub fn igIsRectVisible(item_size: ImVec2) -> u8;
    pub fn igIsPosHoveringAnyWindow(pos: ImVec2) -> u8;
    pub fn igGetTime() -> c_float;
    pub fn igGetFrameCount() -> c_int;
    pub fn igGetStyleColName(idx: ImGuiCol) -> *const c_char;
    pub fn igCalcItemRectClosestPoint(pOut: *mut ImVec2, pos: ImVec2, on_edge: u8, outward: c_float) -> ();
    pub fn igCalcTextSize(pOut: *mut ImVec2, text: *const c_char, text_end: *const c_char, hide_text_after_double_hash: u8, wrap_width: c_float) -> ();
    pub fn igCalcListClipping(items_count: c_int, items_height: c_float, out_items_display_start: *mut c_int, out_items_display_end: *mut c_int) -> ();
    pub fn igBeginChildFrame(id: ImGuiID, size: ImVec2, extra_flags: ImGuiWindowFlags) -> u8;
    pub fn igEndChildFrame() -> ();
    pub fn igColorConvertU32ToFloat4(pOut: *mut ImVec4, _in: ImU32) -> ();
    pub fn igColorConvertFloat4ToU32(_in: ImVec4) -> ImU32;
    pub fn igColorConvertRGBtoHSV(r: c_float, g: c_float, b: c_float, out_h: *mut c_float, out_s: *mut c_float, out_v: *mut c_float) -> ();
    pub fn igColorConvertHSVtoRGB(h: c_float, s: c_float, v: c_float, out_r: *mut c_float, out_g: *mut c_float, out_b: *mut c_float) -> ();
    pub fn igGetKeyIndex(key: ImGuiKey) -> c_int;
    pub fn igIsKeyDown(key_index: c_int) -> u8;
    pub fn igIsKeyPressed(key_index: c_int, repeat: u8) -> u8;
    pub fn igIsKeyReleased(key_index: c_int) -> u8;
    pub fn igIsMouseDown(button: c_int) -> u8;
    pub fn igIsMouseClicked(button: c_int, repeat: u8) -> u8;
    pub fn igIsMouseDoubleClicked(button: c_int) -> u8;
    pub fn igIsMouseReleased(button: c_int) -> u8;
    pub fn igIsMouseHoveringWindow() -> u8;
    pub fn igIsMouseHoveringAnyWindow() -> u8;
    pub fn igIsMouseHoveringRect(pos_min: ImVec2, pos_max: ImVec2, clip: u8) -> u8;
    pub fn igIsMouseDragging(button: c_int, lock_threshold: c_float) -> u8;
    pub fn igGetMousePos(pOut: *mut ImVec2) -> ();
    pub fn igGetMousePosOnOpeningCurrentPopup(pOut: *mut ImVec2) -> ();
    pub fn igGetMouseDragDelta(pOut: *mut ImVec2, button: c_int, lock_threshold: c_float) -> ();
    pub fn igResetMouseDragDelta(button: c_int) -> ();
    pub fn igGetMouseCursor() -> ImGuiMouseCursor;
    pub fn igSetMouseCursor(_type: ImGuiMouseCursor) -> ();
    pub fn igCaptureKeyboardFromApp() -> ();
    pub fn igCaptureMouseFromApp() -> ();
    pub fn igMemAlloc(sz: size_t) -> *mut c_void;
    pub fn igMemFree(ptr: *mut c_void) -> ();
    pub fn igGetClipboardText() -> *const c_char;
    pub fn igSetClipboardText(text: *const c_char) -> ();
    pub fn igGetVersion() -> *const c_char;
    pub fn igGetInternalState() -> *mut c_void;
    pub fn igGetInternalStateSize() -> size_t;
    pub fn igSetInternalState(state: *mut c_void, construct: u8) -> ();
    pub fn ImFontAtlas_GetTexDataAsRGBA32(atlas: *mut ImFontAtlas, out_pixels: *mut *mut c_uchar, out_width: *mut c_int, out_height: *mut c_int, out_bytes_per_pixel: *mut c_int) -> ();
    pub fn ImFontAtlas_GetTexDataAsAlpha8(atlas: *mut ImFontAtlas, out_pixels: *mut *mut c_uchar, out_width: *mut c_int, out_height: *mut c_int, out_bytes_per_pixel: *mut c_int) -> ();
    pub fn ImFontAtlas_SetTexID(atlas: *mut ImFontAtlas, tex: *mut c_void) -> ();
    pub fn ImFontAtlas_AddFont(atlas: *mut ImFontAtlas, font_cfg: *const ImFontConfig) -> *mut ImFont;
    pub fn ImFontAtlas_AddFontDefault(atlas: *mut ImFontAtlas, font_cfg: *const ImFontConfig) -> *mut ImFont;
    pub fn ImFontAtlas_AddFontFromFileTTF(atlas: *mut ImFontAtlas, filename: *const c_char, size_pixels: c_float, font_cfg: *const ImFontConfig, glyph_ranges: *const ImWchar) -> *mut ImFont;
    pub fn ImFontAtlas_AddFontFromMemoryTTF(atlas: *mut ImFontAtlas, ttf_data: *mut c_void, ttf_size: c_int, size_pixels: c_float, font_cfg: *const ImFontConfig, glyph_ranges: *const ImWchar) -> *mut ImFont;
    pub fn ImFontAtlas_AddFontFromMemoryCompressedTTF(atlas: *mut ImFontAtlas, compressed_ttf_data: *const c_void, compressed_ttf_size: c_int, size_pixels: c_float, font_cfg: *const ImFontConfig, glyph_ranges: *const ImWchar) -> *mut ImFont;
    pub fn ImFontAtlas_AddFontFromMemoryCompressedBase85TTF(atlas: *mut ImFontAtlas, compressed_ttf_data_base85: *const c_char, size_pixels: c_float, font_cfg: *const ImFontConfig, glyph_ranges: *const ImWchar) -> *mut ImFont;
    pub fn ImFontAtlas_ClearTexData(atlas: *mut ImFontAtlas) -> ();
    pub fn ImFontAtlas_Clear(atlas: *mut ImFontAtlas) -> ();
    pub fn ImGuiIO_AddInputCharacter(c: c_ushort) -> ();
    pub fn ImGuiIO_AddInputCharactersUTF8(utf8_chars: *const c_char) -> ();
    pub fn ImGuiIO_ClearInputCharacters() -> ();
    pub fn ImDrawData_DeIndexAllBuffers(drawData: *mut ImDrawData) -> ();
    pub fn ImDrawList_GetVertexBufferSize(list: *mut ImDrawList) -> c_int;
    pub fn ImDrawList_GetVertexPtr(list: *mut ImDrawList, n: c_int) -> *mut ImDrawVert;
    pub fn ImDrawList_GetIndexBufferSize(list: *mut ImDrawList) -> c_int;
    pub fn ImDrawList_GetIndexPtr(list: *mut ImDrawList, n: c_int) -> *mut ImDrawIdx;
    pub fn ImDrawList_GetCmdSize(list: *mut ImDrawList) -> c_int;
    pub fn ImDrawList_GetCmdPtr(list: *mut ImDrawList, n: c_int) -> *mut ImDrawCmd;
    pub fn ImDrawList_Clear(list: *mut ImDrawList) -> ();
    pub fn ImDrawList_ClearFreeMemory(list: *mut ImDrawList) -> ();
    pub fn ImDrawList_PushClipRect(list: *mut ImDrawList, clip_rect: ImVec4) -> ();
    pub fn ImDrawList_PushClipRectFullScreen(list: *mut ImDrawList) -> ();
    pub fn ImDrawList_PopClipRect(list: *mut ImDrawList) -> ();
    pub fn ImDrawList_PushTextureID(list: *mut ImDrawList, texture_id: ImTextureID) -> ();
    pub fn ImDrawList_PopTextureID(list: *mut ImDrawList) -> ();
    pub fn ImDrawList_AddLine(list: *mut ImDrawList, a: ImVec2, b: ImVec2, col: ImU32, thickness: c_float) -> ();
    pub fn ImDrawList_AddRect(list: *mut ImDrawList, a: ImVec2, b: ImVec2, col: ImU32, rounding: c_float, rounding_corners: c_int) -> ();
    pub fn ImDrawList_AddRectFilled(list: *mut ImDrawList, a: ImVec2, b: ImVec2, col: ImU32, rounding: c_float, rounding_corners: c_int) -> ();
    pub fn ImDrawList_AddRectFilledMultiColor(list: *mut ImDrawList, a: ImVec2, b: ImVec2, col_upr_left: ImU32, col_upr_right: ImU32, col_bot_right: ImU32, col_bot_left: ImU32) -> ();
    pub fn ImDrawList_AddTriangleFilled(list: *mut ImDrawList, a: ImVec2, b: ImVec2, c: ImVec2, col: ImU32) -> ();
    pub fn ImDrawList_AddCircle(list: *mut ImDrawList, centre: ImVec2, radius: c_float, col: ImU32, num_segments: c_int) -> ();
    pub fn ImDrawList_AddCircleFilled(list: *mut ImDrawList, centre: ImVec2, radius: c_float, col: ImU32, num_segments: c_int) -> ();
    pub fn ImDrawList_AddText(list: *mut ImDrawList, pos: ImVec2, col: ImU32, text_begin: *const c_char, text_end: *const c_char) -> ();
    pub fn ImDrawList_AddTextExt(list: *mut ImDrawList, font: *const ImFont, font_size: c_float, pos: ImVec2, col: ImU32, text_begin: *const c_char, text_end: *const c_char, wrap_width: c_float, cpu_fine_clip_rect: *const ImVec4) -> ();
    pub fn ImDrawList_AddImage(list: *mut ImDrawList, user_texture_id: ImTextureID, a: ImVec2, b: ImVec2, uv0: ImVec2, uv1: ImVec2, col: ImU32) -> ();
    pub fn ImDrawList_AddPolyline(list: *mut ImDrawList, points: *const ImVec2, num_points: c_int, col: ImU32, closed: u8, thickness: c_float, anti_aliased: u8) -> ();
    pub fn ImDrawList_AddConvexPolyFilled(list: *mut ImDrawList, points: *const ImVec2, num_points: c_int, col: ImU32, anti_aliased: u8) -> ();
    pub fn ImDrawList_AddBezierCurve(list: *mut ImDrawList, pos0: ImVec2, cp0: ImVec2, cp1: ImVec2, pos1: ImVec2, col: ImU32, thickness: c_float, num_segments: c_int) -> ();
    pub fn ImDrawList_PathClear(list: *mut ImDrawList) -> ();
    pub fn ImDrawList_PathLineTo(list: *mut ImDrawList, pos: ImVec2) -> ();
    pub fn ImDrawList_PathLineToMergeDuplicate(list: *mut ImDrawList, pos: ImVec2) -> ();
    pub fn ImDrawList_PathFill(list: *mut ImDrawList, col: ImU32) -> ();
    pub fn ImDrawList_PathStroke(list: *mut ImDrawList, col: ImU32, closed: u8, thickness: c_float) -> ();
    pub fn ImDrawList_PathArcTo(list: *mut ImDrawList, centre: ImVec2, radius: c_float, a_min: c_float, a_max: c_float, num_segments: c_int) -> ();
    pub fn ImDrawList_PathArcToFast(list: *mut ImDrawList, centre: ImVec2, radius: c_float, a_min_of_12: c_int, a_max_of_12: c_int) -> ();
    pub fn ImDrawList_PathBezierCurveTo(list: *mut ImDrawList, p1: ImVec2, p2: ImVec2, p3: ImVec2, num_segments: c_int) -> ();
    pub fn ImDrawList_PathRect(list: *mut ImDrawList, rect_min: ImVec2, rect_max: ImVec2, rounding: c_float, rounding_corners: c_int) -> ();
    pub fn ImDrawList_ChannelsSplit(list: *mut ImDrawList, channels_count: c_int) -> ();
    pub fn ImDrawList_ChannelsMerge(list: *mut ImDrawList) -> ();
    pub fn ImDrawList_ChannelsSetCurrent(list: *mut ImDrawList, channel_index: c_int) -> ();
    pub fn ImDrawList_AddCallback(list: *mut ImDrawList, callback: ImDrawCallback, callback_data: *mut c_void) -> ();
    pub fn ImDrawList_AddDrawCmd(list: *mut ImDrawList) -> ();
    pub fn ImDrawList_PrimReserve(list: *mut ImDrawList, idx_count: c_int, vtx_count: c_int) -> ();
    pub fn ImDrawList_PrimRect(list: *mut ImDrawList, a: ImVec2, b: ImVec2, col: ImU32) -> ();
    pub fn ImDrawList_PrimRectUV(list: *mut ImDrawList, a: ImVec2, b: ImVec2, uv_a: ImVec2, uv_b: ImVec2, col: ImU32) -> ();
    pub fn ImDrawList_PrimVtx(list: *mut ImDrawList, pos: ImVec2, uv: ImVec2, col: ImU32) -> ();
    pub fn ImDrawList_PrimWriteVtx(list: *mut ImDrawList, pos: ImVec2, uv: ImVec2, col: ImU32) -> ();
    pub fn ImDrawList_PrimWriteIdx(list: *mut ImDrawList, idx: ImDrawIdx) -> ();
    pub fn ImDrawList_UpdateClipRect(list: *mut ImDrawList) -> ();
    pub fn ImDrawList_UpdateTextureID(list: *mut ImDrawList) -> ();

    pub fn gl3wInit() -> ();
    pub fn ImGui_ImplSdlGL3_Init(window: *mut c_void) -> bool;
    pub fn ImGui_ImplSdlGL3_Shutdown();
    pub fn ImGui_ImplSdlGL3_NewFrame();
}
