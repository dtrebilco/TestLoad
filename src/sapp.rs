use bitflags::bitflags;
use windows_sys::Win32::Foundation::*;
use windows_sys::Win32::Graphics::Gdi::*;
use windows_sys::Win32::Graphics::OpenGL::*;
use windows_sys::Win32::UI::WindowsAndMessaging::*;

#[derive(PartialEq, Clone, Copy)]
pub enum KeyCode {
    Invalid = 0,
    Space = 32,
    Apostrophe = 39, /* ' */
    Comma = 44,      /* , */
    Minus = 45,      /* - */
    Period = 46,     /* . */
    Slash = 47,      /* / */
    Num0 = 48,
    Num1 = 49,
    Num2 = 50,
    Num3 = 51,
    Num4 = 52,
    Num5 = 53,
    Num6 = 54,
    Num7 = 55,
    Num8 = 56,
    Num9 = 57,
    Semicolon = 59, /* ; */
    Equal = 61,     /* = */
    A = 65,
    B = 66,
    C = 67,
    D = 68,
    E = 69,
    F = 70,
    G = 71,
    H = 72,
    I = 73,
    J = 74,
    K = 75,
    L = 76,
    M = 77,
    N = 78,
    O = 79,
    P = 80,
    Q = 81,
    R = 82,
    S = 83,
    T = 84,
    U = 85,
    V = 86,
    W = 87,
    X = 88,
    Y = 89,
    Z = 90,
    LeftBracket = 91,  /* [ */
    Backslash = 92,    /* \ */
    RightBracket = 93, /* ] */
    GraveAccent = 96,  /* ` */
    World1 = 161,      /* non-US #1 */
    World2 = 162,      /* non-US #2 */
    Escape = 256,
    Enter = 257,
    Tab = 258,
    Backspace = 259,
    Insert = 260,
    Delete = 261,
    Right = 262,
    Left = 263,
    Down = 264,
    Up = 265,
    PageUp = 266,
    PageDown = 267,
    Home = 268,
    End = 269,
    CapsLock = 280,
    ScrollLock = 281,
    NumLock = 282,
    PrintScreen = 283,
    Pause = 284,
    F1 = 290,
    F2 = 291,
    F3 = 292,
    F4 = 293,
    F5 = 294,
    F6 = 295,
    F7 = 296,
    F8 = 297,
    F9 = 298,
    F10 = 299,
    F11 = 300,
    F12 = 301,
    F13 = 302,
    F14 = 303,
    F15 = 304,
    F16 = 305,
    F17 = 306,
    F18 = 307,
    F19 = 308,
    F20 = 309,
    F21 = 310,
    F22 = 311,
    F23 = 312,
    F24 = 313,
    F25 = 314,
    Kp0 = 320,
    Kp1 = 321,
    Kp2 = 322,
    Kp3 = 323,
    Kp4 = 324,
    Kp5 = 325,
    Kp6 = 326,
    Kp7 = 327,
    Kp8 = 328,
    Kp9 = 329,
    KpDecimal = 330,
    KpDivide = 331,
    KpMultiply = 332,
    KpSubtract = 333,
    KpAdd = 334,
    KpEnter = 335,
    KpEqual = 336,
    LeftShift = 340,
    LeftControl = 341,
    LeftAlt = 342,
    LeftSuper = 343,
    RightShift = 344,
    RightControl = 345,
    RightAlt = 346,
    RightSuper = 347,
    Menu = 348,
}

#[derive(PartialEq)]
pub enum MouseButton {
    Left = 0x0,
    Right = 0x1,
    Middle = 0x2,
    Invalid = 0x100,
}

// These are currently pressed modifier keys (and mouse buttons) which are
// passed in the event struct field sapp_event.modifiers.
bitflags! {
    pub struct Modifier : u32 {
        const Shift = 0x1;      // left or right shift key
        const Ctrl  = 0x2;      // left or right control key
        const Alt   = 0x4;      // left or right alt key
        const Super = 0x8;      // left or right 'super' key
        const Lmb   = 0x100;    // left mouse button
        const Rmb   = 0x200;    // right mouse button
        const Mmb   = 0x400;    // middle mouse button
    }
}

pub struct KeyEvent {
    pub pressed: bool,       // true if the key is pressed
    pub key_code: KeyCode,   // the virtual key code, only valid in KEY_UP, KEY_DOWN
    pub key_repeat: bool, // true if this is a key-repeat event, valid in KEY_UP, KEY_DOWN and CHAR
    pub modifiers: Modifier, // current modifier keys, valid in all key-, char- and mouse-events
}

pub struct CharEvent {
    pub char_code: char,     // the UTF-32 character code, only valid in CHAR events
    pub key_repeat: bool, // true if this is a key-repeat event, valid in KEY_UP, KEY_DOWN and CHAR
    pub modifiers: Modifier, // current modifier keys, valid in all key-, char- and mouse-events
}

pub struct MouseEvent {
    pub pressed: bool,             // true if the mouse is pressed
    pub mouse_button: MouseButton, // mouse button that was pressed or released, valid in MOUSE_DOWN, MOUSE_UP
    pub modifiers: Modifier, // current modifier keys, valid in all key-, char- and mouse-events
}

pub struct MouseScrollEvent {
    pub scroll_x: f32, // horizontal mouse wheel scroll distance, valid in MOUSE_SCROLL events
    pub scroll_y: f32, // vertical mouse wheel scroll distance, valid in MOUSE_SCROLL events
}

pub enum Event {
    Key(KeyEvent),
    Char(CharEvent),
    Mouse(MouseEvent),
    MouseScroll(MouseScrollEvent),
    MouseMove,
    MouseEnter,
    MouseLeave,
    TouchesBegan,
    TouchesMoved,
    TouchesEnded,
    TouchesCancelled,
    Resized,
    Iconified,
    Restored,
    Focused,
    Unfocused,
    Suspended,
    Resumed,
    QuitRequested,
    ClipboardPasted,
    FilesDropped,
}

/*
    sapp_event

    This is an all-in-one event struct passed to the event handler
    user callback function. Note that it depends on the event
    type what struct fields actually contain useful values, so you
    should first check the event type before reading other struct
    fields.
*/
/*
struct Event {

    sapp_event_type type;               // the event type, always valid

    sapp_keycode key_code;              // the virtual key code, only valid in KEY_UP, KEY_DOWN
    uint32_t char_code;                 // the UTF-32 character code, only valid in CHAR events
    bool key_repeat;                    // true if this is a key-repeat event, valid in KEY_UP, KEY_DOWN and CHAR

    uint32_t modifiers;                 // current modifier keys, valid in all key-, char- and mouse-events
    sapp_mousebutton mouse_button;      // mouse button that was pressed or released, valid in MOUSE_DOWN, MOUSE_UP

    float scroll_x;                     // horizontal mouse wheel scroll distance, valid in MOUSE_SCROLL events
    float scroll_y;                     // vertical mouse wheel scroll distance, valid in MOUSE_SCROLL events
    //int num_touches;                    // number of valid items in the touches[] array
    //sapp_touchpoint touches[SAPP_MAX_TOUCHPOINTS];  // current touch points, valid in TOUCHES_BEGIN, TOUCHES_MOVED, TOUCHES_ENDED

    uint64_t frame_count;               // current frame counter, always valid, useful for checking if two events were issued in the same frame
    float mouse_x;                      // current horizontal mouse position in pixels, always valid except during mouse lock
    float mouse_y;                      // current vertical mouse position in pixels, always valid except during mouse lock
    float mouse_dx;                     // relative horizontal mouse movement since last frame, always valid
    float mouse_dy;                     // relative vertical mouse movement since last frame, always valid

    int window_width;                   // current window- and framebuffer sizes in pixels, always valid
    int window_height;
    int framebuffer_width;              // = window_width * dpi_scale
    int framebuffer_height;             // = window_height * dpi_scale
}
*/

pub trait SAppI {
    fn init(&mut self, _app: &mut SAppData) {}

    fn draw_frame(&mut self, _app: &mut SAppData) {}

    fn on_event(&mut self, _app: &mut SAppData, _event: &Event) {}

    fn shutdown(&mut self, _app: &mut SAppData) {}
}

const SAPP_MAX_TOUCHPOINTS: u32 = 8;
const SAPP_MAX_MOUSEBUTTONS: u32 = 3;
const SAPP_MAX_KEYCODES: u32 = 512;
const SAPP_MAX_ICONIMAGES: u32 = 8;

struct SAppMouse {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
    shown: bool,
    locked: bool,
    pos_valid: bool,
    //sapp_mouse_cursor current_cursor; // Cursor icon enum
}
impl SAppMouse {
    fn new() -> SAppMouse {
        SAppMouse {
            x: 0.0,
            y: 0.0,
            dx: 0.0,
            dy: 0.0,
            shown: true,
            locked: false,
            pos_valid: false,
        }
    }
}

struct SAppWin32 {
    hwnd: HWND,
    hmonitor: HMONITOR,
    dc: HDC,
    big_icon: HICON,
    small_icon: HICON,
    //cursors: [HCURSOR; _SAPP_MOUSECURSOR_NUM],
    orig_codepage: u32,
    mouse_locked_x: i32,
    mouse_locked_y: i32,
    stored_window_rect: RECT, // used to restore window pos/size when toggling fullscreen => windowed
    is_win10_or_greater: bool,
    in_create_window: bool,
    iconified: bool,
    mouse_tracked: bool,
    mouse_capture_mask: u8,
    //dpi : _sapp_win32_dpi_t,
    raw_input_mousepos_valid: bool,
    raw_input_mousepos_x: i32,
    raw_input_mousepos_y: i32,
    raw_input_data: [u8; 256],
}
impl SAppWin32 {
    fn new() -> SAppWin32 {
        SAppWin32 {
            hwnd: 0,
            hmonitor: 0,
            dc: 0,
            big_icon: 0,
            small_icon: 0,
            //cursors: [HCURSOR; _SAPP_MOUSECURSOR_NUM],
            orig_codepage: 0,
            mouse_locked_x: 0,
            mouse_locked_y: 0,
            stored_window_rect: RECT {
                left: 0,
                top: 0,
                right: 0,
                bottom: 0,
            }, // used to restore window pos/size when toggling fullscreen => windowed
            is_win10_or_greater: false,
            in_create_window: false,
            iconified: false,
            mouse_tracked: false,
            mouse_capture_mask: 0,
            //dpi : _sapp_win32_dpi_t,
            raw_input_mousepos_valid: false,
            raw_input_mousepos_x: 0,
            raw_input_mousepos_y: 0,
            raw_input_data: [0; 256],
        }
    }
}

const WGL_NUMBER_PIXEL_FORMATS_ARB: u32 = 0x2000;
const WGL_SUPPORT_OPENGL_ARB: u32 = 0x2010;
const WGL_DRAW_TO_WINDOW_ARB: u32 = 0x2001;
const WGL_PIXEL_TYPE_ARB: u32 = 0x2013;
const WGL_TYPE_RGBA_ARB: u32 = 0x202b;
const WGL_ACCELERATION_ARB: u32 = 0x2003;
const WGL_NO_ACCELERATION_ARB: u32 = 0x2025;
const WGL_RED_BITS_ARB: u32 = 0x2015;
const WGL_GREEN_BITS_ARB: u32 = 0x2017;
const WGL_BLUE_BITS_ARB: u32 = 0x2019;
const WGL_ALPHA_BITS_ARB: u32 = 0x201b;
const WGL_DEPTH_BITS_ARB: u32 = 0x2022;
const WGL_STENCIL_BITS_ARB: u32 = 0x2023;
const WGL_DOUBLE_BUFFER_ARB: u32 = 0x2011;
const WGL_SAMPLES_ARB: u32 = 0x2042;
const WGL_CONTEXT_FORWARD_COMPATIBLE_BIT_ARB: u32 = 0x00000002;
const WGL_CONTEXT_PROFILE_MASK_ARB: u32 = 0x9126;
const WGL_CONTEXT_CORE_PROFILE_BIT_ARB: u32 = 0x00000001;
const WGL_CONTEXT_MAJOR_VERSION_ARB: u32 = 0x2091;
const WGL_CONTEXT_MINOR_VERSION_ARB: u32 = 0x2092;
const WGL_CONTEXT_FLAGS_ARB: u32 = 0x2094;
const ERROR_INVALID_VERSION_ARB: u32 = 0x2095;
const ERROR_INVALID_PROFILE_ARB: u32 = 0x2096;
const ERROR_INCOMPATIBLE_DEVICE_CONTEXTS_ARB: u32 = 0x2054;

//type PFNWGLSWAPINTERVALEXTPROC = extern "system" fn(u32) -> bool;
/*
typedef BOOL (WINAPI * PFNWGLSWAPINTERVALEXTPROC)(int);
typedef BOOL (WINAPI * PFNWGLGETPIXELFORMATATTRIBIVARBPROC)(HDC,int,int,UINT,const int*,int*);
typedef const char* (WINAPI * PFNWGLGETEXTENSIONSSTRINGEXTPROC)(void);
typedef const char* (WINAPI * PFNWGLGETEXTENSIONSSTRINGARBPROC)(HDC);
typedef HGLRC (WINAPI * PFNWGLCREATECONTEXTATTRIBSARBPROC)(HDC,HGLRC,const int*);
typedef HGLRC (WINAPI * PFN_wglCreateContext)(HDC);
typedef BOOL (WINAPI * PFN_wglDeleteContext)(HGLRC);
typedef PROC (WINAPI * PFN_wglGetProcAddress)(LPCSTR);
typedef HDC (WINAPI * PFN_wglGetCurrentDC)(void);
typedef BOOL (WINAPI * PFN_wglMakeCurrent)(HDC,HGLRC);
*/

/*
struct SAppWgl {
    HINSTANCE opengl32;
    HGLRC gl_ctx;
    PFN_wglCreateContext CreateContext;
    PFN_wglDeleteContext DeleteContext;
    PFN_wglGetProcAddress GetProcAddress;
    PFN_wglGetCurrentDC GetCurrentDC;
    PFN_wglMakeCurrent MakeCurrent;
    PFNWGLSWAPINTERVALEXTPROC SwapIntervalEXT;
    PFNWGLGETPIXELFORMATATTRIBIVARBPROC GetPixelFormatAttribivARB;
    PFNWGLGETEXTENSIONSSTRINGEXTPROC GetExtensionsStringEXT;
    PFNWGLGETEXTENSIONSSTRINGARBPROC GetExtensionsStringARB;
    PFNWGLCREATECONTEXTATTRIBSARBPROC CreateContextAttribsARB;
    bool ext_swap_control;
    bool arb_multisample;
    bool arb_pixel_format;
    bool arb_create_context;
    bool arb_create_context_profile;
    HWND msg_hwnd;
    HDC msg_dc;
}
*/

pub struct SAppData<'a> {
    desc: SAppDesc<'a>,
    valid: bool,
    fullscreen: bool,

    first_frame: bool,
    init_called: bool,
    cleanup_called: bool,
    quit_requested: bool,
    quit_ordered: bool,
    event_consumed: bool,

    window_width: u32,
    window_height: u32,
    framebuffer_width: u32,
    framebuffer_height: u32,
    sample_count: u32,
    swap_interval: u32,
    dpi_scale: f32,
    frame_count: u64,

    //_sapp_timing_t timing;
    //sapp_event event;
    mouse: SAppMouse,
    //_sapp_clipboard_t clipboard;
    //_sapp_drop_t drop;
    //sapp_icon_desc default_icon_desc;
    //uint32_t* default_icon_pixels;
    win32: SAppWin32,
    //wgl : SAppWgl,
    //char window_title[_SAPP_MAX_TITLE_LENGTH];      /* UTF-8 */
    //wchar_t window_title_wide[_SAPP_MAX_TITLE_LENGTH];   /* UTF-32 or UCS-2 */
    keycodes: [KeyCode; SAPP_MAX_KEYCODES as usize],
}

impl<'a> SAppData<'a> {
    fn new(desc: SAppDesc) -> SAppData {
        SAppData {
            valid: false,
            fullscreen: desc.fullscreen,
            first_frame: true,
            init_called: false,
            cleanup_called: false,
            quit_requested: false,
            quit_ordered: false,
            event_consumed: false,

            // NOTE: _sapp.desc.width/height may be 0! Platform backends need to deal with this
            window_width: desc.width,
            window_height: desc.height,
            framebuffer_width: desc.width,
            framebuffer_height: desc.height,
            sample_count: desc.sample_count,
            swap_interval: desc.swap_interval,
            //clipboard.enabled = desc.enable_clipboard,
            //if (_sapp.clipboard.enabled) {
            //    _sapp.clipboard.buf_size = _sapp.desc.clipboard_size;
            //    _sapp.clipboard.buffer = (char*) _sapp_malloc_clear((size_t)_sapp.clipboard.buf_size);
            //}
            //_sapp.drop.enabled = _sapp.desc.enable_dragndrop;
            //if (_sapp.drop.enabled) {
            //    _sapp.drop.max_files = _sapp.desc.max_dropped_files;
            //    _sapp.drop.max_path_length = _sapp.desc.max_dropped_file_path_length;
            //    _sapp.drop.buf_size = _sapp.drop.max_files * _sapp.drop.max_path_length;
            //    _sapp.drop.buffer = (char*) _sapp_malloc_clear((size_t)_sapp.drop.buf_size);
            //}
            dpi_scale: 1.0,
            frame_count: 0,
            mouse: SAppMouse::new(),
            //_sapp_timing_init(&_sapp.timing);
            win32: SAppWin32::new(),
            keycodes: [KeyCode::Invalid; SAPP_MAX_KEYCODES as usize],
            desc,
        }
    }
}

pub struct SApp<'a, T>
where
    T: SAppI,
{
    base: SAppData<'a>,
    app: T,
}

pub fn run_app<T>(app: T, desc: SAppDesc)
where
    T: SAppI,
{
    let mut b = SApp {
        base: SAppData::new(desc),
        app,
    };

    b.app.init(&mut b.base);
}

impl<'a, T> SApp<'a, T> where T: SAppI {}

pub struct SAppDesc<'a> {
    pub width: u32,  // the preferred width of the window / canvas
    pub height: u32, // the preferred height of the window / canvas

    pub sample_count: u32,  // MSAA sample count
    pub swap_interval: u32, // the preferred swap interval (ignored on some platforms)

    pub high_dpi: bool, // whether the rendering canvas is full-resolution on HighDPI displays
    pub fullscreen: bool, // whether the window should be created in fullscreen mode
    pub alpha: bool, // whether the framebuffer should have an alpha channel (ignored on some platforms)

    pub window_title: &'a str,  // the window title as UTF-8 encoded string
    pub enable_clipboard: bool, // enable clipboard access, default is false
    pub clipboard_size: u32,    // max size of clipboard content in bytes
    pub enable_dragndrop: bool, // enable file dropping (drag'n'drop), default is false
    pub max_dropped_files: u32, // max number of dropped files to process (default: 1)
    pub max_dropped_file_path_length: u32, // max length in bytes of a dropped UTF-8 file path (default: 2048)
    //sapp_icon_desc icon;                // the initial window icon to set
    pub gl_major_version: u32, // override GL major and minor version (the default GL version is 3.2)
    pub gl_minor_version: u32,
    pub win32_console_utf8: bool, // if true, set the output console codepage to UTF-8
    pub win32_console_create: bool, // if true, attach stdout/stderr to a new console window
    pub win32_console_attach: bool, // if true, attach stdout/stderr to parent process
}

impl<'a> SAppDesc<'a> {
    pub fn new() -> SAppDesc<'a> {
        SAppDesc {
            width: 1,
            height: 1,

            sample_count: 1,
            swap_interval: 1,

            high_dpi: false,
            fullscreen: false,
            alpha: false,

            window_title: "Title",
            enable_clipboard: false,
            clipboard_size: 0,
            enable_dragndrop: false,
            max_dropped_files: 1,
            max_dropped_file_path_length: 2048,

            gl_major_version: 3,
            gl_minor_version: 2,
            win32_console_utf8: false,
            win32_console_create: false,
            win32_console_attach: false,
        }
    }
}

/*

typedef struct {
    bool enabled;
    int buf_size;
    char* buffer;
} _sapp_clipboard_t;

typedef struct {
    bool enabled;
    int max_files;
    int max_path_length;
    int num_files;
    int buf_size;
    char* buffer;
} _sapp_drop_t;

typedef struct {
    float x, y;
    float dx, dy;
    bool shown;
    bool locked;
    bool pos_valid;
    sapp_mouse_cursor current_cursor;
} _sapp_mouse_t;

_SOKOL_PRIVATE void _sapp_win32_run(const sapp_desc* desc) {
    _sapp_init_state(desc);
    _sapp_win32_init_console();
    _sapp.win32.is_win10_or_greater = _sapp_win32_is_win10_or_greater();
    _sapp_win32_init_keytable();
    _sapp_win32_utf8_to_wide(_sapp.window_title, _sapp.window_title_wide, sizeof(_sapp.window_title_wide));
    _sapp_win32_init_dpi();
    _sapp_win32_init_cursors();
    _sapp_win32_create_window();
    sapp_set_icon(&desc->icon);
    _sapp_wgl_init();
    _sapp_wgl_load_extensions();
    _sapp_wgl_create_context();
    _sapp.valid = true;

    bool done = false;
    while (!(done || _sapp.quit_ordered)) {
        _sapp_win32_timing_measure();
        MSG msg;
        while (PeekMessageW(&msg, NULL, 0, 0, PM_REMOVE)) {
            if (WM_QUIT == msg.message) {
                done = true;
                continue;
            }
            else {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
        _sapp_frame();
        _sapp_wgl_swap_buffers();

        /* check for window resized, this cannot happen in WM_SIZE as it explodes memory usage */
        if (_sapp_win32_update_dimensions()) {
            _sapp_win32_app_event(SAPP_EVENTTYPE_RESIZED);
        }
        /* check if the window monitor has changed, need to reset timing because
           the new monitor might have a different refresh rate
        */
        if (_sapp_win32_update_monitor()) {
            _sapp_timing_reset(&_sapp.timing);
        }
        if (_sapp.quit_requested) {
            PostMessage(_sapp.win32.hwnd, WM_CLOSE, 0, 0);
        }
    }
    _sapp_call_cleanup();

    _sapp_wgl_destroy_context();
    _sapp_wgl_shutdown();
    _sapp_win32_destroy_window();
    _sapp_win32_destroy_icons();
    _sapp_win32_restore_console();
    _sapp_discard_state();
}


*/
