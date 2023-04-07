use bitflags::bitflags;
use windows_sys::core::*;
use windows_sys::Win32::Devices::HumanInterfaceDevice::MOUSE_MOVE_ABSOLUTE;
use windows_sys::Win32::Foundation::*;
use windows_sys::Win32::Globalization::*;
use windows_sys::Win32::Graphics::Gdi::*;
use windows_sys::Win32::Graphics::OpenGL::*;
use windows_sys::Win32::System::Console::*;
use windows_sys::Win32::System::DataExchange::*;
use windows_sys::Win32::System::LibraryLoader::{
    FreeLibrary, GetModuleHandleW, GetProcAddress, LoadLibraryA,
};
use windows_sys::Win32::System::Memory::{
    GlobalAlloc, GlobalFree, GlobalLock, GlobalUnlock, GMEM_MOVEABLE,
};
use windows_sys::Win32::System::Ole::CF_UNICODETEXT;
use windows_sys::Win32::UI::Controls::WM_MOUSELEAVE;
use windows_sys::Win32::UI::HiDpi::*;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::*;
use windows_sys::Win32::UI::Input::{
    GetRawInputData, RegisterRawInputDevices, HRAWINPUT, RAWINPUT, RAWINPUTDEVICE, RAWINPUTHEADER,
    RIDEV_REMOVE, RID_INPUT,
};
use windows_sys::Win32::UI::Shell::{DragAcceptFiles, DragFinish, DragQueryFileW, HDROP};
use windows_sys::Win32::UI::WindowsAndMessaging::*;

use crate::enum_sequential;
use crate::static_assert;
use crate::EnumLoadError;
use crate::Timer;

#[inline]
pub fn LOWORD(l: u32) -> u16 {
    (l & 0xffff) as u16
}

#[inline]
pub fn HIWORD(l: u32) -> u16 {
    ((l >> 16) & 0xffff) as u16
}

#[inline]
pub fn GET_X_LPARAM(lp: LPARAM) -> i16 {
    LOWORD(lp as u32) as i16
}
#[inline]
pub fn GET_Y_LPARAM(lp: LPARAM) -> i16 {
    HIWORD(lp as u32) as i16
}

#[inline]
pub fn MAKEINTRESOURCEW(i: u16) -> PCWSTR {
    i as usize as PCWSTR
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
}

enum_sequential! {
    #[derive(PartialEq, Clone, Copy)]
    pub enum MouseCursor {
        Default,   // equivalent with system default cursor
        Arrow,
        Ibeam,
        Crosshair,
        PointingHand,
        ResizeEW,
        ResizeNS,
        ResizeNWSE,
        ResizeNESW,
        ResizeAll,
        NotAllowed,
    }
}
const SAPP_MOUSECURSOR_NUM: u32 = MouseCursor::len() as u32;

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
    pub pressed: bool,     // true if the key is pressed
    pub key_code: KeyCode, // the virtual key code, only valid in KEY_UP, KEY_DOWN
    pub key_repeat: bool,  // true if this is a key-repeat event, valid in KEY_UP, KEY_DOWN and CHAR
}

pub struct CharEvent {
    pub char_code: char,  // the UTF-32 character code, only valid in CHAR events
    pub key_repeat: bool, // true if this is a key-repeat event, valid in KEY_UP, KEY_DOWN and CHAR
}

pub struct MouseEvent {
    pub pressed: bool,             // true if the mouse is pressed
    pub mouse_button: MouseButton, // mouse button that was pressed or released, valid in MOUSE_DOWN, MOUSE_UP
}

pub struct MouseMoveEvent {
    pub mouse_dx: f32, // relative horizontal mouse movement
    pub mouse_dy: f32, // relative vertical mouse movement
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
    MouseMove(MouseMoveEvent),
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

pub trait SAppI {
    fn init(&mut self, _app: &mut SAppData) {}

    fn draw_frame(&mut self, _app: &mut SAppData) {}

    fn on_event(&mut self, _app: &mut SAppData, _event: &Event) {}

    fn shutdown(&mut self, _app: &mut SAppData) {}
}

//const SAPP_MAX_TOUCHPOINTS: u32 = 8;
//const SAPP_MAX_MOUSEBUTTONS: u32 = 3;
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
    current_cursor: MouseCursor, // Cursor icon enum
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
            current_cursor: MouseCursor::Default,
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

fn sapp_win32_capture_mouse(sapp: &mut SAppData, btn_mask: u8) {
    if 0 == sapp.win32.mouse_capture_mask {
        unsafe {
            SetCapture(sapp.win32.hwnd);
        }
    }
    sapp.win32.mouse_capture_mask |= btn_mask;
}

fn sapp_win32_release_mouse(sapp: &mut SAppData, btn_mask: u8) {
    if 0 != sapp.win32.mouse_capture_mask {
        sapp.win32.mouse_capture_mask &= !btn_mask;
        if 0 == sapp.win32.mouse_capture_mask {
            unsafe {
                ReleaseCapture();
            }
        }
    }
}

unsafe fn sapp_win32_lock_mouse(sapp: &mut SAppData, lock: bool) {
    if lock == sapp.mouse.locked {
        return;
    }
    sapp.mouse.dx = 0.0;
    sapp.mouse.dy = 0.0;
    sapp.mouse.locked = lock;
    sapp_win32_release_mouse(sapp, 0xFF);
    if sapp.mouse.locked {
        // store the current mouse position, so it can be restored when unlocked
        let mut pos = POINT { x: 0, y: 0 };
        let res = GetCursorPos(&mut pos);
        debug_assert!(res == TRUE);
        sapp.win32.mouse_locked_x = pos.x;
        sapp.win32.mouse_locked_y = pos.y;

        // while the mouse is locked, make the mouse cursor invisible and
        // confine the mouse movement to a small rectangle inside our window
        // (so that we dont miss any mouse up events)
        let client_rect = RECT {
            left: sapp.win32.mouse_locked_x,
            top: sapp.win32.mouse_locked_y,
            right: sapp.win32.mouse_locked_x,
            bottom: sapp.win32.mouse_locked_y,
        };
        ClipCursor(&client_rect);

        // make the mouse cursor invisible, this will stack with sapp_show_mouse()
        ShowCursor(FALSE);

        // enable raw input for mouse, starts sending WM_INPUT messages to WinProc (see GLFW)
        let rid = RAWINPUTDEVICE {
            usUsagePage: 0x01,           // usUsagePage: HID_USAGE_PAGE_GENERIC
            usUsage: 0x02,               // usUsage: HID_USAGE_GENERIC_MOUSE
            dwFlags: 0,                  // dwFlags
            hwndTarget: sapp.win32.hwnd, // hwndTarget
        };

        if RegisterRawInputDevices(&rid, 1, std::mem::size_of::<RAWINPUTDEVICE>() as u32) != TRUE {
            //_SAPP_ERROR(WIN32_REGISTER_RAW_INPUT_DEVICES_FAILED_MOUSE_LOCK);
        }
        // in case the raw mouse device only supports absolute position reporting,
        // we need to skip the dx/dy compution for the first WM_INPUT event
        sapp.win32.raw_input_mousepos_valid = false;
    } else {
        // disable raw input for mouse
        let rid = RAWINPUTDEVICE {
            usUsagePage: 0x01,
            usUsage: 0x02,
            dwFlags: RIDEV_REMOVE,
            hwndTarget: 0,
        };
        if RegisterRawInputDevices(&rid, 1, std::mem::size_of::<RAWINPUTDEVICE>() as u32) != TRUE {
            //_SAPP_ERROR(WIN32_REGISTER_RAW_INPUT_DEVICES_FAILED_MOUSE_UNLOCK);
        }

        // let the mouse roam freely again
        ClipCursor(std::ptr::null());
        ShowCursor(TRUE);

        // restore the 'pre-locked' mouse position
        let res = SetCursorPos(sapp.win32.mouse_locked_x, sapp.win32.mouse_locked_y);
        debug_assert!(res == TRUE);
    }
}

fn sapp_win32_mouse_update(sapp: &mut SAppData, lParam: LPARAM) {
    if !sapp.mouse.locked {
        let new_x = GET_X_LPARAM(lParam) as f32 * sapp.win32.dpi.mouse_scale;
        let new_y = GET_Y_LPARAM(lParam) as f32 * sapp.win32.dpi.mouse_scale;
        if sapp.mouse.pos_valid {
            // don't update dx/dy in the very first event
            sapp.mouse.dx = new_x - sapp.mouse.x;
            sapp.mouse.dy = new_y - sapp.mouse.y;
        }
        sapp.mouse.x = new_x;
        sapp.mouse.y = new_y;
        sapp.mouse.pos_valid = true;
    }
}

fn sapp_win32_init_cursor(sapp: &mut SAppData, cursor: MouseCursor) {
    let id = match cursor {
        MouseCursor::Default => OCR_NORMAL,
        MouseCursor::Arrow => OCR_NORMAL,
        MouseCursor::Ibeam => OCR_IBEAM,
        MouseCursor::Crosshair => OCR_CROSS,
        MouseCursor::PointingHand => OCR_HAND,
        MouseCursor::ResizeEW => OCR_SIZEWE,
        MouseCursor::ResizeNS => OCR_SIZENS,
        MouseCursor::ResizeNWSE => OCR_SIZENWSE,
        MouseCursor::ResizeNESW => OCR_SIZENESW,
        MouseCursor::ResizeAll => OCR_SIZEALL,
        MouseCursor::NotAllowed => OCR_NO,
    };

    let cursor_index = cursor as usize;
    if id != 0 {
        sapp.win32.cursors[cursor_index] = unsafe {
            LoadImageW(
                0,
                MAKEINTRESOURCEW(id as u16),
                IMAGE_CURSOR,
                0,
                0,
                LR_DEFAULTSIZE | LR_SHARED,
            )
        };
    }
    // fallback: default cursor
    if 0 == sapp.win32.cursors[cursor_index] {
        sapp.win32.cursors[cursor_index] =
            unsafe { LoadCursorW(0, MAKEINTRESOURCEW(IDC_ARROW as u16)) };
    }
    debug_assert!(0 != sapp.win32.cursors[cursor_index]);
}

fn sapp_win32_init_cursors(sapp: &mut SAppData) {
    for i in MouseCursor::iter() {
        sapp_win32_init_cursor(sapp, i);
    }
}

unsafe fn sapp_win32_cursor_in_content_area(sapp: &SAppData) -> bool {
    let mut pos = POINT { x: 0, y: 0 };
    if GetCursorPos(&mut pos) == 0 {
        return false;
    }
    if WindowFromPoint(pos) != sapp.win32.hwnd {
        return false;
    }
    let mut area = RECT {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };
    GetClientRect(sapp.win32.hwnd, &mut area);

    let mut left_top = POINT {
        x: area.left,
        y: area.top,
    };
    let mut right_bottom = POINT {
        x: area.right,
        y: area.bottom,
    };
    ClientToScreen(sapp.win32.hwnd, &mut left_top);
    ClientToScreen(sapp.win32.hwnd, &mut right_bottom);
    area.left = left_top.x;
    area.top = left_top.y;
    area.right = right_bottom.x;
    area.bottom = right_bottom.y;
    return PtInRect(&area, pos) == TRUE;
}

fn sapp_win32_update_cursor(
    sapp: &SAppData,
    cursor: MouseCursor,
    shown: bool,
    skip_area_test: bool,
) {
    // NOTE: when called from WM_SETCURSOR, the area test would be redundant
    unsafe {
        if !skip_area_test {
            if !sapp_win32_cursor_in_content_area(sapp) {
                return;
            }
        }
        if !shown {
            SetCursor(0);
        } else {
            debug_assert!(0 != sapp.win32.cursors[cursor as usize]);
            SetCursor(sapp.win32.cursors[cursor as usize]);
        }
    }
}

unsafe fn sapp_win32_dpi_changed(sapp: &mut SAppData, hwnd: HWND, proposed_win_rect: &RECT) {
    // called on WM_DPICHANGED, which will only be sent to the application
    //    if sapp_desc.high_dpi is true and the Windows version is recent enough
    //    to support DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2
    debug_assert!(sapp.high_dpi);
    let user32 = LoadLibraryA(s!("user32.dll"));
    if user32 == 0 {
        return;
    }

    type FnGetDpiForWindowT = extern "system" fn(hwnd: HWND) -> u32;
    let fn_get_dpi_for_window: Option<FnGetDpiForWindowT> =
        std::mem::transmute(GetProcAddress(user32, s!("GetDpiForWindow")));

    if let Some(get_dpi_for_window) = fn_get_dpi_for_window {
        let dpix = get_dpi_for_window(sapp.win32.hwnd);
        // NOTE: for high-dpi apps, mouse_scale remains one
        sapp.win32.dpi.window_scale = dpix as f32 / 96.0;
        sapp.win32.dpi.content_scale = sapp.win32.dpi.window_scale;
        sapp.dpi_scale = sapp.win32.dpi.window_scale;
        SetWindowPos(
            hwnd,
            0,
            proposed_win_rect.left,
            proposed_win_rect.top,
            proposed_win_rect.right - proposed_win_rect.left,
            proposed_win_rect.bottom - proposed_win_rect.top,
            SWP_NOZORDER | SWP_NOACTIVATE,
        );
    }
    FreeLibrary(user32);
}

unsafe fn sapp_win32_set_clipboard_string(sapp: &SAppData, str: &str) -> bool {
    debug_assert!(sapp.win32.hwnd != 0);
    debug_assert!(sapp.clipboard.enabled && (sapp.clipboard.buffer.capacity() > 0));

    if OpenClipboard(sapp.win32.hwnd) == FALSE {
        return false;
    }

    let str_utf16_length = str.encode_utf16().count();

    let wchar_buf_size = (str_utf16_length + 1) * std::mem::size_of::<u16>();
    let object = GlobalAlloc(GMEM_MOVEABLE, wchar_buf_size);
    if object == 0 {
        CloseClipboard();
        return false;
    }

    // Lock and unlock to copy the buffer
    {
        let mut wchar_buf = GlobalLock(object) as *mut u16;
        if wchar_buf == std::ptr::null_mut() {
            GlobalFree(object);
            CloseClipboard();
            return false;
        }

        let mut utf16_iter = str.encode_utf16();
        while let Some(val) = utf16_iter.next() {
            *wchar_buf = val;
            wchar_buf = wchar_buf.add(1);
        }
        *wchar_buf = 0; // Add null terminator

        GlobalUnlock(object);
    }

    EmptyClipboard();

    // NOTE: when successful, SetClipboardData() takes ownership of memory object!
    if SetClipboardData(CF_UNICODETEXT as u32, object) == 0 {
        GlobalFree(object);
        CloseClipboard();
        return false;
    }
    CloseClipboard();

    return true;
}

struct CStringIterator {
    string: *const u8,
}
impl Iterator for CStringIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let val = *self.string;

            if val == 0 {
                None
            } else {
                self.string = self.string.add(1);
                Some(val)
            }
        }
    }
}

struct CWideStringIterator {
    string: *const u16,
}
impl Iterator for CWideStringIterator {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let val = *self.string;

            if val == 0 {
                None
            } else {
                self.string = self.string.add(1);
                Some(val)
            }
        }
    }
}

unsafe fn sapp_win32_get_clipboard_string(sapp: &mut SAppData) {
    debug_assert!(sapp.clipboard.enabled);
    debug_assert!(sapp.win32.hwnd != 0);
    if OpenClipboard(sapp.win32.hwnd) == FALSE {
        // silently ignore any errors and just return the current
        //   content of the local clipboard buffer
        return;
    }
    let object = GetClipboardData(CF_UNICODETEXT as u32);
    if object == 0 {
        CloseClipboard();
        return;
    }
    let wchar_buf = GlobalLock(object) as *const u16;
    if wchar_buf == std::ptr::null() {
        CloseClipboard();
        return;
    }

    // Add all the characters into the buffer until the limit (or bad data) // DT_TODO: Stop at clipboard capacity?
    sapp.clipboard.buffer.clear();
    for c in char::decode_utf16(CWideStringIterator { string: wchar_buf }) {
        if let Ok(c) = c {
            sapp.clipboard.buffer.push(c);
        } else {
            break;
        }
    }
    GlobalUnlock(object);
    CloseClipboard();
}

fn sapp_win32_files_dropped(sapp: &mut SApp, hdrop: HDROP) {
    if sapp.base.drop.max_files <= 0 {
        return;
    }

    let mut drop_failed = false;
    unsafe {
        let mut count = DragQueryFileW(hdrop, 0xffffffff, std::ptr::null_mut(), 0);
        if count > sapp.base.drop.max_files {
            count = sapp.base.drop.max_files;
        }
        sapp.base
            .drop
            .file_paths
            .resize(count as usize, String::new());

        // Get max needed buffer size
        let mut max_chars = 0;
        for i in 0..count {
            let num_chars = DragQueryFileW(hdrop, i, std::ptr::null_mut(), 0) + 1; // DragQueryFileW expects space for a null terminator
            if num_chars > max_chars {
                max_chars = num_chars;
            }
        }

        // Get each filename
        let mut buffer = vec![0; max_chars as usize];
        for i in 0..count {
            let num_chars = DragQueryFileW(hdrop, i, buffer.as_mut_ptr(), max_chars);
            if num_chars == 0 {
                drop_failed = true;
                break;
            }
            let str = &mut sapp.base.drop.file_paths[i as usize];
            str.clear();
            str.reserve(num_chars as usize + 10);
            for c in char::decode_utf16(buffer[..num_chars as usize].iter().copied()) {
                if let Ok(c) = c {
                    str.push(c);
                } else {
                    break;
                }
            }
        }
        DragFinish(hdrop);
    }
    if !drop_failed {
        sapp.call_event(&Event::FilesDropped);
    } else {
        sapp.base.drop.file_paths.clear();
    }
}

unsafe fn sapp_win32_mods() -> Modifier {
    let mut mods = Modifier::empty();
    if (GetKeyState(VK_SHIFT as i32) & (1 << 15)) != 0 {
        mods |= Modifier::Shift;
    }
    if (GetKeyState(VK_CONTROL as i32) & (1 << 15)) != 0 {
        mods |= Modifier::Ctrl;
    }
    if (GetKeyState(VK_MENU as i32) & (1 << 15)) != 0 {
        mods |= Modifier::Alt;
    }
    if ((GetKeyState(VK_LWIN as i32) | GetKeyState(VK_RWIN as i32)) & (1 << 15)) != 0 {
        mods |= Modifier::Super;
    }
    let swapped = TRUE == GetSystemMetrics(SM_SWAPBUTTON);
    if GetAsyncKeyState(VK_LBUTTON as i32) != 0 {
        // DT_TODO: Should this use GetAsyncKeyState() here? Should it only check the top bit as above?
        mods |= if swapped {
            Modifier::Rmb
        } else {
            Modifier::Lmb
        };
    }
    if GetAsyncKeyState(VK_RBUTTON as i32) != 0 {
        mods |= if swapped {
            Modifier::Lmb
        } else {
            Modifier::Rmb
        };
    }
    if GetAsyncKeyState(VK_MBUTTON as i32) != 0 {
        mods |= Modifier::Mmb;
    }
    return mods;
}

unsafe extern "system" fn wndproc(
    window: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    let sapp = GetWindowLongPtrW(window, GWLP_USERDATA) as *mut SApp;
    let sapp = match sapp.as_mut() {
        Some(app) => app,
        _ => return DefWindowProcW(window, message, wparam, lparam),
    };

    match message {
        WM_CLOSE => {
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
                _ => {}
            }
        }
        WM_ERASEBKGND => return 1,
        WM_SIZE => {
            let iconified = wparam == SIZE_MINIMIZED as usize;
            if iconified != sapp.base.win32.iconified {
                sapp.base.win32.iconified = iconified;
                if iconified {
                    sapp.call_event(&Event::Iconified);
                } else {
                    sapp.call_event(&Event::Restored);
                }
            }
        }

        WM_SETFOCUS => sapp.call_event(&Event::Focused),

        WM_KILLFOCUS => {
            // if focus is lost for any reason, and we're in mouse locked mode, disable mouse lock
            if sapp.base.mouse.locked {
                sapp_win32_lock_mouse(&mut sapp.base, false);
            }
            sapp.call_event(&Event::Unfocused);
        }
        WM_SETCURSOR => {
            if LOWORD(lparam as u32) == HTCLIENT as u16 {
                sapp_win32_update_cursor(
                    &sapp.base,
                    sapp.base.mouse.current_cursor,
                    sapp.base.mouse.shown,
                    true,
                );
                return TRUE as isize;
            }
        }
        WM_DPICHANGED => {
            // DT_TODO: Test this and look at https://learn.microsoft.com/en-us/windows/win32/hidpi/wm-dpichanged - why not access g_dpi = HIWORD(wParam); ??
            // Update window's DPI and size if its moved to another monitor with a different DPI
            // Only sent if DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2 is used.
            let rect = lparam as *const RECT;
            if let Some(rect) = rect.as_ref() {
                sapp_win32_dpi_changed(&mut sapp.base, window, rect);
            }
        }
        WM_LBUTTONDOWN => {
            sapp_win32_mouse_update(&mut sapp.base, lparam);
            sapp.call_event(&Event::Mouse(MouseEvent {
                pressed: true,
                mouse_button: MouseButton::Left,
            }));
            sapp_win32_capture_mouse(&mut sapp.base, 1u8 << MouseButton::Left as u8);
        }
        WM_RBUTTONDOWN => {
            sapp_win32_mouse_update(&mut sapp.base, lparam);
            sapp.call_event(&Event::Mouse(MouseEvent {
                pressed: true,
                mouse_button: MouseButton::Right,
            }));
            sapp_win32_capture_mouse(&mut sapp.base, 1u8 << MouseButton::Right as u8);
        }
        WM_MBUTTONDOWN => {
            sapp_win32_mouse_update(&mut sapp.base, lparam);
            sapp.call_event(&Event::Mouse(MouseEvent {
                pressed: true,
                mouse_button: MouseButton::Middle,
            }));
            sapp_win32_capture_mouse(&mut sapp.base, 1u8 << MouseButton::Middle as u8);
        }
        WM_LBUTTONUP => {
            sapp_win32_mouse_update(&mut sapp.base, lparam);
            sapp.call_event(&Event::Mouse(MouseEvent {
                pressed: false,
                mouse_button: MouseButton::Left,
            }));
            sapp_win32_release_mouse(&mut sapp.base, 1u8 << MouseButton::Left as u8);
        }
        WM_RBUTTONUP => {
            sapp_win32_mouse_update(&mut sapp.base, lparam);
            sapp.call_event(&Event::Mouse(MouseEvent {
                pressed: false,
                mouse_button: MouseButton::Right,
            }));
            sapp_win32_release_mouse(&mut sapp.base, 1u8 << MouseButton::Right as u8);
        }
        WM_MBUTTONUP => {
            sapp_win32_mouse_update(&mut sapp.base, lparam);
            sapp.call_event(&Event::Mouse(MouseEvent {
                pressed: false,
                mouse_button: MouseButton::Middle,
            }));
            sapp_win32_release_mouse(&mut sapp.base, 1u8 << MouseButton::Middle as u8);
        }
        WM_MOUSEMOVE => {
            if !sapp.base.mouse.locked {
                sapp_win32_mouse_update(&mut sapp.base, lparam);
                if !sapp.base.win32.mouse_tracked {
                    sapp.base.win32.mouse_tracked = true;
                    let mut tme = TRACKMOUSEEVENT {
                        cbSize: std::mem::size_of::<TRACKMOUSEEVENT>() as u32,
                        dwFlags: TME_LEAVE,
                        hwndTrack: sapp.base.win32.hwnd,
                        dwHoverTime: 0,
                    };
                    TrackMouseEvent(&mut tme);
                    sapp.call_event(&Event::MouseEnter);
                }
                sapp.call_event(&Event::MouseMove(MouseMoveEvent {
                    mouse_dx: sapp.base.mouse.dx,
                    mouse_dy: sapp.base.mouse.dy,
                }));
            }
        }

        WM_INPUT => {
            // raw mouse input during mouse-lock
            if sapp.base.mouse.locked {
                let ri = lparam as HRAWINPUT;
                let mut size: u32 = sapp.base.win32.raw_input_data.len() as u32;
                let ptr = &mut sapp.base.win32.raw_input_data as *mut u8 as *mut core::ffi::c_void;
                // see: https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getrawinputdata
                if -1i32 as u32
                    != GetRawInputData(
                        ri,
                        RID_INPUT,
                        ptr,
                        &mut size,
                        std::mem::size_of::<RAWINPUTHEADER>() as u32,
                    )
                {
                    let raw_mouse_data = ptr as *const RAWINPUT; // DT_TODO: Check casting of this - copy into a RAWINPUT Struct for safety? (or have array of RAWINPUT's up to a size?)
                    let raw_data = &(*raw_mouse_data).data;
                    if (raw_data.mouse.usFlags as u32 & MOUSE_MOVE_ABSOLUTE) != 0 {
                        // mouse only reports absolute position
                        // NOTE: This code is untested and will most likely behave wrong in Remote Desktop sessions.
                        // (such remote desktop sessions are setting the MOUSE_MOVE_ABSOLUTE flag).
                        // See: https://github.com/floooh/sokol/issues/806 and
                        // https://github.com/microsoft/DirectXTK/commit/ef56b63f3739381e451f7a5a5bd2c9779d2a7555)
                        let new_x = raw_data.mouse.lLastX;
                        let new_y = raw_data.mouse.lLastY;
                        if sapp.base.win32.raw_input_mousepos_valid {
                            sapp.base.mouse.dx =
                                (new_x - sapp.base.win32.raw_input_mousepos_x) as f32;
                            sapp.base.mouse.dy =
                                (new_y - sapp.base.win32.raw_input_mousepos_y) as f32;
                        }
                        sapp.base.win32.raw_input_mousepos_x = new_x;
                        sapp.base.win32.raw_input_mousepos_y = new_y;
                        sapp.base.win32.raw_input_mousepos_valid = true;
                    } else {
                        // mouse reports movement delta (this seems to be the common case)
                        sapp.base.mouse.dx = raw_data.mouse.lLastX as f32;
                        sapp.base.mouse.dy = raw_data.mouse.lLastY as f32;
                    }
                    sapp.call_event(&Event::MouseMove(MouseMoveEvent {
                        mouse_dx: sapp.base.mouse.dx,
                        mouse_dy: sapp.base.mouse.dy,
                    }));
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
            sapp_win32_mouse_update(&mut sapp.base, lparam);
            sapp.call_event(&Event::MouseScroll(MouseScrollEvent {
                scroll_x: 0.0,
                scroll_y: HIWORD(wparam as u32) as f32 / 30.0,
            }));
        }
        WM_MOUSEHWHEEL => {
            sapp_win32_mouse_update(&mut sapp.base, lparam);
            sapp.call_event(&Event::MouseScroll(MouseScrollEvent {
                scroll_x: HIWORD(wparam as u32) as f32 / -30.0,
                scroll_y: 0.0,
            }));
        }
        WM_CHAR => {
            let wparam_u32 = wparam as u32;
            let is_high_surrogate =
                (HIGH_SURROGATE_START..=HIGH_SURROGATE_END).contains(&wparam_u32);
            let is_low_surrogate = (LOW_SURROGATE_START..=LOW_SURROGATE_END).contains(&wparam_u32);

            let set_char = if is_high_surrogate {
                sapp.base.win32.high_surrogate = wparam_u32 as u16;
                None
            } else if is_low_surrogate {
                let high_surrogate = sapp.base.win32.high_surrogate;
                if let Some(Ok(chr)) = char::decode_utf16([high_surrogate, wparam as u16]).next() {
                    Some(chr)
                } else {
                    None
                }
            } else {
                char::from_u32(wparam_u32)
            };

            let key_repeat = lparam & 0x40000000 != 0;
            if let Some(char_code) = set_char {
                // DT_TODO: Why? perhaps handled in key events?
                if char_code as u32 >= 32 {
                    sapp.call_event(&Event::Char(CharEvent {
                        char_code,
                        key_repeat,
                    }));
                }
            }
        }
        WM_KEYDOWN | WM_SYSKEYDOWN => {
            let key = (HIWORD(lparam as u32) & 0x1FF) as usize; // DT_TODO: Accessing lparam for scan codes??
            if key < sapp.base.keycodes.len() {
                let key_code = sapp.base.keycodes[key];
                sapp.call_event(&Event::Key(KeyEvent {
                    pressed: true,
                    key_code,
                    key_repeat: lparam & 0x40000000 != 0,
                }));

                // check if a CLIPBOARD_PASTED event must be sent too
                if sapp.base.clipboard.enabled
                    && (key_code == KeyCode::V)
                    && (sapp_win32_mods() == Modifier::Ctrl)
                {
                    sapp.call_event(&Event::ClipboardPasted);
                }
            }
        }
        WM_KEYUP | WM_SYSKEYUP => {
            let key = (HIWORD(lparam as u32) & 0x1FF) as usize;
            if key < sapp.base.keycodes.len() {
                sapp.call_event(&Event::Key(KeyEvent {
                    pressed: false,
                    key_code: sapp.base.keycodes[key],
                    key_repeat: false,
                }));
            }
        }
        WM_ENTERSIZEMOVE => {
            SetTimer(sapp.base.win32.hwnd, 1, USER_TIMER_MINIMUM, None);
        }
        WM_EXITSIZEMOVE => {
            KillTimer(sapp.base.win32.hwnd, 1);
        }

        WM_TIMER => {
            sapp.call_frame();
            sapp_wgl_swap_buffers(&mut sapp.base);

            //NOTE: resizing the swap-chain during resize leads to a substantial
            //   memory spike (hundreds of megabytes for a few seconds).
        }
        WM_NCLBUTTONDOWN => {
            // workaround for half-second pause when starting to move window
            //    see: https://gamedev.net/forums/topic/672094-keeping-things-moving-during-win32-moveresize-events/5254386/
            if SendMessageW(sapp.base.win32.hwnd, WM_NCHITTEST, wparam, lparam)
                == HTCAPTION as isize
            {
                let mut point = POINT { x: 0, y: 0 };
                GetCursorPos(&mut point);
                ScreenToClient(sapp.base.win32.hwnd, &mut point);
                PostMessageW(
                    sapp.base.win32.hwnd,
                    WM_MOUSEMOVE,
                    0,
                    ((point.x as u32) | ((point.y as u32) << 16)) as isize,
                );
            }
        }
        WM_DROPFILES => sapp_win32_files_dropped(sapp, wparam as HDROP),
        _ => {}
    }

    return DefWindowProcW(window, message, wparam, lparam);
}

unsafe fn win32_process_loop(sapp: *mut SApp) {
    // Very unsure of Rust rules of lifetimes when having a pointer to that data stored with the window
    // To be sure, leaving as a pointer always except in the message loop when it is made into a reference temporarily
    SetWindowLongPtrW((*sapp).base.win32.hwnd, GWL_USERDATA, sapp as isize);

    let mut done = false;
    while !done && !(*sapp).base.quit_ordered {
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
        (*sapp).frame();
        sapp_wgl_swap_buffers(&mut (*sapp).base);

        // check for window resized, this cannot happen in WM_SIZE as it explodes memory usage
        if sapp_win32_update_dimensions(&mut (*sapp).base) {
            (*sapp).call_event(&Event::Resized)
        }

        if (*sapp).base.quit_requested {
            PostMessageW((*sapp).base.win32.hwnd, WM_CLOSE, 0, 0);
        }
    }

    // Unset pointer on the window to ensure no more processing with the data
    SetWindowLongPtrW((*sapp).base.win32.hwnd, GWL_USERDATA, 0);
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
        fn_get_dpi_for_monitor =
            std::mem::transmute(GetProcAddress(shcore, s!("GetDpiForMonitor")));
    }

    // NOTE on SetProcessDpiAware() vs SetProcessDpiAwareness() vs SetProcessDpiAwarenessContext():
    //
    // These are different attempts to get DPI handling on Windows right, from oldest
    // to newest. SetProcessDpiAwarenessContext() is required for the new
    // DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2 method.
    if let Some(set_process_dpi_awareness) = fn_set_process_dpi_awareness {
        if sapp.high_dpi {
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

    if sapp.high_dpi {
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

unsafe fn sapp_win32_set_fullscreen(sapp: &mut SAppData, fullscreen: bool, swp_flags: u32) {
    let monitor = MonitorFromWindow(sapp.win32.hwnd, MONITOR_DEFAULTTONEAREST);
    let mut minfo = MONITORINFO {
        cbSize: std::mem::size_of::<MONITORINFO>() as u32,
        rcMonitor: RECT {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        },
        rcWork: RECT {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        },
        dwFlags: 0,
    };
    GetMonitorInfoW(monitor, &mut minfo);
    let mr = minfo.rcMonitor;
    let monitor_w = mr.right - mr.left;
    let monitor_h = mr.bottom - mr.top;

    let win_ex_style = WS_EX_APPWINDOW | WS_EX_WINDOWEDGE;
    let mut win_style = 0;
    let mut rect = RECT {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };

    sapp.fullscreen = fullscreen;
    if !sapp.fullscreen {
        win_style = WS_CLIPSIBLINGS
            | WS_CLIPCHILDREN
            | WS_CAPTION
            | WS_SYSMENU
            | WS_MINIMIZEBOX
            | WS_MAXIMIZEBOX
            | WS_SIZEBOX;
        rect = sapp.win32.stored_window_rect;
    } else {
        GetWindowRect(sapp.win32.hwnd, &mut sapp.win32.stored_window_rect);
        win_style = WS_POPUP | WS_SYSMENU | WS_VISIBLE;
        rect.left = mr.left;
        rect.top = mr.top;
        rect.right = rect.left + monitor_w;
        rect.bottom = rect.top + monitor_h;
        AdjustWindowRectEx(&mut rect, win_style, FALSE, win_ex_style);
    }
    let win_w = rect.right - rect.left;
    let win_h = rect.bottom - rect.top;
    let win_x = rect.left;
    let win_y = rect.top;
    SetWindowLongPtrW(sapp.win32.hwnd, GWL_STYLE, win_style as isize);
    SetWindowPos(
        sapp.win32.hwnd,
        HWND_TOP,
        win_x,
        win_y,
        win_w,
        win_h,
        swp_flags | SWP_FRAMECHANGED,
    );
}

fn sapp_win32_toggle_fullscreen(sapp: &mut SAppData) {
    unsafe { sapp_win32_set_fullscreen(sapp, !sapp.fullscreen, SWP_SHOWWINDOW) };
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

unsafe fn sapp_win32_create_window(desc: &SAppDesc, sapp: &mut SAppData) {

    let timer = Timer::new();
    let mut last_time = 1;

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

    println!("Register class {} ms", Timer::ms(timer.laptime(&mut last_time)));

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

    println!("Adjust window Rect {} ms", Timer::ms(timer.laptime(&mut last_time)));

    let win_width = rect.right - rect.left;
    let win_height = rect.bottom - rect.top;
    sapp.win32.in_create_window = true;

    // DT_TODO: See about setting active code page in the manifest to utf8 to not have to do this
    // UTF16 null terminated string
    let mut title = Vec::with_capacity(desc.window_title.encode_utf16().count() + 1);
    title.extend(desc.window_title.encode_utf16());
    title.push(0);

    println!("UTF title {} ms", Timer::ms(timer.laptime(&mut last_time)));

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

    println!("Create window {} ms", Timer::ms(timer.laptime(&mut last_time)));

    sapp.win32.in_create_window = false;
    sapp.win32.dc = GetDC(sapp.win32.hwnd);
    sapp.win32.hmonitor = MonitorFromWindow(sapp.win32.hwnd, MONITOR_DEFAULTTONULL);
    debug_assert!(sapp.win32.dc != 0);

    println!("MonitorFromWindow {} ms", Timer::ms(timer.laptime(&mut last_time)));

    /* this will get the actual windowed-mode window size, if fullscreen
       is requested, the set_fullscreen function will then capture the
       current window rectangle, which then might be used later to
       restore the window position when switching back to windowed
    */
    sapp_win32_update_dimensions(sapp);

    println!("sapp_win32_update_dimensions {} ms", Timer::ms(timer.laptime(&mut last_time)));

    if sapp.fullscreen {
        sapp_win32_set_fullscreen(sapp, sapp.fullscreen, SWP_HIDEWINDOW);
        sapp_win32_update_dimensions(sapp);
    }
    ShowWindow(sapp.win32.hwnd, SW_SHOW);

    println!("ShowWindow {} ms", Timer::ms(timer.laptime(&mut last_time)));

    if sapp.drop.max_files > 0 {
        DragAcceptFiles(sapp.win32.hwnd, 1);
    }
}

fn sapp_setup_default_icon<'a>(icon_buffer: &'a mut Vec<u32>) -> SappIconDesc<'a> {
    let icon_sizes = [16, 32, 64]; // must be multiple of 8!

    // allocate a pixel buffer for all icon pixels
    let mut all_num_pixels = 0;
    for size in icon_sizes {
        all_num_pixels += size * size;
    }

    icon_buffer.resize(all_num_pixels, 0);
    let (first, back) = icon_buffer.split_at_mut(icon_sizes[0] * icon_sizes[0]);
    let (second, third) = back.split_at_mut(icon_sizes[1] * icon_sizes[1]);
    let mut buf_pixels = [first, second, third];

    // Amstrad CPC font 'S'
    let tile: [u8; 8] = [0x3C, 0x66, 0x60, 0x3C, 0x06, 0x66, 0x3C, 0x00];
    // rainbow colors
    let colors: [u32; 8] = [
        0xFF4370FF, 0xFF26A7FF, 0xFF58EEFF, 0xFF57E1D4, 0xFF65CC9C, 0xFF6ABB66, 0xFFF5A542,
        0xFFC2577E,
    ];

    let blank: u32 = 0x00FFFFFF;
    let shadow: u32 = 0xFF000000;

    for i in 0..icon_sizes.len() {
        let dim = icon_sizes[i];
        let dst = &mut buf_pixels[i];
        let mut index = 0;
        debug_assert!((dim % 8) == 0);
        let scale = dim / 8;
        for ty in 0..8 {
            let color = colors[ty];
            for _sy in 0..scale {
                let mut bits = tile[ty];
                for _tx in 0..8 {
                    let pixel = if 0 == (bits & 0x80) { blank } else { color };
                    for _sx in 0..scale {
                        dst[index] = pixel;
                        index += 1;
                    }
                    bits <<= 1;
                }
            }
        }
    }

    // right shadow
    for i in 0..icon_sizes.len() {
        let dim = icon_sizes[i];
        let dst = &mut buf_pixels[i];
        for y in 0..dim {
            let mut prev_color = blank;
            for x in 0..dim {
                let dst_index = y * dim + x;
                let cur_color = dst[dst_index];
                if (cur_color == blank) && (prev_color != blank) {
                    dst[dst_index] = shadow;
                }
                prev_color = cur_color;
            }
        }
    }

    // bottom shadow
    for i in 0..icon_sizes.len() {
        let dim = icon_sizes[i];
        let dst = &mut buf_pixels[i];
        for x in 0..dim {
            let mut prev_color = blank;
            for y in 0..dim {
                let dst_index = y * dim + x;
                let cur_color = dst[dst_index];
                if (cur_color == blank) && (prev_color != blank) {
                    dst[dst_index] = shadow;
                }
                prev_color = cur_color;
            }
        }
    }

    // initialize default_icon_desc struct
    let mut desc = SappIconDesc::new();
    for i in 0..icon_sizes.len() {
        let dim = icon_sizes[i];

        let img_desc = &mut desc.images[i];
        img_desc.width = dim as u32;
        img_desc.height = dim as u32;
        img_desc.pixels = buf_pixels[i];
    }
    desc
}

unsafe fn sapp_win32_create_icon_from_image(desc: &SappImageDesc) -> HICON {
    let bi = BITMAPV5HEADER {
        bV5Size: std::mem::size_of::<BITMAPV5HEADER>() as u32,
        bV5Width: desc.width as i32,
        bV5Height: -(desc.height as i32), // NOTE the '-' here to indicate that origin is top-left
        bV5Planes: 1,
        bV5BitCount: 32,
        bV5Compression: BI_BITFIELDS,
        bV5RedMask: 0x00FF0000,
        bV5GreenMask: 0x0000FF00,
        bV5BlueMask: 0x000000FF,
        bV5AlphaMask: 0xFF000000,

        bV5SizeImage: 0,
        bV5XPelsPerMeter: 0,
        bV5YPelsPerMeter: 0,
        bV5ClrUsed: 0,
        bV5ClrImportant: 0,
        bV5CSType: 0,
        bV5Endpoints: CIEXYZTRIPLE {
            ciexyzRed: CIEXYZ {
                ciexyzX: 0,
                ciexyzY: 0,
                ciexyzZ: 0,
            },
            ciexyzGreen: CIEXYZ {
                ciexyzX: 0,
                ciexyzY: 0,
                ciexyzZ: 0,
            },
            ciexyzBlue: CIEXYZ {
                ciexyzX: 0,
                ciexyzY: 0,
                ciexyzZ: 0,
            },
        },
        bV5GammaRed: 0,
        bV5GammaGreen: 0,
        bV5GammaBlue: 0,
        bV5Intent: 0,
        bV5ProfileData: 0,
        bV5ProfileSize: 0,
        bV5Reserved: 0,
    };

    let mut target_void: *mut ::core::ffi::c_void = std::ptr::null_mut();
    let dc = GetDC(0);
    let color = CreateDIBSection(
        dc,
        &bi as *const BITMAPV5HEADER as *const BITMAPINFO,
        DIB_RGB_COLORS,
        &mut target_void,
        0,
        0,
    );
    ReleaseDC(0, dc);
    if 0 == color {
        return 0;
    }
    debug_assert!(target_void != std::ptr::null_mut());

    let mask = CreateBitmap(
        desc.width as i32,
        desc.height as i32,
        1,
        1,
        std::ptr::null(),
    );
    if 0 == mask {
        DeleteObject(color);
        return 0;
    }

    // DT_TODO: Check if a better way of doing this
    let mut target: *mut u8 = target_void as *mut u8;
    let mut source = desc.pixels.as_ptr() as *mut u8;
    for _ in 0..(desc.width * desc.height) {
        *target.add(0) = *source.add(2);
        *target.add(1) = *source.add(1);
        *target.add(2) = *source.add(0);
        *target.add(3) = *source.add(3);

        target = target.add(4);
        source = source.add(4);
    }

    let icon_info = ICONINFO {
        fIcon: TRUE,
        xHotspot: 0,
        yHotspot: 0,
        hbmMask: mask,
        hbmColor: color,
    };
    let icon_handle = CreateIconIndirect(&icon_info);
    DeleteObject(color);
    DeleteObject(mask);

    return icon_handle;
}

fn sapp_image_validate(desc: &SappImageDesc) -> bool {
    debug_assert!(desc.width > 0);
    debug_assert!(desc.height > 0);
    debug_assert!(desc.pixels.len() != 0);
    let wh_size = (desc.width * desc.height) as usize;
    if wh_size != desc.pixels.len() {
        //_SAPP_ERROR(IMAGE_DATA_SIZE_MISMATCH);
        return false;
    }
    return true;
}

fn sapp_image_bestmatch(image_descs: &[SappImageDesc], width: i32, height: i32) -> i32 {
    let mut least_diff: i32 = 0x7FFFFFFF;
    let mut least_index: i32 = 0;
    for i in 0..image_descs.len() {
        let mut diff: i32 =
            (image_descs[i].width * image_descs[i].height) as i32 - (width * height);
        if diff < 0 {
            diff = -diff;
        }
        if diff < least_diff {
            least_diff = diff;
            least_index = i as i32;
        }
    }
    return least_index;
}

fn sapp_icon_num_images(desc: &SappIconDesc) -> u32 {
    for index in 0..SAPP_MAX_ICONIMAGES {
        if 0 == desc.images[index as usize].pixels.len() {
            return index;
        }
    }
    SAPP_MAX_ICONIMAGES
}

fn sapp_validate_icon_desc(desc: &SappIconDesc, num_images: u32) -> bool {
    debug_assert!(num_images <= SAPP_MAX_ICONIMAGES);
    for i in 0..num_images {
        if !sapp_image_validate(&desc.images[i as usize]) {
            return false;
        }
    }
    return true;
}

unsafe fn sapp_win32_set_icon(sapp: &mut SAppData, icon_desc: &SappIconDesc, num_images: u32) {
    debug_assert!((num_images > 0) && (num_images <= SAPP_MAX_ICONIMAGES));

    let big_img_index = sapp_image_bestmatch(
        &icon_desc.images[0..num_images as usize],
        GetSystemMetrics(SM_CXICON),
        GetSystemMetrics(SM_CYICON),
    );
    let sml_img_index = sapp_image_bestmatch(
        &icon_desc.images[0..num_images as usize],
        GetSystemMetrics(SM_CXSMICON),
        GetSystemMetrics(SM_CYSMICON),
    );
    let big_icon = sapp_win32_create_icon_from_image(&icon_desc.images[big_img_index as usize]);
    let sml_icon = sapp_win32_create_icon_from_image(&icon_desc.images[sml_img_index as usize]);

    // if icon creation or lookup has failed for some reason, leave the currently set icon untouched
    if 0 != big_icon {
        SendMessageW(sapp.win32.hwnd, WM_SETICON, ICON_BIG as usize, big_icon);
        if 0 != sapp.win32.big_icon {
            DestroyIcon(sapp.win32.big_icon);
        }
        sapp.win32.big_icon = big_icon;
    }
    if 0 != sml_icon {
        SendMessageW(sapp.win32.hwnd, WM_SETICON, ICON_SMALL as usize, sml_icon);
        if 0 != sapp.win32.small_icon {
            DestroyIcon(sapp.win32.small_icon);
        }
        sapp.win32.small_icon = sml_icon;
    }
}

unsafe fn sapp_win32_destroy_window(sapp: &mut SAppData) {
    DestroyWindow(sapp.win32.hwnd);
    sapp.win32.hwnd = 0;
    UnregisterClassW(w!("SOKOLAPP"), GetModuleHandleW(std::ptr::null()));
}

unsafe fn sapp_win32_destroy_icons(sapp: &mut SAppData) {
    if sapp.win32.big_icon != 0 {
        DestroyIcon(sapp.win32.big_icon);
        sapp.win32.big_icon = 0;
    }
    if sapp.win32.small_icon != 0 {
        DestroyIcon(sapp.win32.small_icon);
        sapp.win32.small_icon = 0;
    }
}

unsafe fn sapp_win32_init_console(sapp: &mut SAppData) {
    if sapp.win32.console_create || sapp.win32.console_attach {
        let mut con_valid = FALSE;
        if sapp.win32.console_create {
            con_valid = AllocConsole();
        } else if sapp.win32.console_attach {
            con_valid = AttachConsole(ATTACH_PARENT_PROCESS);
        }

        // Not needed as if the above succeeds, the console output is already setup?
        //if con_valid != FALSE {
        //let file_handle = CreateFileA(s!("CONOUT$"), GENERIC_WRITE, FILE_SHARE_WRITE, std::ptr::null(), OPEN_EXISTING, FILE_ATTRIBUTE_NORMAL, 0);
        //SetStdHandle(STD_OUTPUT_HANDLE, file_handle);
        //SetStdHandle(STD_ERROR_HANDLE , file_handle);
        //}
    }
    if sapp.win32.console_utf8 {
        sapp.win32.orig_codepage = GetConsoleOutputCP();
        SetConsoleOutputCP(CP_UTF8);
    }
}

unsafe fn sapp_win32_restore_console(sapp: &mut SAppData) {
    if sapp.win32.console_utf8 {
        SetConsoleOutputCP(sapp.win32.orig_codepage);
    }
    // Not closing any created console as it may still be used
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
    cursors: [HCURSOR; SAPP_MOUSECURSOR_NUM as usize],
    orig_codepage: u32,
    mouse_locked_x: i32,
    mouse_locked_y: i32,
    stored_window_rect: RECT, // used to restore window pos/size when toggling fullscreen => windowed
    in_create_window: bool,
    iconified: bool,
    mouse_tracked: bool,
    mouse_capture_mask: u8,
    dpi: DPI,
    high_surrogate: u16, // Used to create the other side of a large unicode character

    raw_input_mousepos_valid: bool,
    raw_input_mousepos_x: i32,
    raw_input_mousepos_y: i32,
    raw_input_data: [u8; 256],

    console_utf8: bool,   // if true, set the output console codepage to UTF-8
    console_create: bool, // if true, attach stdout/stderr to a new console window
    console_attach: bool, // if true, attach stdout/stderr to parent process
}
impl SAppWin32 {
    fn new(desc: &SAppDesc) -> SAppWin32 {
        SAppWin32 {
            hwnd: 0,
            hmonitor: 0,
            dc: 0,
            big_icon: 0,
            small_icon: 0,
            cursors: [0; SAPP_MOUSECURSOR_NUM as usize],
            orig_codepage: 0,
            mouse_locked_x: 0,
            mouse_locked_y: 0,
            stored_window_rect: RECT {
                left: 0,
                top: 0,
                right: 0,
                bottom: 0,
            }, // used to restore window pos/size when toggling fullscreen => windowed
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
            high_surrogate: 0,
            raw_input_mousepos_valid: false,
            raw_input_mousepos_x: 0,
            raw_input_mousepos_y: 0,
            raw_input_data: [0; 256],

            console_utf8: desc.win32_console_utf8,
            console_create: desc.win32_console_create,
            console_attach: desc.win32_console_attach,
        }
    }
}

struct GLFBConfig {
    red_bits: i32,
    green_bits: i32,
    blue_bits: i32,
    alpha_bits: i32,
    depth_bits: i32,
    stencil_bits: i32,
    samples: i32,
    handle: usize,
}

struct GLSelect {
    first : bool,
    least_missing : i32,
    least_color_diff : i32,
    least_extra_diff : i32,
    handle: usize,
}
impl GLSelect {
    fn new() -> GLSelect{
        GLSelect {
            first: true,
            least_missing: 0,
            least_color_diff: 0,
            least_extra_diff: 0,
            handle: 0,
        }
    }

    fn process(&mut self, desired: &GLFBConfig, current: &GLFBConfig) -> bool {
        let mut missing = 0;
        if desired.alpha_bits > 0 && current.alpha_bits == 0 {
            missing += 1;
        }
        if desired.depth_bits > 0 && current.depth_bits == 0 {
            missing += 1;
        }
        if desired.stencil_bits > 0 && current.stencil_bits == 0 {
            missing += 1;
        }
        if desired.samples > 0 && current.samples == 0 {
            // Technically, several multisampling buffers could be
            // involved, but that's a lower level implementation detail and
            // not important to us here, so we count them as one
            missing += 1;
        }

        // These polynomials make many small channel size differences matter
        // less than one large channel size difference
        // Calculate color channel size difference value
        let mut color_diff = 0;
        if desired.red_bits != -1 {
            color_diff +=
                (desired.red_bits - current.red_bits) * (desired.red_bits - current.red_bits);
        }
        if desired.green_bits != -1 {
            color_diff += (desired.green_bits - current.green_bits)
                * (desired.green_bits - current.green_bits);
        }
        if desired.blue_bits != -1 {
            color_diff +=
                (desired.blue_bits - current.blue_bits) * (desired.blue_bits - current.blue_bits);
        }

        // Calculate non-color channel size difference value
        let mut extra_diff = 0;
        if desired.alpha_bits != -1 {
            extra_diff += (desired.alpha_bits - current.alpha_bits)
                * (desired.alpha_bits - current.alpha_bits);
        }
        if desired.depth_bits != -1 {
            extra_diff += (desired.depth_bits - current.depth_bits)
                * (desired.depth_bits - current.depth_bits);
        }
        if desired.stencil_bits != -1 {
            extra_diff += (desired.stencil_bits - current.stencil_bits)
                * (desired.stencil_bits - current.stencil_bits);
        }
        if desired.samples != -1 {
            extra_diff += (desired.samples - current.samples) * (desired.samples - current.samples);
        }

        // Figure out if the current one is better than the best one found so far
        // Least number of missing buffers is the most important heuristic,
        // then color buffer size match and lastly size match for other buffers
        let mut update = false;
        if self.first {
            self.first = false;
            update = true;
        }
        else if missing < self.least_missing {
            update = true;
        } else if missing == self.least_missing {
            if (color_diff < self.least_color_diff)
                || (color_diff == self.least_color_diff && extra_diff < self.least_extra_diff)
            {
                update = true;
            }
        }
        if update {
            self.least_missing = missing;
            self.least_color_diff = color_diff;
            self.least_extra_diff = extra_diff;
            self.handle = current.handle;
        }

        // Check for perfect match
        if (missing | color_diff | extra_diff) == 0 {
            return true;
        }
        false

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

// DT_TODO: Do the function signatures need unsafe on them?
type FnWGLCreateContextT = extern "system" fn(HDC) -> HGLRC;
type FnWGLDeleteContextT = extern "system" fn(HGLRC) -> BOOL;
type FnWGLGetProcAddressT = extern "system" fn(PCSTR) -> FARPROC;
type FnWGLGetCurrentDCT = extern "system" fn() -> HDC;
type FnWGLMakeCurrentT = extern "system" fn(HDC, HGLRC) -> BOOL;

type FnWGLSwapIntervalEXTT = extern "system" fn(u32) -> BOOL;
type FnWGLGetPixelFormatAttribivARBT =
    extern "system" fn(HDC, i32, i32, u32, *const i32, *mut i32) -> BOOL;

type FnWGLGetExtensionsStringEXTT = extern "system" fn() -> PCSTR;
type FnWGLGetExtensionsStringARBT = extern "system" fn(HDC) -> PCSTR;
type FnWGLCreateContextAttribsARBT = extern "system" fn(HDC, HGLRC, *const i32) -> HGLRC;

struct SAppWgl {
    gl_major_version: u32,
    gl_minor_version: u32,

    opengl32: HINSTANCE,
    gl_ctx: HGLRC,
    CreateContext: Option<FnWGLCreateContextT>,
    DeleteContext: Option<FnWGLDeleteContextT>,
    GetProcAddress: Option<FnWGLGetProcAddressT>,
    GetCurrentDC: Option<FnWGLGetCurrentDCT>,
    MakeCurrent: Option<FnWGLMakeCurrentT>,
    SwapIntervalEXT: Option<FnWGLSwapIntervalEXTT>,
    GetPixelFormatAttribivARB: Option<FnWGLGetPixelFormatAttribivARBT>,
    GetExtensionsStringEXT: Option<FnWGLGetExtensionsStringEXTT>,
    GetExtensionsStringARB: Option<FnWGLGetExtensionsStringARBT>,
    CreateContextAttribsARB: Option<FnWGLCreateContextAttribsARBT>,
    ext_swap_control: bool,
    arb_multisample: bool,
    arb_pixel_format: bool,
    arb_create_context: bool,
    arb_create_context_profile: bool,
    msg_hwnd: HWND,
    msg_dc: HDC,
}
impl SAppWgl {
    fn new(desc: &SAppDesc) -> SAppWgl {
        SAppWgl {
            gl_major_version: desc.gl_major_version,
            gl_minor_version: desc.gl_minor_version,
            opengl32: 0,
            gl_ctx: 0,
            CreateContext: None,
            DeleteContext: None,
            GetProcAddress: None,
            GetCurrentDC: None,
            MakeCurrent: None,
            SwapIntervalEXT: None,
            GetPixelFormatAttribivARB: None,
            GetExtensionsStringEXT: None,
            GetExtensionsStringARB: None,
            CreateContextAttribsARB: None,
            ext_swap_control: false,
            arb_multisample: false,
            arb_pixel_format: false,
            arb_create_context: false,
            arb_create_context_profile: false,
            msg_hwnd: 0,
            msg_dc: 0,
        }
    }
}

unsafe fn sapp_wgl_init(sapp: &mut SAppData) {
    sapp.wgl.opengl32 = LoadLibraryA(s!("opengl32.dll"));
    if sapp.wgl.opengl32 == 0 {
        panic!(); // DT_TODO:
                  //_SAPP_PANIC(WIN32_LOAD_OPENGL32_DLL_FAILED);
    }
    debug_assert!(sapp.wgl.opengl32 != 0);
    sapp.wgl.CreateContext =
        std::mem::transmute(GetProcAddress(sapp.wgl.opengl32, s!("wglCreateContext")));
    debug_assert!(!sapp.wgl.CreateContext.is_none());
    sapp.wgl.DeleteContext =
        std::mem::transmute(GetProcAddress(sapp.wgl.opengl32, s!("wglDeleteContext")));
    debug_assert!(!sapp.wgl.DeleteContext.is_none());
    sapp.wgl.GetProcAddress =
        std::mem::transmute(GetProcAddress(sapp.wgl.opengl32, s!("wglGetProcAddress")));
    debug_assert!(!sapp.wgl.GetProcAddress.is_none());
    sapp.wgl.GetCurrentDC =
        std::mem::transmute(GetProcAddress(sapp.wgl.opengl32, s!("wglGetCurrentDC")));
    debug_assert!(!sapp.wgl.GetCurrentDC.is_none());
    sapp.wgl.MakeCurrent =
        std::mem::transmute(GetProcAddress(sapp.wgl.opengl32, s!("wglMakeCurrent")));
    debug_assert!(!sapp.wgl.MakeCurrent.is_none());

    sapp.wgl.msg_hwnd = CreateWindowExW(
        WS_EX_OVERLAPPEDWINDOW,
        w!("SOKOLAPP"),
        w!("sokol-app message window"),
        WS_CLIPSIBLINGS | WS_CLIPCHILDREN,
        0,
        0,
        1,
        1,
        0,
        0,
        GetModuleHandleW(std::ptr::null()),
        std::ptr::null(),
    );
    if sapp.wgl.msg_hwnd == 0 {
        panic!(); // DT_TODO:
                  //_SAPP_PANIC(WIN32_CREATE_HELPER_WINDOW_FAILED);
    }
    debug_assert!(sapp.wgl.msg_hwnd != 0);
    ShowWindow(sapp.wgl.msg_hwnd, SW_HIDE);
    let mut msg: MSG = std::mem::zeroed();
    while PeekMessageW(&mut msg, sapp.wgl.msg_hwnd, 0, 0, PM_REMOVE) == TRUE {
        TranslateMessage(&msg);
        DispatchMessageW(&msg);
    }
    sapp.wgl.msg_dc = GetDC(sapp.wgl.msg_hwnd);
    if sapp.wgl.msg_dc == 0 {
        panic!(); // DT_TODO:
                  //_SAPP_PANIC(WIN32_HELPER_WINDOW_GETDC_FAILED);
    }
}

unsafe fn sapp_wgl_shutdown(sapp: &mut SAppData) {
    debug_assert!(sapp.wgl.opengl32 != 0 && sapp.wgl.msg_hwnd != 0);
    DestroyWindow(sapp.wgl.msg_hwnd);
    sapp.wgl.msg_hwnd = 0;
    FreeLibrary(sapp.wgl.opengl32);
    sapp.wgl.opengl32 = 0;
}

fn sapp_wgl_has_ext(ext: &str, extensions: CStringIterator) -> bool {
    // Don't really need to check for zero length strings
    //if ext.len() == 0 {
    //    return true;
    //}

    let ext = ext.as_bytes();
    let mut valid = true;
    let mut iter = ext.iter();
    for c in extensions {
        // Reset search at spaces
        if c == ' ' as u8 {
            if valid && iter.next() == None {
                return true;
            }
            valid = true;
            iter = ext.iter();
        } else if valid {
            // If matching the string still
            valid = false;
            if let Some(v) = iter.next() {
                if c == *v {
                    valid = true;
                }
            }
        }
    }

    return valid && iter.next() == None;
}

/*
fn sapp_wgl_has_ext(ext: &str, extensions: &str) -> bool {

    let mut start = extensions;
    loop {
        if let Some(offset) = start.find(ext) {

            let terminator = &start[offset + ext.len()..];

            if (offset == 0) || start[offset - 1..offset].starts_with(' ') {
                if terminator.starts_with(' ') || terminator.is_empty() {
                    break;
                }
            }
            start = terminator;
        }
        else
        {
            return false;
        }
    }
    return true;
}
*/

fn sapp_wgl_ext_supported(sapp: &SAppData, ext: &str) -> bool {
    // DT_TODO: Does it really need to call both methods?
    // DT_TODO: Can we cache the strings?
    // DT_TODO: What is the relevance of calling the GetCurrentDC?
    if let Some(GetExtensionStr) = sapp.wgl.GetExtensionsStringEXT {
        let extensions = GetExtensionStr();
        if extensions != std::ptr::null() {
            if sapp_wgl_has_ext(ext, CStringIterator { string: extensions }) {
                return true;
            }
        }
    }
    if let Some(GetExtensionStr) = sapp.wgl.GetExtensionsStringARB {
        if let Some(GetCurrentDC) = sapp.wgl.GetCurrentDC {
            let extensions = GetExtensionStr(GetCurrentDC());
            if extensions != std::ptr::null() {
                if sapp_wgl_has_ext(ext, CStringIterator { string: extensions }) {
                    return true;
                }
            }
        }
    }
    return false;
}

unsafe fn sapp_wgl_load_extensions(sapp: &mut SAppData) {

    let timer = Timer::new();
    let mut last_time = 1;

    debug_assert!(sapp.wgl.msg_dc != 0);
    let pfd = PIXELFORMATDESCRIPTOR {
        nSize: std::mem::size_of::<PIXELFORMATDESCRIPTOR>() as u16,
        nVersion: 1,
        dwFlags: PFD_DRAW_TO_WINDOW | PFD_SUPPORT_OPENGL | PFD_DOUBLEBUFFER,
        iPixelType: PFD_TYPE_RGBA,
        cColorBits: 24,
        cRedBits: 0,
        cRedShift: 0,
        cGreenBits: 0,
        cGreenShift: 0,
        cBlueBits: 0,
        cBlueShift: 0,
        cAlphaBits: 0,
        cAlphaShift: 0,
        cAccumBits: 0,
        cAccumRedBits: 0,
        cAccumGreenBits: 0,
        cAccumBlueBits: 0,
        cAccumAlphaBits: 0,
        cDepthBits: 0,
        cStencilBits: 0,
        cAuxBuffers: 0,
        iLayerType: 0,
        bReserved: 0,
        dwLayerMask: 0,
        dwVisibleMask: 0,
        dwDamageMask: 0,
    };

    let pf = ChoosePixelFormat(sapp.wgl.msg_dc, &pfd);

    println!("Choose Pixel format {} ms", Timer::ms(timer.laptime(&mut last_time)));

    if SetPixelFormat(
        sapp.wgl.msg_dc,
        pf,
        &pfd,
    ) == FALSE
    {
        //_SAPP_PANIC(WIN32_DUMMY_CONTEXT_SET_PIXELFORMAT_FAILED);
        panic!();
    }

    println!("SetPixel formats {} ms", Timer::ms(timer.laptime(&mut last_time)));

    // DT_TODO: Remove all these unwraps
    let rc = sapp.wgl.CreateContext.unwrap()(sapp.wgl.msg_dc);
    if rc == 0 {
        //_SAPP_PANIC(WIN32_CREATE_DUMMY_CONTEXT_FAILED);
        panic!();
    }
    println!("Context create and set {} ms", Timer::ms(timer.laptime(&mut last_time)));

    if sapp.wgl.MakeCurrent.unwrap()(sapp.wgl.msg_dc, rc) == 0 {
        panic!();
        //_SAPP_PANIC(WIN32_DUMMY_CONTEXT_MAKE_CURRENT_FAILED);
    }

    println!("Make current {} ms", Timer::ms(timer.laptime(&mut last_time)));

    sapp.wgl.GetExtensionsStringEXT = std::mem::transmute(sapp.wgl.GetProcAddress.unwrap()(s!(
        "wglGetExtensionsStringEXT"
    )));
    sapp.wgl.GetExtensionsStringARB = std::mem::transmute(sapp.wgl.GetProcAddress.unwrap()(s!(
        "wglGetExtensionsStringARB"
    )));
    sapp.wgl.CreateContextAttribsARB = std::mem::transmute(sapp.wgl.GetProcAddress.unwrap()(s!(
        "wglCreateContextAttribsARB"
    )));
    sapp.wgl.SwapIntervalEXT =
        std::mem::transmute(sapp.wgl.GetProcAddress.unwrap()(s!("wglSwapIntervalEXT")));
    sapp.wgl.GetPixelFormatAttribivARB = std::mem::transmute(sapp.wgl.GetProcAddress.unwrap()(s!(
        "wglGetPixelFormatAttribivARB"
    )));

    println!("Get exts functions {} ms", Timer::ms(timer.laptime(&mut last_time)));

    sapp.wgl.arb_multisample = sapp_wgl_ext_supported(sapp, "WGL_ARB_multisample");
    sapp.wgl.arb_create_context = sapp_wgl_ext_supported(sapp, "WGL_ARB_create_context");
    sapp.wgl.arb_create_context_profile =
        sapp_wgl_ext_supported(sapp, "WGL_ARB_create_context_profile");
    sapp.wgl.ext_swap_control = sapp_wgl_ext_supported(sapp, "WGL_EXT_swap_control");
    sapp.wgl.arb_pixel_format = sapp_wgl_ext_supported(sapp, "WGL_ARB_pixel_format");

    println!("Test for ext {} ms", Timer::ms(timer.laptime(&mut last_time)));

    sapp.wgl.MakeCurrent.unwrap()(sapp.wgl.msg_dc, 0);
    sapp.wgl.DeleteContext.unwrap()(rc);

    println!("Context del {} ms", Timer::ms(timer.laptime(&mut last_time)));    
}

fn sapp_wgl_attrib(sapp: &mut SAppData, pixel_format: i32, attrib: i32) -> i32 {
    debug_assert!(sapp.wgl.arb_pixel_format);
    let mut value = 0;
    // DT_TODO: How is this callable without unsafe?
    if sapp.wgl.GetPixelFormatAttribivARB.unwrap()(
        sapp.win32.dc,
        pixel_format,
        0,
        1,
        &attrib,
        &mut value,
    ) == 0
    {
        panic!();
        //_SAPP_PANIC(WIN32_GET_PIXELFORMAT_ATTRIB_FAILED);
    }
    value
}

fn sapp_wgl_find_pixel_format(sapp: &mut SAppData) -> i32 {

    let timer = Timer::new();
    let mut last_time = 1;

    debug_assert!(sapp.win32.dc != 0);
    debug_assert!(sapp.wgl.arb_pixel_format);

    let desired = GLFBConfig {
        red_bits : 8,
        green_bits : 8,
        blue_bits : 8,
        alpha_bits : 8,
        depth_bits : 24,
        stencil_bits : 8,
        samples : if sapp.sample_count > 1 {
            sapp.sample_count as i32
        } else {
            0
        },
        handle: 0,
    };

    // DT_TODO: Update this to not need the array - call on the filter method progressively
    let native_count = sapp_wgl_attrib(sapp, 1, WGL_NUMBER_PIXEL_FORMATS_ARB as i32);

    const QUERY_TAGS : [i32; 12] = [
        WGL_SUPPORT_OPENGL_ARB as i32,
        WGL_DRAW_TO_WINDOW_ARB as i32,
        WGL_PIXEL_TYPE_ARB as i32,
        WGL_ACCELERATION_ARB as i32,
        WGL_DOUBLE_BUFFER_ARB as i32,
        WGL_RED_BITS_ARB as i32,
        WGL_GREEN_BITS_ARB as i32,
        WGL_BLUE_BITS_ARB as i32,
        WGL_ALPHA_BITS_ARB as i32,
        WGL_DEPTH_BITS_ARB as i32,
        WGL_STENCIL_BITS_ARB as i32,
        WGL_SAMPLES_ARB as i32,
    ];
    const RESULT_SUPPORT_OPENGL_INDEX: usize = 0;
    const RESULT_DRAW_TO_WINDOW_INDEX: usize = 1;
    const RESULT_PIXEL_TYPE_INDEX: usize = 2;
    const RESULT_ACCELERATION_INDEX: usize = 3;
    const RESULT_DOUBLE_BUFFER_INDEX: usize = 4;
    const RESULT_RED_BITS_INDEX: usize = 5;
    const RESULT_GREEN_BITS_INDEX: usize = 6;
    const RESULT_BLUE_BITS_INDEX: usize = 7;
    const RESULT_ALPHA_BITS_INDEX: usize = 8;
    const RESULT_DEPTH_BITS_INDEX: usize = 9;
    const RESULT_STENCIL_BITS_INDEX: usize = 10;
    const RESULT_SAMPLES_INDEX: usize = 11;

    let mut results:[i32; QUERY_TAGS.len()] = [0; QUERY_TAGS.len()];

    // Drop the last item if multisample extension is not supported.
    //  If in future querying with multiple extensions, will have to shuffle index values to have active extensions on the end.
    let max_index = if sapp.wgl.arb_multisample {
        QUERY_TAGS.len()
    } else {
        QUERY_TAGS.len() - 1
    };

    let mut processor = GLSelect::new();
    for i in 0..native_count {

        let pixel_format = i + 1;   // 1 based indices    
        if sapp.wgl.GetPixelFormatAttribivARB.unwrap()(
            sapp.win32.dc,
            pixel_format,
            0,
            max_index as u32,
            QUERY_TAGS.as_ptr(),
            results.as_mut_ptr(),
        ) == 0
        {
            panic!();
            //_SAPP_PANIC(WIN32_GET_PIXELFORMAT_ATTRIB_FAILED);
        }
        if results[RESULT_SUPPORT_OPENGL_INDEX] == 0 ||
           results[RESULT_DRAW_TO_WINDOW_INDEX] == 0 ||
           results[RESULT_PIXEL_TYPE_INDEX] != WGL_TYPE_RGBA_ARB as i32 ||
           results[RESULT_ACCELERATION_INDEX] == WGL_NO_ACCELERATION_ARB as i32 ||
           results[RESULT_DOUBLE_BUFFER_INDEX] == 0 { 
            continue;    
        }

        let current = GLFBConfig
        {
            red_bits: results[RESULT_RED_BITS_INDEX],
            green_bits: results[RESULT_GREEN_BITS_INDEX],
            blue_bits: results[RESULT_BLUE_BITS_INDEX],
            alpha_bits: results[RESULT_ALPHA_BITS_INDEX],

            depth_bits: results[RESULT_DEPTH_BITS_INDEX],
            stencil_bits: results[RESULT_STENCIL_BITS_INDEX],

            samples: results[RESULT_SAMPLES_INDEX], // Note: If arb_multisample is not supported  - just takes the default 0
            handle: pixel_format as usize,
        };

        // Stop querying if already found the best extension
        if processor.process(&desired, &current) {
            break;
        }
    }

    println!("Get Pixel formats {} ms", Timer::ms(timer.laptime(&mut last_time)));

    debug_assert!(processor.first == false);
    let pixel_format = processor.handle;

    println!("Find Pixel format {} ms", Timer::ms(timer.laptime(&mut last_time)));

    return pixel_format as i32;
}

unsafe fn sapp_wgl_create_context(sapp: &mut SAppData) {
    let pixel_format = sapp_wgl_find_pixel_format(sapp);
    if 0 == pixel_format {
        panic!();
        //_SAPP_PANIC(WIN32_WGL_FIND_PIXELFORMAT_FAILED);
    }
    let mut pfd: PIXELFORMATDESCRIPTOR = std::mem::zeroed(); // DT_TODO: SetPixelFormat does not really need this pixel format to be accurate - remove this and set values from find pixel format? (timing test)
    if DescribePixelFormat(
        sapp.win32.dc,
        pixel_format,
        std::mem::size_of::<PIXELFORMATDESCRIPTOR>() as u32,
        &mut pfd,
    ) == 0
    {
        panic!();
        //_SAPP_PANIC(WIN32_WGL_DESCRIBE_PIXELFORMAT_FAILED);
    }
    if SetPixelFormat(sapp.win32.dc, pixel_format, &pfd) == 0 {
        panic!();
        //_SAPP_PANIC(WIN32_WGL_SET_PIXELFORMAT_FAILED);
    }
    if !sapp.wgl.arb_create_context {
        panic!();
        //_SAPP_PANIC(WIN32_WGL_ARB_CREATE_CONTEXT_REQUIRED);
    }
    if !sapp.wgl.arb_create_context_profile {
        panic!();
        //_SAPP_PANIC(WIN32_WGL_ARB_CREATE_CONTEXT_PROFILE_REQUIRED);
    }
    let attrs = [
        WGL_CONTEXT_MAJOR_VERSION_ARB as i32,
        sapp.wgl.gl_major_version as i32,
        WGL_CONTEXT_MINOR_VERSION_ARB as i32,
        sapp.wgl.gl_minor_version as i32,
        WGL_CONTEXT_FLAGS_ARB as i32,
        WGL_CONTEXT_FORWARD_COMPATIBLE_BIT_ARB as i32,
        WGL_CONTEXT_PROFILE_MASK_ARB as i32,
        WGL_CONTEXT_CORE_PROFILE_BIT_ARB as i32,
        0,
        0,
    ];
    sapp.wgl.gl_ctx = sapp.wgl.CreateContextAttribsARB.unwrap()(sapp.win32.dc, 0, attrs.as_ptr());
    if sapp.wgl.gl_ctx == 0 {
        let err = GetLastError();
        if err == (0xc0070000 | ERROR_INVALID_VERSION_ARB) {
            panic!();
            //_SAPP_PANIC(WIN32_WGL_OPENGL_3_2_NOT_SUPPORTED);
        } else if err == (0xc0070000 | ERROR_INVALID_PROFILE_ARB) {
            panic!();
            //_SAPP_PANIC(WIN32_WGL_OPENGL_PROFILE_NOT_SUPPORTED);
        } else if err == (0xc0070000 | ERROR_INCOMPATIBLE_DEVICE_CONTEXTS_ARB) {
            panic!();
            //_SAPP_PANIC(WIN32_WGL_INCOMPATIBLE_DEVICE_CONTEXT);
        } else {
            panic!();
            //_SAPP_PANIC(WIN32_WGL_CREATE_CONTEXT_ATTRIBS_FAILED_OTHER);
        }
    }
    sapp.wgl.MakeCurrent.unwrap()(sapp.win32.dc, sapp.wgl.gl_ctx);
    if sapp.wgl.ext_swap_control {
        /* FIXME: DwmIsCompositionEnabled() (see GLFW) */
        sapp.wgl.SwapIntervalEXT.unwrap()(sapp.swap_interval);
    }
}

fn sapp_wgl_destroy_context(sapp: &mut SAppData) {
    debug_assert!(sapp.wgl.gl_ctx != 0);
    sapp.wgl.DeleteContext.unwrap()(sapp.wgl.gl_ctx);
    sapp.wgl.gl_ctx = 0;
}

unsafe fn sapp_wgl_swap_buffers(sapp: &SAppData) {
    debug_assert!(sapp.win32.dc != 0);
    /* FIXME: DwmIsCompositionEnabled? (see GLFW) */
    SwapBuffers(sapp.win32.dc);
}

#[derive(Clone, Copy)]
pub struct SappImageDesc<'a> {
    pub width: u32,
    pub height: u32,
    pub pixels: &'a [u32],
}

static EMPTY_ARRAY: [u32; 0] = [0; 0];
impl<'a> SappImageDesc<'a> {
    pub fn new() -> SappImageDesc<'a> {
        SappImageDesc {
            width: 0,
            height: 0,
            pixels: &EMPTY_ARRAY,
        }
    }
}

pub struct SappIconDesc<'a> {
    pub sokol_default: bool,
    pub images: [SappImageDesc<'a>; SAPP_MAX_ICONIMAGES as usize],
}

impl<'a> SappIconDesc<'a> {
    pub fn new() -> SappIconDesc<'a> {
        SappIconDesc {
            sokol_default: false,
            images: [SappImageDesc::new(); SAPP_MAX_ICONIMAGES as usize],
        }
    }
}

pub struct Clipboard {
    enabled: bool,
    buffer: String,
}

pub struct SDrop {
    max_files: u32,
    file_paths: Vec<String>,
}

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
    pub max_dropped_files: u32, // enable file dropping (drag'n'drop) if > 1 - max number of dropped files to process
    pub icon: SappIconDesc<'a>, // the initial window icon to set
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
            max_dropped_files: 0,
            icon: SappIconDesc::new(),

            gl_major_version: 3,
            gl_minor_version: 2,
            win32_console_utf8: false,
            win32_console_create: false,
            win32_console_attach: false,
        }
    }
}

pub struct SAppData {
    valid: bool,
    fullscreen: bool,

    first_frame: bool,
    init_called: bool,
    cleanup_called: bool,
    quit_requested: bool,
    quit_ordered: bool,

    window_width: u32,
    window_height: u32,
    framebuffer_width: u32,
    framebuffer_height: u32,
    sample_count: u32,
    swap_interval: u32,
    high_dpi: bool,
    dpi_scale: f32,
    frame_count: u64,

    mouse: SAppMouse,
    clipboard: Clipboard,
    drop: SDrop,
    win32: SAppWin32,
    wgl: SAppWgl,
    keycodes: [KeyCode; SAPP_MAX_KEYCODES as usize],
}

impl SAppData {
    fn new(desc: &SAppDesc) -> SAppData {
        SAppData {
            valid: false,
            fullscreen: desc.fullscreen,
            first_frame: true,
            init_called: false,
            cleanup_called: false,
            quit_requested: false,
            quit_ordered: false,

            // NOTE: _sapp.desc.width/height may be 0! Platform backends need to deal with this
            window_width: desc.width,
            window_height: desc.height,
            framebuffer_width: desc.width,
            framebuffer_height: desc.height,
            sample_count: desc.sample_count,
            swap_interval: desc.swap_interval,
            clipboard: Clipboard {
                enabled: desc.enable_clipboard,
                buffer: String::with_capacity(desc.clipboard_size as usize),
            },
            drop: SDrop {
                max_files: desc.max_dropped_files,
                file_paths: Vec::with_capacity(desc.max_dropped_files as usize),
            },
            high_dpi: desc.high_dpi,
            dpi_scale: 1.0,
            frame_count: 0,
            mouse: SAppMouse::new(),
            win32: SAppWin32::new(desc),
            wgl: SAppWgl::new(desc),
            keycodes: [KeyCode::Invalid; SAPP_MAX_KEYCODES as usize],
        }
    }

    pub fn request_quit(&mut self) {
        self.quit_requested = true;
    }

    pub fn lock_mouse(&mut self, lock: bool) {
        unsafe {
            sapp_win32_lock_mouse(self, lock);
        }
    }

    pub fn mouse_locked(&self) -> bool {
        self.mouse.locked
    }

    pub fn get_modifiers() -> Modifier {
        unsafe { sapp_win32_mods() }
    }

    pub fn set_icon(&mut self, desc: &SappIconDesc) {
        if desc.sokol_default {
            let mut icon_buffer: Vec<u32> = vec![0; 0];
            self.set_icon(&sapp_setup_default_icon(&mut icon_buffer));
            return;
        }
        let num_images = sapp_icon_num_images(&desc);
        if num_images == 0 || num_images > SAPP_MAX_ICONIMAGES {
            return;
        }
        if !sapp_validate_icon_desc(&desc, num_images) {
            return;
        }
        unsafe {
            sapp_win32_set_icon(self, &desc, num_images);
        }
    }

    // NOTE that sapp_show_mouse() does not "stack" like the Win32 or macOS API functions!
    pub fn show_mouse(&mut self, show: bool) {
        if self.mouse.shown != show {
            sapp_win32_update_cursor(&self, self.mouse.current_cursor, show, false);
            self.mouse.shown = show;
        }
    }

    pub fn mouse_shown(&self) -> bool {
        self.mouse.shown
    }

    pub fn set_mouse_cursor(&mut self, cursor: MouseCursor) {
        if self.mouse.current_cursor != cursor {
            sapp_win32_update_cursor(&self, cursor, self.mouse.shown, false);
            self.mouse.current_cursor = cursor;
        }
    }

    pub fn get_mouse_cursor(&self) -> MouseCursor {
        self.mouse.current_cursor
    }

    /* NOTE: on HTML5, sapp_set_clipboard_string() must be called from within event handler! */
    pub fn set_clipboard_string(&mut self, str: &str) {
        if !self.clipboard.enabled {
            return;
        }
        unsafe {
            sapp_win32_set_clipboard_string(self, str);
        }

        self.clipboard.buffer.clear();
        self.clipboard.buffer.push_str(str);
    }

    pub fn get_clipboard_string(&mut self) -> &str {
        if !self.clipboard.enabled {
            return "";
        }
        unsafe {
            sapp_win32_get_clipboard_string(self);
        }
        self.clipboard.buffer.as_str()
    }

    pub fn get_dropped_file_paths(&self) -> &Vec<String> {
        &self.drop.file_paths
    }

    pub fn is_fullscreen(&self) -> bool {
        self.fullscreen
    }

    pub fn toggle_fullscreen(&mut self) {
        sapp_win32_toggle_fullscreen(self);
    }
}

pub struct SApp<'a> {
    base: SAppData,
    app: &'a mut dyn SAppI,
}

impl<'a> SApp<'a> {
    fn call_event(&mut self, event: &Event) {
        if !self.base.cleanup_called && self.base.init_called {
            self.app.on_event(&mut self.base, event);
        }
    }

    fn call_init(&mut self) {
        self.app.init(&mut self.base);
        self.base.init_called = true;
    }

    fn frame(&mut self) {
        if self.base.first_frame {
            self.base.first_frame = false;
            self.call_init();
        }
        self.call_frame();
        self.base.frame_count += 1;
    }

    fn call_frame(&mut self) {
        if self.base.init_called && !self.base.cleanup_called {
            self.app.draw_frame(&mut self.base);
        }
    }

    fn call_cleanup(&mut self) {
        if !self.base.cleanup_called {
            self.app.shutdown(&mut self.base);
            self.base.cleanup_called = true;
        }
    }
}

pub fn run_app(app: &mut dyn SAppI, desc: &SAppDesc) {
    let timer = Timer::new();
    let mut last_time = 1;

    let mut sapp = SApp {
        base: SAppData::new(&desc),
        app,
    };

    unsafe {
        sapp_win32_init_console(&mut sapp.base);
    }
    sapp_win32_init_keytable(&mut sapp.base.keycodes);

    println!("* Init console keytable {} ms", Timer::ms(timer.laptime(&mut last_time)));

    unsafe {
        sapp_win32_init_dpi(&mut sapp.base);
    }
    println!("* Init DPI {} ms", Timer::ms(timer.laptime(&mut last_time)));

    sapp_win32_init_cursors(&mut sapp.base);

    println!("* Init Cursor {} ms", Timer::ms(timer.laptime(&mut last_time)));

    unsafe {
        sapp_win32_create_window(&desc, &mut sapp.base);
    }
    println!("* Create Window {} ms", Timer::ms(timer.laptime(&mut last_time)));

    sapp.base.set_icon(&desc.icon);
    println!("* Set Icon {} ms", Timer::ms(timer.laptime(&mut last_time)));

    unsafe {
        sapp_wgl_init(&mut sapp.base);
        println!("* WGL Init {} ms", Timer::ms(timer.laptime(&mut last_time)));

        sapp_wgl_load_extensions(&mut sapp.base);
        println!("* WGL Load Ext {} ms", Timer::ms(timer.laptime(&mut last_time)));

        sapp_wgl_create_context(&mut sapp.base);
        println!("* WGL Create context {} ms", Timer::ms(timer.laptime(&mut last_time)));        
    }
    sapp.base.valid = true;

    unsafe { win32_process_loop(&mut sapp) };

    sapp.call_cleanup();

    unsafe {
        sapp_wgl_destroy_context(&mut sapp.base);
        sapp_wgl_shutdown(&mut sapp.base);
        sapp_win32_destroy_window(&mut sapp.base);
        sapp_win32_destroy_icons(&mut sapp.base);
        sapp_win32_restore_console(&mut sapp.base);
    }
}
