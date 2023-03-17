use bitflags::bitflags;
use windows_sys::core::*;
use windows_sys::Win32::Foundation::*;
use windows_sys::Win32::Graphics::Gdi::*;
use windows_sys::Win32::Graphics::OpenGL::*;
use windows_sys::Win32::System::LibraryLoader::{FreeLibrary, GetModuleHandleW, GetProcAddress, LoadLibraryA};
use windows_sys::Win32::UI::HiDpi::*;
use windows_sys::Win32::UI::WindowsAndMessaging::*;
use windows_sys::Win32::UI::Input::{GetRawInputData, RAWINPUT, HRAWINPUT, RID_INPUT, RAWINPUTHEADER};
use windows_sys::Win32::Devices::HumanInterfaceDevice::MOUSE_MOVE_ABSOLUTE;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{TrackMouseEvent, TRACKMOUSEEVENT, TME_LEAVE};

#[inline]
pub fn HIWORD(l: u32) -> u16 {
    ((l >> 16) & 0xffff) as u16
}

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
    MouseMove(Modifier),
    MouseEnter(Modifier),
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

fn sapp_win32_init_keytable(keycodes: &mut [KeyCode; SAPP_MAX_KEYCODES as usize]) {
    /* same as GLFW */
    keycodes[0x00B] = KeyCode::Num0;
    keycodes[0x002] = KeyCode::Num1;
    keycodes[0x003] = KeyCode::Num2;
    keycodes[0x004] = KeyCode::Num3;
    keycodes[0x005] = KeyCode::Num4;
    keycodes[0x006] = KeyCode::Num5;
    keycodes[0x007] = KeyCode::Num6;
    keycodes[0x008] = KeyCode::Num7;
    keycodes[0x009] = KeyCode::Num8;
    keycodes[0x00A] = KeyCode::Num9;
    keycodes[0x01E] = KeyCode::A;
    keycodes[0x030] = KeyCode::B;
    keycodes[0x02E] = KeyCode::C;
    keycodes[0x020] = KeyCode::D;
    keycodes[0x012] = KeyCode::E;
    keycodes[0x021] = KeyCode::F;
    keycodes[0x022] = KeyCode::G;
    keycodes[0x023] = KeyCode::H;
    keycodes[0x017] = KeyCode::I;
    keycodes[0x024] = KeyCode::J;
    keycodes[0x025] = KeyCode::K;
    keycodes[0x026] = KeyCode::L;
    keycodes[0x032] = KeyCode::M;
    keycodes[0x031] = KeyCode::N;
    keycodes[0x018] = KeyCode::O;
    keycodes[0x019] = KeyCode::P;
    keycodes[0x010] = KeyCode::Q;
    keycodes[0x013] = KeyCode::R;
    keycodes[0x01F] = KeyCode::S;
    keycodes[0x014] = KeyCode::T;
    keycodes[0x016] = KeyCode::U;
    keycodes[0x02F] = KeyCode::V;
    keycodes[0x011] = KeyCode::W;
    keycodes[0x02D] = KeyCode::X;
    keycodes[0x015] = KeyCode::Y;
    keycodes[0x02C] = KeyCode::Z;
    keycodes[0x028] = KeyCode::Apostrophe;
    keycodes[0x02B] = KeyCode::Backslash;
    keycodes[0x033] = KeyCode::Comma;
    keycodes[0x00D] = KeyCode::Equal;
    keycodes[0x029] = KeyCode::GraveAccent;
    keycodes[0x01A] = KeyCode::LeftBracket;
    keycodes[0x00C] = KeyCode::Minus;
    keycodes[0x034] = KeyCode::Period;
    keycodes[0x01B] = KeyCode::RightBracket;
    keycodes[0x027] = KeyCode::Semicolon;
    keycodes[0x035] = KeyCode::Slash;
    keycodes[0x056] = KeyCode::World2;
    keycodes[0x00E] = KeyCode::Backspace;
    keycodes[0x153] = KeyCode::Delete;
    keycodes[0x14F] = KeyCode::End;
    keycodes[0x01C] = KeyCode::Enter;
    keycodes[0x001] = KeyCode::Escape;
    keycodes[0x147] = KeyCode::Home;
    keycodes[0x152] = KeyCode::Insert;
    keycodes[0x15D] = KeyCode::Menu;
    keycodes[0x151] = KeyCode::PageDown;
    keycodes[0x149] = KeyCode::PageUp;
    keycodes[0x045] = KeyCode::Pause;
    keycodes[0x146] = KeyCode::Pause;
    keycodes[0x039] = KeyCode::Space;
    keycodes[0x00F] = KeyCode::Tab;
    keycodes[0x03A] = KeyCode::CapsLock;
    keycodes[0x145] = KeyCode::NumLock;
    keycodes[0x046] = KeyCode::ScrollLock;
    keycodes[0x03B] = KeyCode::F1;
    keycodes[0x03C] = KeyCode::F2;
    keycodes[0x03D] = KeyCode::F3;
    keycodes[0x03E] = KeyCode::F4;
    keycodes[0x03F] = KeyCode::F5;
    keycodes[0x040] = KeyCode::F6;
    keycodes[0x041] = KeyCode::F7;
    keycodes[0x042] = KeyCode::F8;
    keycodes[0x043] = KeyCode::F9;
    keycodes[0x044] = KeyCode::F10;
    keycodes[0x057] = KeyCode::F11;
    keycodes[0x058] = KeyCode::F12;
    keycodes[0x064] = KeyCode::F13;
    keycodes[0x065] = KeyCode::F14;
    keycodes[0x066] = KeyCode::F15;
    keycodes[0x067] = KeyCode::F16;
    keycodes[0x068] = KeyCode::F17;
    keycodes[0x069] = KeyCode::F18;
    keycodes[0x06A] = KeyCode::F19;
    keycodes[0x06B] = KeyCode::F20;
    keycodes[0x06C] = KeyCode::F21;
    keycodes[0x06D] = KeyCode::F22;
    keycodes[0x06E] = KeyCode::F23;
    keycodes[0x076] = KeyCode::F24;
    keycodes[0x038] = KeyCode::LeftAlt;
    keycodes[0x01D] = KeyCode::LeftControl;
    keycodes[0x02A] = KeyCode::LeftShift;
    keycodes[0x15B] = KeyCode::LeftSuper;
    keycodes[0x137] = KeyCode::PrintScreen;
    keycodes[0x138] = KeyCode::RightAlt;
    keycodes[0x11D] = KeyCode::RightControl;
    keycodes[0x036] = KeyCode::RightShift;
    keycodes[0x15C] = KeyCode::RightSuper;
    keycodes[0x150] = KeyCode::Down;
    keycodes[0x14B] = KeyCode::Left;
    keycodes[0x14D] = KeyCode::Right;
    keycodes[0x148] = KeyCode::Up;
    keycodes[0x052] = KeyCode::Kp0;
    keycodes[0x04F] = KeyCode::Kp1;
    keycodes[0x050] = KeyCode::Kp2;
    keycodes[0x051] = KeyCode::Kp3;
    keycodes[0x04B] = KeyCode::Kp4;
    keycodes[0x04C] = KeyCode::Kp5;
    keycodes[0x04D] = KeyCode::Kp6;
    keycodes[0x047] = KeyCode::Kp7;
    keycodes[0x048] = KeyCode::Kp8;
    keycodes[0x049] = KeyCode::Kp9;
    keycodes[0x04E] = KeyCode::KpAdd;
    keycodes[0x053] = KeyCode::KpDecimal;
    keycodes[0x135] = KeyCode::KpDivide;
    keycodes[0x11C] = KeyCode::KpEnter;
    keycodes[0x037] = KeyCode::KpMultiply;
    keycodes[0x04A] = KeyCode::KpSubtract;
}

/*
extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message {
            WM_PAINT => {
                //println!("WM_PAINT");
                ValidateRect(window, std::ptr::null());
                0
            }
            WM_DESTROY => {
                //println!("WM_DESTROY");
                PostQuitMessage(0);
                0
            }
            _ => DefWindowProcW(window, message, wparam, lparam),
        }
    }
}
*/

unsafe extern "system" fn wndproc<T>(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT
where T : SAppI
{
    let sapp = GetWindowLongPtrW(window, GWLP_USERDATA) as *mut SApp<T>;
    let sapp = match sapp.as_mut() {
        Some(app) => app,
        _ => return DefWindowProcW(window, message, wparam, lparam)
    };

        match message {
            WM_CLOSE =>  {
                // only give user a chance to intervene when sapp_quit() wasn't already called
                if !sapp.base.quit_ordered {
                    // if window should be closed and event handling is enabled, give user code
                    //    a change to intervene via sapp_cancel_quit()
                    sapp.base.quit_requested = true;
                    sapp.call_event(&Event::QuitRequested);
                    // if user code hasn't intervened, quit the app
                    if sapp.base.quit_requested {
                        sapp.base.quit_ordered = true;
                    }
                }
                if sapp.base.quit_ordered {
                    PostQuitMessage(0);
                }
                return 0;
            }
            WM_SYSCOMMAND => {
                match (wparam & 0xFFF0) as u32 {
                    SC_SCREENSAVE | SC_MONITORPOWER => {
                        if sapp.base.fullscreen {
                            // disable screen saver and blanking in fullscreen mode
                            return 0;
                        }
                    }
                    SC_KEYMENU => {
                        // user trying to access menu via ALT
                        return 0;
                    }
                }
            }
            WM_ERASEBKGND => return 1,
            WM_SIZE => {
                    let iconified = wparam == SIZE_MINIMIZED as usize;
                    if iconified != sapp.base.win32.iconified {
                        sapp.base.win32.iconified = iconified;
                        if iconified {
                            sapp.call_event(&Event::Iconified);
                        }
                        else {
                            sapp.call_event(&Event::Restored);
                        }
                    }
                }

            WM_SETFOCUS => sapp.call_event(&Event::Focused),

            WM_KILLFOCUS => {
                // if focus is lost for any reason, and we're in mouse locked mode, disable mouse lock
                if sapp.base.mouse.locked {
                    sapp_win32_lock_mouse(false);
                }
                sapp.call_event(&Event::Unfocused);
            }
            WM_SETCURSOR => {
                //if (LOWORD(lParam) == HTCLIENT) {
                //    sapp_win32_update_cursor(_sapp.mouse.current_cursor, _sapp.mouse.shown, true);
                //    return TRUE;
                //}
            }
            WM_DPICHANGED =>
            {
                // Update window's DPI and size if its moved to another monitor with a different DPI
                // Only sent if DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2 is used.
                sapp_win32_dpi_changed(hwnd, (LPRECT)lParam);
            }
            WM_LBUTTONDOWN => {
                sapp_win32_mouse_update(lParam);
                sapp.call_event(&Event::Mouse(MouseEvent{ pressed: true, mouse_button: MouseButton::Left, modifiers: Modifier::empty() }));
                sapp_win32_capture_mouse(1<<SAPP_MOUSEBUTTON_LEFT);
            }
            WM_RBUTTONDOWN => {
                sapp_win32_mouse_update(lParam);
                sapp.call_event(&Event::Mouse(MouseEvent{ pressed: true, mouse_button: MouseButton::Right, modifiers: Modifier::empty() }));                
                sapp_win32_capture_mouse(1<<SAPP_MOUSEBUTTON_RIGHT);
            }
            WM_MBUTTONDOWN => {
                sapp_win32_mouse_update(lParam);
                sapp.call_event(&Event::Mouse(MouseEvent{ pressed: true, mouse_button: MouseButton::Middle, modifiers: Modifier::empty() }));                
                sapp_win32_capture_mouse(1<<SAPP_MOUSEBUTTON_MIDDLE);
            }
            WM_LBUTTONUP => {
                sapp_win32_mouse_update(lParam);
                sapp.call_event(&Event::Mouse(MouseEvent{ pressed: false, mouse_button: MouseButton::Left, modifiers: Modifier::empty() }));
                sapp_win32_release_mouse(1<<SAPP_MOUSEBUTTON_LEFT);
            }
            WM_RBUTTONUP => {
                _sapp_win32_mouse_update(lParam);
                sapp.call_event(&Event::Mouse(MouseEvent{ pressed: false, mouse_button: MouseButton::Right, modifiers: Modifier::empty() }));                
                _sapp_win32_release_mouse(1<<SAPP_MOUSEBUTTON_RIGHT);
            }
            WM_MBUTTONUP => {
                _sapp_win32_mouse_update(lParam);
                sapp.call_event(&Event::Mouse(MouseEvent{ pressed: false, mouse_button: MouseButton::Middle, modifiers: Modifier::empty() }));                
                _sapp_win32_release_mouse(1<<SAPP_MOUSEBUTTON_MIDDLE);
            }
            WM_MOUSEMOVE => {
                if !sapp.base.mouse.locked {
                    sapp_win32_mouse_update(lParam);
                    if !sapp.base.win32.mouse_tracked {
                        sapp.base.win32.mouse_tracked = true;
                        let mut tme = TRACKMOUSEEVENT{ 
                            cbSize: std::mem::size_of::<TRACKMOUSEEVENT>() as u32,
                            dwFlags: TME_LEAVE,
                            hwndTrack: sapp.base.win32.hwnd,
                            dwHoverTime: 0};
                        TrackMouseEvent(&mut tme);
                        sapp.call_event(&Event::MouseEnter(Modifier::empty()));
                    }
                    sapp.call_event(&Event::MouseMove(Modifier::empty()));
                }
            }
            WM_INPUT => {
                /* raw mouse input during mouse-lock */
                if sapp.base.mouse.locked {
                    let ri = lparam as HRAWINPUT;
                    let size : u32 = sapp.base.win32.raw_input_data.len() as u32;
                    let ptr = &mut sapp.base.win32.raw_input_data as *mut u8 as *mut core::ffi::c_void;
                    // see: https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getrawinputdata
                    if -1i32 as u32 != GetRawInputData(ri, RID_INPUT, ptr, &mut size, std::mem::size_of::<RAWINPUTHEADER>() as u32) {
                        let raw_mouse_data = ptr as *const RAWINPUT; // DT_TODO: Check casting of this - use local var of just type RAWINPUT? Link seems to indicate you can do this?
                        let raw_data = &(*raw_mouse_data).data;
                        if (raw_data.mouse.usFlags as u32 & MOUSE_MOVE_ABSOLUTE) != 0 {
                            // mouse only reports absolute position
                            //   NOTE: THIS IS UNTESTED, it's unclear from reading the
                            //   Win32 RawInput docs under which circumstances absolute
                            //   positions are sent.
                            if sapp.base.win32.raw_input_mousepos_valid {
                                let new_x = raw_data.mouse.lLastX;
                                let new_y = raw_data.mouse.lLastY;
                                sapp.base.mouse.dx = (new_x - sapp.base.win32.raw_input_mousepos_x) as f32;
                                sapp.base.mouse.dy = (new_y - sapp.base.win32.raw_input_mousepos_y) as f32;
                                sapp.base.win32.raw_input_mousepos_x = new_x;
                                sapp.base.win32.raw_input_mousepos_y = new_y;
                                sapp.base.win32.raw_input_mousepos_valid = true;
                            }
                        }
                        else {
                            // mouse reports movement delta (this seems to be the common case)
                            sapp.base.mouse.dx = raw_data.mouse.lLastX as f32;
                            sapp.base.mouse.dy = raw_data.mouse.lLastY as f32;
                        }
                        sapp.call_event(&Event::MouseMove(Modifier::empty()));
                    }

                    //else _SAPP_ERROR(WIN32_GET_RAW_INPUT_DATA_FAILED); // DT_TODO:

                }
            }

            WM_MOUSELEAVE => {
                if !sapp.base.mouse.locked {
                    sapp.base.win32.mouse_tracked = false;
                    sapp.call_event(&Event::MouseLeave);
                }
            }
            WM_MOUSEWHEEL => {
                sapp_win32_mouse_update(lParam);
                sapp.call_event(&Event::MouseScroll(MouseScrollEvent{scroll_x : 0.0, scroll_y : HIWORD(wparam as u32) as f32 / 30.0 }));
            }
            WM_MOUSEHWHEEL => {
                _sapp_win32_mouse_update(lParam);
                sapp.call_event(&Event::MouseScroll(MouseScrollEvent{scroll_x : HIWORD(wparam as u32) as f32 / -30.0, scroll_y : 0.0 }));                
            }
            WM_CHAR => _sapp_win32_char_event((uint32_t)wParam, !!(lParam&0x40000000)),
            WM_KEYDOWN | WM_SYSKEYDOWN => sapp_win32_key_event(SAPP_EVENTTYPE_KEY_DOWN, (int)(HIWORD(lParam)&0x1FF), !!(lParam&0x40000000)),
            WM_KEYUP | WM_SYSKEYUP => sapp_win32_key_event(SAPP_EVENTTYPE_KEY_UP, (int)(HIWORD(lParam)&0x1FF), false),
            WM_ENTERSIZEMOVE => SetTimer(sapp.base.win32.hwnd, 1, USER_TIMER_MINIMUM, None),
            WM_EXITSIZEMOVE => KillTimer(sapp.base.win32.hwnd, 1),

            WM_TIMER => {
                sapp_win32_timing_measure();
                sapp_frame();
                sapp_wgl_swap_buffers();

                /* NOTE: resizing the swap-chain during resize leads to a substantial
                   memory spike (hundreds of megabytes for a few seconds).
                if (_sapp_win32_update_dimensions()) {
                    _sapp_win32_app_event(SAPP_EVENTTYPE_RESIZED);
                }
                */
            }
            WM_NCLBUTTONDOWN => {
                // workaround for half-second pause when starting to move window
                //    see: https://gamedev.net/forums/topic/672094-keeping-things-moving-during-win32-moveresize-events/5254386/
                if SendMessageW(sapp.base.win32.hwnd, WM_NCHITTEST, wparam, lparam) == HTCAPTION as isize {
                    let mut point = POINT { x: 0, y:0 };
                    GetCursorPos(&mut point);
                    ScreenToClient(sapp.base.win32.hwnd, &mut point);
                    PostMessageW(sapp.base.win32.hwnd, WM_MOUSEMOVE, 0, ((point.x as u32)|((point.y as u32) << 16)) as isize);
                }
            }
            WM_DROPFILES => sapp_win32_files_dropped((HDROP)wParam),
            WM_DISPLAYCHANGE => sapp_timing_reset(&_sapp.timing), // refresh rate might have changed

            _ => {}
        }

    return DefWindowProcW(window, message, wparam, lparam);
}


unsafe fn sapp_win32_init_dpi(sapp: &mut SAppData) {
    // Manually loading entry points to support old windows versions - perhaps gate this behind a modern window feature version?
    type FnSetProcessDPIAwareT = extern "system" fn() -> BOOL;
    type FnSetProcessDPIAwarenessT = extern "system" fn(value: PROCESS_DPI_AWARENESS) -> HRESULT;
    type FnSetProcessDPIAwarenessContextT =
        extern "system" fn(value: DPI_AWARENESS_CONTEXT) -> BOOL; // since Windows 10, version 1703
    type FnGetDpiForMonitorT = extern "system" fn(
        hmonitor: HMONITOR,
        dpitype: MONITOR_DPI_TYPE,
        dpix: *mut u32,
        dpiy: *mut u32,
    ) -> HRESULT;

    let mut fn_set_process_dpi_aware = Option::<FnSetProcessDPIAwareT>::None;
    let mut fn_set_process_dpi_awareness = Option::<FnSetProcessDPIAwarenessT>::None;
    let mut fn_set_process_dpi_awareness_context = Option::<FnSetProcessDPIAwarenessContextT>::None;
    let mut fn_get_dpi_for_monitor = Option::<FnGetDpiForMonitorT>::None;

    let user32 = LoadLibraryA(s!("user32.dll"));
    if user32 != 0 {
        fn_set_process_dpi_aware =
            std::mem::transmute(GetProcAddress(user32, s!("SetProcessDPIAware")));
        fn_set_process_dpi_awareness_context =
            std::mem::transmute(GetProcAddress(user32, s!("SetProcessDpiAwarenessContext")));
    }
    let shcore = LoadLibraryA(s!("shcore.dll"));
    if shcore != 0 {
        fn_set_process_dpi_awareness =
            std::mem::transmute(GetProcAddress(shcore, s!("SetProcessDpiAwareness")));
        fn_get_dpi_for_monitor = std::mem::transmute(GetProcAddress(shcore, s!("GetDpiForMonitor")));
    }

    // NOTE on SetProcessDpiAware() vs SetProcessDpiAwareness() vs SetProcessDpiAwarenessContext():
    //
    // These are different attempts to get DPI handling on Windows right, from oldest
    // to newest. SetProcessDpiAwarenessContext() is required for the new
    // DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2 method.
    if let Some(set_process_dpi_awareness) = fn_set_process_dpi_awareness {
        if sapp.desc.high_dpi {

            // app requests HighDPI rendering, first try the Win10 Creator Update per-monitor-dpi awareness,
            // if that fails, fall back to system-dpi-awareness
            sapp.win32.dpi.aware = true;
            let per_monitor_aware_v2: DPI_AWARENESS_CONTEXT = -4;
            if let Some(set_process_dpi_awareness_context) = fn_set_process_dpi_awareness_context {
                if set_process_dpi_awareness_context(per_monitor_aware_v2) == FALSE {
                    set_process_dpi_awareness(PROCESS_SYSTEM_DPI_AWARE);
                }
            } else {
                set_process_dpi_awareness(PROCESS_SYSTEM_DPI_AWARE);
            }
        } else {
            // if the app didn't request HighDPI rendering, let Windows do the upscaling
            sapp.win32.dpi.aware = false;
            set_process_dpi_awareness(PROCESS_DPI_UNAWARE);
        }
    } else if let Some(set_process_dpi_aware) = fn_set_process_dpi_aware {
        // fallback for Windows 7
        sapp.win32.dpi.aware = true;
        set_process_dpi_aware();
    }
    // get dpi scale factor for main monitor
    sapp.win32.dpi.window_scale = 1.0;
    if let Some(get_dpi_for_monitor) = fn_get_dpi_for_monitor {
        if sapp.win32.dpi.aware {
            let pt: POINT = POINT { x: 1, y: 1 };
            let hm: HMONITOR = MonitorFromPoint(pt, MONITOR_DEFAULTTONEAREST);
            let mut dpix: u32 = 0;
            let mut dpiy: u32 = 0;
            let hr: HRESULT = get_dpi_for_monitor(hm, MDT_EFFECTIVE_DPI, &mut dpix, &mut dpiy);
            debug_assert!(hr >= 0);
            // clamp window scale to an integer factor
            sapp.win32.dpi.window_scale = dpix as f32 / 96.0;
        }
    }

    if sapp.desc.high_dpi {
        sapp.win32.dpi.content_scale = sapp.win32.dpi.window_scale;
        sapp.win32.dpi.mouse_scale = 1.0;
    } else {
        sapp.win32.dpi.content_scale = 1.0;
        sapp.win32.dpi.mouse_scale = 1.0 / sapp.win32.dpi.window_scale;
    }

    sapp.dpi_scale = sapp.win32.dpi.content_scale;
    if user32 != 0 {
        FreeLibrary(user32);
    }
    if shcore != 0 {
        FreeLibrary(shcore);
    }
}

unsafe fn sapp_win32_update_dimensions(sapp: &mut SAppData) -> bool {
    let mut rect = RECT {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };
    if GetClientRect(sapp.win32.hwnd, &mut rect) == TRUE {
        let window_width = (rect.right - rect.left) as f32 / sapp.win32.dpi.window_scale;
        let window_height = (rect.bottom - rect.top) as f32 / sapp.win32.dpi.window_scale;
        sapp.window_width = window_width.round() as u32;
        sapp.window_height = window_height.round() as u32;
        let mut fb_width = (window_width as f32 * sapp.win32.dpi.content_scale).round() as u32;
        let mut fb_height = (window_height as f32 * sapp.win32.dpi.content_scale).round() as u32;

        /* prevent a framebuffer size of 0 when window is minimized */
        if 0 == fb_width {
            fb_width = 1;
        }
        if 0 == fb_height {
            fb_height = 1;
        }
        if (fb_width != sapp.framebuffer_width) || (fb_height != sapp.framebuffer_height) {
            sapp.framebuffer_width = fb_width;
            sapp.framebuffer_height = fb_height;
            return true;
        }
    } else {
        sapp.window_width = 1;
        sapp.window_height = 1;
        sapp.framebuffer_width = 1;
        sapp.framebuffer_height = 1;
    }
    return false;
}

unsafe fn sapp_win32_create_window(sapp: &mut SAppData) {
    let instance = GetModuleHandleW(std::ptr::null());
    let wndclassw = WNDCLASSW {
        hCursor: LoadCursorW(0, IDC_ARROW),
        hInstance: instance,
        lpszClassName: w!("SOKOLAPP"),
        style: CS_HREDRAW | CS_VREDRAW | CS_OWNDC,
        lpfnWndProc: Some(wndproc),
        cbClsExtra: 0,
        cbWndExtra: 0,
        hIcon: LoadIconW(0, IDI_WINLOGO),
        hbrBackground: 0,
        lpszMenuName: std::ptr::null(),
    };
    RegisterClassW(&wndclassw);

    /* NOTE: regardless whether fullscreen is requested or not, a regular
       windowed-mode window will always be created first (however in hidden
       mode, so that no windowed-mode window pops up before the fullscreen window)
    */
    let win_ex_style = WS_EX_APPWINDOW | WS_EX_WINDOWEDGE;
    let mut rect = RECT {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };
    let win_style = WS_CLIPSIBLINGS
        | WS_CLIPCHILDREN
        | WS_CAPTION
        | WS_SYSMENU
        | WS_MINIMIZEBOX
        | WS_MAXIMIZEBOX
        | WS_SIZEBOX;

    rect.right = (sapp.window_width as f32 * sapp.win32.dpi.window_scale) as i32;
    rect.bottom = (sapp.window_height as f32 * sapp.win32.dpi.window_scale) as i32;
    let use_default_width = 0 == sapp.window_width;
    let use_default_height = 0 == sapp.window_height;
    AdjustWindowRectEx(&mut rect, win_style, FALSE, win_ex_style);
    let win_width = rect.right - rect.left;
    let win_height = rect.bottom - rect.top;
    sapp.win32.in_create_window = true;

    // DT_TODO: See about setting active code page in the manifest to utf8 to not have to do this
    // UTF16 null terminated string
    let mut title = Vec::with_capacity(sapp.desc.window_title.encode_utf16().count() + 1);
    title.extend(sapp.desc.window_title.encode_utf16());
    title.push(0);

    sapp.win32.hwnd = CreateWindowExW(
        win_ex_style,   // dwExStyle
        w!("SOKOLAPP"), // lpClassName
        title.as_ptr(), // lpWindowName
        win_style,      // dwStyle
        CW_USEDEFAULT,  // X
        SW_HIDE as i32, // Y (NOTE: CW_USEDEFAULT is not used for position here, but internally calls ShowWindow!
        if use_default_width {
            CW_USEDEFAULT
        } else {
            win_width
        }, // nWidth
        if use_default_height {
            CW_USEDEFAULT
        } else {
            win_height
        }, // nHeight (NOTE: if width is CW_USEDEFAULT, height is actually ignored)
        0,              // hWndParent
        0,              // hMenu
        instance,       // hInstance
        std::ptr::null(),
    ); // lParam
    sapp.win32.in_create_window = false;
    sapp.win32.dc = GetDC(sapp.win32.hwnd);
    sapp.win32.hmonitor = MonitorFromWindow(sapp.win32.hwnd, MONITOR_DEFAULTTONULL);
    debug_assert!(sapp.win32.dc != 0);

    /* this will get the actual windowed-mode window size, if fullscreen
       is requested, the set_fullscreen function will then capture the
       current window rectangle, which then might be used later to
       restore the window position when switching back to windowed
    */
    sapp_win32_update_dimensions(sapp);
    if sapp.fullscreen {
        //_sapp_win32_set_fullscreen(_sapp.fullscreen, SWP_HIDEWINDOW);
        //sapp_win32_update_dimensions(sapp);
    }
    ShowWindow(sapp.win32.hwnd, SW_SHOW);
    //DragAcceptFiles(sapp.win32.hwnd, 1);
}

struct DPI {
    aware: bool,
    content_scale: f32,
    window_scale: f32,
    mouse_scale: f32,
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
    dpi: DPI,
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
            dpi: DPI {
                aware: false,
                content_scale: 0.0,
                window_scale: 0.0,
                mouse_scale: 0.0,
            },
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
            width: 0,
            height: 0,

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

impl<'a, T> SApp<'a, T>
where T: SAppI
{
    fn call_event(&mut self, event : &Event) {
        if !self.base.cleanup_called && self.base.init_called {
            self.app.on_event(&mut self.base, event);
        }
    }
} 


pub fn run_app<T>(app: T, desc: SAppDesc)
where
    T: SAppI,
{
    let mut b = SApp {
        base: SAppData::new(desc),
        app,
    };

    //_sapp_win32_init_console();
    //_sapp.win32.is_win10_or_greater = _sapp_win32_is_win10_or_greater();
    sapp_win32_init_keytable(&mut b.base.keycodes);
    //_sapp_win32_utf8_to_wide(_sapp.window_title, _sapp.window_title_wide, sizeof(_sapp.window_title_wide));
    unsafe {
        sapp_win32_init_dpi(&mut b.base);
    }
    //_sapp_win32_init_cursors();
    unsafe {
        sapp_win32_create_window(&mut b.base);
    }
    //sapp_set_icon(&desc->icon);
    //_sapp_wgl_init();
    //_sapp_wgl_load_extensions();
    //_sapp_wgl_create_context();
    b.base.valid = true;

    let user_data = &mut b;
    let ptr = user_data as *mut SApp<T>;

    unsafe {
        // DT_TODO: check safety of doing this
        SetWindowLongPtrW(b.base.win32.hwnd, GWL_USERDATA, ptr as isize);
    }

    let mut done = false;
    while !done && !b.base.quit_ordered {
        //_sapp_win32_timing_measure();
        unsafe {

            let mut msg: MSG = std::mem::zeroed();
            while PeekMessageW(&mut msg, 0, 0, 0, PM_REMOVE) == TRUE {
                if WM_QUIT == msg.message {
                    done = true;
                    continue;
                } else {
                    TranslateMessage(&msg);
                    DispatchMessageW(&msg);
                }
            }
        }
        //_sapp_frame();
        //_sapp_wgl_swap_buffers();

        /* check for window resized, this cannot happen in WM_SIZE as it explodes memory usage */
        //if (_sapp_win32_update_dimensions()) {
        //    _sapp_win32_app_event(SAPP_EVENTTYPE_RESIZED);
        //}
        /* check if the window monitor has changed, need to reset timing because
           the new monitor might have a different refresh rate
        */
        //if (_sapp_win32_update_monitor()) {
        //    _sapp_timing_reset(&_sapp.timing);
        //}
        if b.base.quit_requested {
            unsafe {
                PostMessageW(b.base.win32.hwnd, WM_CLOSE, 0, 0);
            }
        }
    }
    //DT_TODO Unset pointer on the window
    //SetWindowLongPtrW(b.base.win32.hwnd, GWL_USERDATA, 0);

    //_sapp_call_cleanup();

    //_sapp_wgl_destroy_context();
    //_sapp_wgl_shutdown();
    //_sapp_win32_destroy_window();
    //_sapp_win32_destroy_icons();
    //_sapp_win32_restore_console();
    //_sapp_discard_state();
}

impl<'a, T> SApp<'a, T> where T: SAppI {}

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
