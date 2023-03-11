use bitflags::bitflags;

#[derive(PartialEq)]
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

pub struct SAppData {}

pub struct SApp<T>
where
    T: SAppI,
{
    base: SAppData,
    app: T,
}

pub fn run_app<T>(app: T)
where
    T: SAppI,
{
    let mut b = SApp {
        base: SAppData {},
        app,
    };

    b.app.init(&mut b.base);
}

/*
enum {
    SAPP_MAX_TOUCHPOINTS = 8,
    SAPP_MAX_MOUSEBUTTONS = 3,
    SAPP_MAX_KEYCODES = 512,
    SAPP_MAX_ICONIMAGES = 8,
};


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

typedef struct {
    sapp_desc desc;
    bool valid;
    bool fullscreen;
    bool gles2_fallback;
    bool first_frame;
    bool init_called;
    bool cleanup_called;
    bool quit_requested;
    bool quit_ordered;
    bool event_consumed;
    bool html5_ask_leave_site;
    bool onscreen_keyboard_shown;
    int window_width;
    int window_height;
    int framebuffer_width;
    int framebuffer_height;
    int sample_count;
    int swap_interval;
    float dpi_scale;
    uint64_t frame_count;
    _sapp_timing_t timing;
    sapp_event event;
    _sapp_mouse_t mouse;
    _sapp_clipboard_t clipboard;
    _sapp_drop_t drop;
    sapp_icon_desc default_icon_desc;
    uint32_t* default_icon_pixels;
    _sapp_win32_t win32;
    _sapp_wgl_t wgl;
    char window_title[_SAPP_MAX_TITLE_LENGTH];      /* UTF-8 */
    wchar_t window_title_wide[_SAPP_MAX_TITLE_LENGTH];   /* UTF-32 or UCS-2 */
    sapp_keycode keycodes[SAPP_MAX_KEYCODES];
} _sapp_t;
*/
