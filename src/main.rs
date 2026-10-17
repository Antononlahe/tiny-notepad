#![windows_subsystem = "windows"]

use std::ffi::c_void;
use std::mem::zeroed;
use std::ptr::{null, null_mut};
use std::sync::atomic::{AtomicBool, AtomicPtr, Ordering};

type Handle = *mut c_void;
type Hwnd = Handle;
type Hdc = Handle;
type Hbrush = Handle;
type Hhook = Handle;
type Hinstance = Handle;
type Wparam = usize;
type Lparam = isize;
type Lresult = isize;

const CLASS_NAME: &[u16] = &[84, 105, 110, 121, 78, 111, 116, 101, 112, 97, 100, 0];
const WINDOW_TITLE: &[u16] = &[84, 105, 110, 121, 32, 78, 111, 116, 101, 112, 97, 100, 0];
const EDIT_CLASS: &[u16] = &[69, 68, 73, 84, 0];
const EMPTY_TEXT: &[u16] = &[0];
const FONT_NAME: &[u16] = &[67, 111, 110, 115, 111, 108, 97, 115, 0];
const MAXIMIZE_LABEL: &[u16] = &[77, 65, 88, 73, 77, 73, 90, 69, 0];
const RESTORE_LABEL: &[u16] = &[82, 69, 83, 84, 79, 82, 69, 0];
const HOOK_ERROR: &[u16] = &[
    84, 104, 101, 32, 107, 101, 121, 98, 111, 97, 114, 100, 32, 103, 117, 97, 114, 100, 32, 99,
    111, 117, 108, 100, 32, 110, 111, 116, 32, 115, 116, 97, 114, 116, 46, 32, 84, 104, 101, 32,
    119, 105, 110, 100, 111, 119, 32, 119, 105, 108, 108, 32, 114, 101, 109, 97, 105, 110, 32, 105,
    110, 32, 105, 116, 115, 32, 99, 117, 114, 114, 101, 110, 116, 32, 115, 105, 122, 101, 46, 0,
];

const WM_CREATE: u32 = 0x0001;
const WM_DESTROY: u32 = 0x0002;
const WM_SIZE: u32 = 0x0005;
const WM_PAINT: u32 = 0x000F;
const WM_ERASEBKGND: u32 = 0x0014;
const WM_SETFONT: u32 = 0x0030;
const WM_SETICON: u32 = 0x0080;
const WM_LBUTTONUP: u32 = 0x0202;
const WM_KEYDOWN: u32 = 0x0100;
const WM_KEYUP: u32 = 0x0101;
const WM_SYSKEYDOWN: u32 = 0x0104;
const WM_SYSKEYUP: u32 = 0x0105;
const WM_CTLCOLOREDIT: u32 = 0x0133;

const WS_CHILD: u32 = 0x40000000;
const WS_TABSTOP: u32 = 0x00010000;
const WS_CAPTION: u32 = 0x00C00000;
const WS_SYSMENU: u32 = 0x00080000;
const WS_MINIMIZEBOX: u32 = 0x00020000;
const WS_POPUP: u32 = 0x80000000;
const WS_VISIBLE: u32 = 0x10000000;
const WS_EX_APPWINDOW: u32 = 0x00040000;
const WS_EX_TOPMOST: u32 = 0x00000008;
const ES_MULTILINE: u32 = 0x0004;
const ES_AUTOVSCROLL: u32 = 0x0040;
const ES_WANTRETURN: u32 = 0x1000;
const ES_NOHIDESEL: u32 = 0x0100;

const GWL_STYLE: i32 = -16;
const GWL_EXSTYLE: i32 = -20;
const SW_SHOW: i32 = 5;
const SWP_NOZORDER: u32 = 0x0004;
const SWP_NOACTIVATE: u32 = 0x0010;
const SWP_NOOWNERZORDER: u32 = 0x0200;
const SWP_FRAMECHANGED: u32 = 0x0020;
const SWP_SHOWWINDOW: u32 = 0x0040;
const HWND_TOPMOST: Hwnd = -1isize as Hwnd;
const HWND_NOTOPMOST: Hwnd = -2isize as Hwnd;
const SM_CXSCREEN: i32 = 0;
const SM_CYSCREEN: i32 = 1;
const LOGPIXELSY: i32 = 90;
const FW_NORMAL: i32 = 400;
const DEFAULT_CHARSET: u32 = 1;
const CLEARTYPE_QUALITY: u32 = 5;
const FIXED_PITCH_MODERN: u32 = 0x31;
const TRANSPARENT: i32 = 1;
const DT_CENTER: u32 = 0x00000001;
const DT_VCENTER: u32 = 0x00000004;
const DT_SINGLELINE: u32 = 0x00000020;
const MB_OK: u32 = 0x00000000;
const MB_ICONERROR: u32 = 0x00000010;
const WH_KEYBOARD_LL: i32 = 13;
const ICON_SMALL: usize = 0;
const ICON_BIG: usize = 1;
const LR_DEFAULTCOLOR: u32 = 0;

const WINDOWED_WIDTH: i32 = 520;
const WINDOWED_HEIGHT: i32 = 400;
const BUTTON_X: i32 = 24;
const BUTTON_Y: i32 = 20;
const BUTTON_WIDTH: i32 = 196;
const BUTTON_HEIGHT: i32 = 68;
const EDIT_LEFT: i32 = 24;
const EDIT_TOP: i32 = 108;
const EDIT_RIGHT_MARGIN: i32 = 24;
const EDIT_BOTTOM_MARGIN: i32 = 24;

static IS_FULLSCREEN: AtomicBool = AtomicBool::new(false);
static CONTROL_DOWN: AtomicBool = AtomicBool::new(false);
static ALT_DOWN: AtomicBool = AtomicBool::new(false);
static WINDOWS_KEY_DOWN: AtomicBool = AtomicBool::new(false);
static KEYBOARD_HOOK: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
static MAIN_WINDOW: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
static EDIT_WINDOW: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
static BLACK_BRUSH: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
static TEXT_FONT: AtomicPtr<c_void> = AtomicPtr::new(null_mut());
static APP_ICON: AtomicPtr<c_void> = AtomicPtr::new(null_mut());

const ICON_BYTES: &[u8] = include_bytes!("../shortcut-notepad.ico");

#[repr(C)]
struct Point {
    x: i32,
    y: i32,
}

#[repr(C)]
struct Rect {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}

#[repr(C)]
struct WndClassW {
    style: u32,
    wnd_proc: Option<unsafe extern "system" fn(Hwnd, u32, Wparam, Lparam) -> Lresult>,
    cb_cls_extra: i32,
    cb_wnd_extra: i32,
    instance: Hinstance,
    icon: Handle,
    cursor: Handle,
    background: Hbrush,
    menu_name: *const u16,
    class_name: *const u16,
}

#[repr(C)]
struct Message {
    hwnd: Hwnd,
    message: u32,
    w_param: Wparam,
    l_param: Lparam,
    time: u32,
    point: Point,
    l_private: u32,
}

#[repr(C)]
struct PaintStruct {
    hdc: Hdc,
    erase: i32,
    paint: Rect,
    restore: i32,
    inc_update: i32,
    reserved: [u8; 32],
}

#[repr(C)]
struct KeyboardLowLevel {
    virtual_key: u32,
    scan_code: u32,
    flags: u32,
    time: u32,
    extra_info: usize,
}

#[link(name = "user32")]
unsafe extern "system" {
    fn BeginPaint(hwnd: Hwnd, paint: *mut PaintStruct) -> Hdc;
    fn CallNextHookEx(hook: Hhook, code: i32, w_param: Wparam, l_param: Lparam) -> Lresult;
    fn CreateIconFromResourceEx(
        bits: *const u8,
        size: u32,
        is_icon: i32,
        version: u32,
        width: i32,
        height: i32,
        flags: u32,
    ) -> Handle;
    fn CreateWindowExW(
        ex_style: u32,
        class_name: *const u16,
        window_name: *const u16,
        style: u32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        parent: Hwnd,
        menu: Handle,
        instance: Hinstance,
        param: *mut c_void,
    ) -> Hwnd;
    fn DefWindowProcW(hwnd: Hwnd, message: u32, w_param: Wparam, l_param: Lparam) -> Lresult;
    fn DestroyIcon(icon: Handle) -> i32;
    fn DispatchMessageW(message: *const Message) -> Lresult;
    fn DrawTextW(hdc: Hdc, text: *const u16, count: i32, rect: *mut Rect, format: u32) -> i32;
    fn EndPaint(hwnd: Hwnd, paint: *const PaintStruct) -> i32;
    fn FillRect(hdc: Hdc, rect: *const Rect, brush: Hbrush) -> i32;
    fn FrameRect(hdc: Hdc, rect: *const Rect, brush: Hbrush) -> i32;
    fn GetClientRect(hwnd: Hwnd, rect: *mut Rect) -> i32;
    fn GetDC(hwnd: Hwnd) -> Hdc;
    fn GetForegroundWindow() -> Hwnd;
    fn GetMessageW(message: *mut Message, hwnd: Hwnd, min: u32, max: u32) -> i32;
    fn GetSystemMetrics(index: i32) -> i32;
    fn InvalidateRect(hwnd: Hwnd, rect: *const Rect, erase: i32) -> i32;
    fn LoadCursorW(instance: Hinstance, cursor_name: *const u16) -> Handle;
    fn MessageBoxW(hwnd: Hwnd, text: *const u16, caption: *const u16, kind: u32) -> i32;
    fn PostQuitMessage(exit_code: i32);
    fn RegisterClassW(class: *const WndClassW) -> u16;
    fn ReleaseDC(hwnd: Hwnd, dc: Hdc) -> i32;
    fn SendMessageW(hwnd: Hwnd, message: u32, w_param: Wparam, l_param: Lparam) -> Lresult;
    fn SetFocus(hwnd: Hwnd) -> Hwnd;
    fn SetForegroundWindow(hwnd: Hwnd) -> i32;
    fn SetWindowsHookExW(
        hook_id: i32,
        callback: Option<unsafe extern "system" fn(i32, Wparam, Lparam) -> Lresult>,
        module: Hinstance,
        thread_id: u32,
    ) -> Hhook;
    fn SetWindowLongPtrW(hwnd: Hwnd, index: i32, value: isize) -> isize;
    fn SetWindowPos(
        hwnd: Hwnd,
        insert_after: Hwnd,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        flags: u32,
    ) -> i32;
    fn ShowWindow(hwnd: Hwnd, command: i32) -> i32;
    fn TranslateMessage(message: *const Message) -> i32;
    fn UnhookWindowsHookEx(hook: Hhook) -> i32;
    fn UpdateWindow(hwnd: Hwnd) -> i32;
}

#[link(name = "gdi32")]
unsafe extern "system" {
    fn CreateFontW(
        height: i32,
        width: i32,
        escapement: i32,
        orientation: i32,
        weight: i32,
        italic: u32,
        underline: u32,
        strike_out: u32,
        charset: u32,
        output_precision: u32,
        clip_precision: u32,
        quality: u32,
        pitch_and_family: u32,
        face_name: *const u16,
    ) -> Handle;
    fn CreateSolidBrush(color: u32) -> Hbrush;
    fn DeleteObject(object: Handle) -> i32;
    fn GetDeviceCaps(dc: Hdc, index: i32) -> i32;
    fn SelectObject(dc: Hdc, object: Handle) -> Handle;
    fn SetBkColor(dc: Hdc, color: u32) -> u32;
    fn SetBkMode(dc: Hdc, mode: i32) -> i32;
    fn SetTextColor(dc: Hdc, color: u32) -> u32;
}

#[link(name = "kernel32")]
unsafe extern "system" {
    fn GetModuleHandleW(module_name: *const u16) -> Hinstance;
}

unsafe extern "system" fn keyboard_hook(code: i32, w_param: Wparam, l_param: Lparam) -> Lresult {
    if code < 0 {
        return CallNextHookEx(
            KEYBOARD_HOOK.load(Ordering::Relaxed),
            code,
            w_param,
            l_param,
        );
    }

    if !IS_FULLSCREEN.load(Ordering::Relaxed) {
        return CallNextHookEx(
            KEYBOARD_HOOK.load(Ordering::Relaxed),
            code,
            w_param,
            l_param,
        );
    }

    let key = &*(l_param as *const KeyboardLowLevel);
    let down = w_param as u32 == WM_KEYDOWN || w_param as u32 == WM_SYSKEYDOWN;
    let up = w_param as u32 == WM_KEYUP || w_param as u32 == WM_SYSKEYUP;

    if set_modifier_state(key.virtual_key, down, up) {
        return 1;
    }
    if GetForegroundWindow() != MAIN_WINDOW.load(Ordering::Relaxed) {
        return 1;
    }
    if CONTROL_DOWN.load(Ordering::Relaxed)
        || ALT_DOWN.load(Ordering::Relaxed)
        || WINDOWS_KEY_DOWN.load(Ordering::Relaxed)
        || !is_text_or_editing_key(key.virtual_key)
    {
        return 1;
    }

    CallNextHookEx(
        KEYBOARD_HOOK.load(Ordering::Relaxed),
        code,
        w_param,
        l_param,
    )
}

fn set_modifier_state(virtual_key: u32, down: bool, up: bool) -> bool {
    let state = match virtual_key {
        0x11 | 0xA2 | 0xA3 => &CONTROL_DOWN,
        0x12 | 0xA4 | 0xA5 => &ALT_DOWN,
        0x5B | 0x5C => &WINDOWS_KEY_DOWN,
        _ => return false,
    };

    if down {
        state.store(true, Ordering::Relaxed);
    } else if up {
        state.store(false, Ordering::Relaxed);
    }
    true
}

fn is_text_or_editing_key(key: u32) -> bool {
    matches!(
        key,
        0x08 | 0x0D // Backspace, Enter
            | 0x10 | 0xA0 | 0xA1 // Shift
            | 0x14 | 0x90 // Caps Lock, Num Lock
            | 0x20 // Space
            | 0x21..=0x28 // Page, End, Home, arrows
            | 0x2E // Delete
            | 0x30..=0x39 // Top row digits
            | 0x41..=0x5A // Letters
            | 0x60..=0x6F // Numpad
            | 0xBA..=0xC0 // OEM punctuation
            | 0xDB..=0xDF // OEM punctuation
            | 0xE2 | 0xE5 | 0xE7 // OEM 102, IME, Unicode packet
            | 0x15 | 0x17..=0x19 | 0x1C..=0x1F // IME input keys
    )
}

unsafe extern "system" fn window_proc(
    hwnd: Hwnd,
    message: u32,
    w_param: Wparam,
    l_param: Lparam,
) -> Lresult {
    match message {
        WM_CREATE => {
            create_editor(hwnd);
            0
        }
        WM_PAINT => {
            paint_window(hwnd);
            0
        }
        WM_ERASEBKGND => 1,
        WM_SIZE => {
            layout_editor(hwnd);
            InvalidateRect(hwnd, null(), 0);
            0
        }
        WM_LBUTTONUP => {
            let x = (l_param as u16) as i16 as i32;
            let y = ((l_param >> 16) as u16) as i16 as i32;
            if (BUTTON_X..BUTTON_X + BUTTON_WIDTH).contains(&x)
                && (BUTTON_Y..BUTTON_Y + BUTTON_HEIGHT).contains(&y)
            {
                toggle_fullscreen(hwnd);
            }
            focus_editor();
            0
        }
        WM_CTLCOLOREDIT => {
            SetTextColor(w_param as Hdc, 0x00FFFFFF);
            SetBkColor(w_param as Hdc, 0x00000000);
            BLACK_BRUSH.load(Ordering::Relaxed) as Lresult
        }
        WM_KEYDOWN | WM_KEYUP | WM_SYSKEYDOWN | WM_SYSKEYUP => 0,
        WM_DESTROY => {
            remove_keyboard_hook();
            release_graphics();
            PostQuitMessage(0);
            0
        }
        _ => DefWindowProcW(hwnd, message, w_param, l_param),
    }
}

unsafe fn create_editor(hwnd: Hwnd) {
    let brush = CreateSolidBrush(0x00000000);
    BLACK_BRUSH.store(brush, Ordering::Relaxed);

    let dc = GetDC(hwnd);
    let dpi = if dc.is_null() {
        96
    } else {
        GetDeviceCaps(dc, LOGPIXELSY).max(96)
    };
    if !dc.is_null() {
        ReleaseDC(hwnd, dc);
    }
    let height = -((12 * dpi + 36) / 72);
    let font = CreateFontW(
        height,
        0,
        0,
        0,
        FW_NORMAL,
        0,
        0,
        0,
        DEFAULT_CHARSET,
        0,
        0,
        CLEARTYPE_QUALITY,
        FIXED_PITCH_MODERN,
        FONT_NAME.as_ptr(),
    );
    TEXT_FONT.store(font, Ordering::Relaxed);

    let edit = CreateWindowExW(
        0,
        EDIT_CLASS.as_ptr(),
        EMPTY_TEXT.as_ptr(),
        WS_CHILD
            | WS_VISIBLE
            | WS_TABSTOP
            | ES_MULTILINE
            | ES_AUTOVSCROLL
            | ES_WANTRETURN
            | ES_NOHIDESEL,
        EDIT_LEFT,
        EDIT_TOP,
        1,
        1,
        hwnd,
        null_mut(),
        GetModuleHandleW(null()),
        null_mut(),
    );
    EDIT_WINDOW.store(edit, Ordering::Relaxed);
    if !edit.is_null() && !font.is_null() {
        SendMessageW(edit, WM_SETFONT, font as usize, 1);
    }
    layout_editor(hwnd);
}

unsafe fn layout_editor(hwnd: Hwnd) {
    let edit = EDIT_WINDOW.load(Ordering::Relaxed);
    if edit.is_null() {
        return;
    }

    let mut client: Rect = zeroed();
    GetClientRect(hwnd, &mut client);
    let width = (client.right - EDIT_LEFT - EDIT_RIGHT_MARGIN).max(1);
    let height = (client.bottom - EDIT_TOP - EDIT_BOTTOM_MARGIN).max(1);
    SetWindowPos(
        edit,
        null_mut(),
        EDIT_LEFT,
        EDIT_TOP,
        width,
        height,
        SWP_NOZORDER | SWP_NOACTIVATE,
    );
}

unsafe fn paint_window(hwnd: Hwnd) {
    let mut paint: PaintStruct = zeroed();
    let dc = BeginPaint(hwnd, &mut paint);
    let mut client: Rect = zeroed();
    GetClientRect(hwnd, &mut client);

    let black = BLACK_BRUSH.load(Ordering::Relaxed);
    let gray = CreateSolidBrush(0x00C0C0C0);
    FillRect(dc, &client, black);

    let button = Rect {
        left: BUTTON_X,
        top: BUTTON_Y,
        right: BUTTON_X + BUTTON_WIDTH,
        bottom: BUTTON_Y + BUTTON_HEIGHT,
    };
    FillRect(dc, &button, black);
    FrameRect(dc, &button, gray);

    let old_font = SelectObject(dc, TEXT_FONT.load(Ordering::Relaxed));
    SetBkMode(dc, TRANSPARENT);
    SetTextColor(dc, 0x00FFFFFF);
    let label = if IS_FULLSCREEN.load(Ordering::Relaxed) {
        RESTORE_LABEL
    } else {
        MAXIMIZE_LABEL
    };
    DrawTextW(
        dc,
        label.as_ptr(),
        -1,
        &mut Rect {
            left: button.left + 2,
            top: button.top + 2,
            right: button.right - 2,
            bottom: button.bottom - 2,
        },
        DT_CENTER | DT_VCENTER | DT_SINGLELINE,
    );
    SelectObject(dc, old_font);

    DeleteObject(gray);
    EndPaint(hwnd, &paint);
}

unsafe fn toggle_fullscreen(hwnd: Hwnd) {
    if IS_FULLSCREEN.load(Ordering::Relaxed) {
        remove_keyboard_hook();
        IS_FULLSCREEN.store(false, Ordering::Relaxed);

        SetWindowLongPtrW(
            hwnd,
            GWL_STYLE,
            (WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX | WS_VISIBLE) as isize,
        );
        SetWindowLongPtrW(hwnd, GWL_EXSTYLE, WS_EX_APPWINDOW as isize);
        let x = (GetSystemMetrics(SM_CXSCREEN) - WINDOWED_WIDTH) / 2;
        let y = (GetSystemMetrics(SM_CYSCREEN) - WINDOWED_HEIGHT) / 2;
        SetWindowPos(
            hwnd,
            HWND_NOTOPMOST,
            x,
            y,
            WINDOWED_WIDTH,
            WINDOWED_HEIGHT,
            SWP_FRAMECHANGED | SWP_SHOWWINDOW | SWP_NOOWNERZORDER,
        );
    } else {
        let hook = SetWindowsHookExW(
            WH_KEYBOARD_LL,
            Some(keyboard_hook),
            GetModuleHandleW(null()),
            0,
        );
        if hook.is_null() {
            MessageBoxW(
                hwnd,
                HOOK_ERROR.as_ptr(),
                WINDOW_TITLE.as_ptr(),
                MB_OK | MB_ICONERROR,
            );
            return;
        }
        KEYBOARD_HOOK.store(hook, Ordering::Relaxed);
        CONTROL_DOWN.store(false, Ordering::Relaxed);
        ALT_DOWN.store(false, Ordering::Relaxed);
        WINDOWS_KEY_DOWN.store(false, Ordering::Relaxed);
        IS_FULLSCREEN.store(true, Ordering::Relaxed);

        SetWindowLongPtrW(hwnd, GWL_STYLE, (WS_POPUP | WS_VISIBLE) as isize);
        SetWindowLongPtrW(hwnd, GWL_EXSTYLE, WS_EX_TOPMOST as isize);
        SetWindowPos(
            hwnd,
            HWND_TOPMOST,
            0,
            0,
            GetSystemMetrics(SM_CXSCREEN),
            GetSystemMetrics(SM_CYSCREEN),
            SWP_FRAMECHANGED | SWP_SHOWWINDOW | SWP_NOOWNERZORDER,
        );
        SetForegroundWindow(hwnd);
    }

    InvalidateRect(hwnd, null(), 1);
    layout_editor(hwnd);
    focus_editor();
}

unsafe fn focus_editor() {
    let edit = EDIT_WINDOW.load(Ordering::Relaxed);
    if !edit.is_null() {
        SetFocus(edit);
    }
}

unsafe fn remove_keyboard_hook() {
    let hook = KEYBOARD_HOOK.swap(null_mut(), Ordering::Relaxed);
    if !hook.is_null() {
        UnhookWindowsHookEx(hook);
    }
    CONTROL_DOWN.store(false, Ordering::Relaxed);
    ALT_DOWN.store(false, Ordering::Relaxed);
    WINDOWS_KEY_DOWN.store(false, Ordering::Relaxed);
}

unsafe fn load_embedded_icon() -> Handle {
    if ICON_BYTES.len() < 22 {
        return null_mut();
    }

    let size = u32::from_le_bytes(ICON_BYTES[14..18].try_into().unwrap());
    let offset = u32::from_le_bytes(ICON_BYTES[18..22].try_into().unwrap()) as usize;
    let end = offset.saturating_add(size as usize);
    if offset < 22 || end > ICON_BYTES.len() {
        return null_mut();
    }

    CreateIconFromResourceEx(
        ICON_BYTES[offset..end].as_ptr(),
        size,
        1,
        0x00030000,
        0,
        0,
        LR_DEFAULTCOLOR,
    )
}

unsafe fn release_graphics() {
    let brush = BLACK_BRUSH.swap(null_mut(), Ordering::Relaxed);
    if !brush.is_null() {
        DeleteObject(brush);
    }
    let font = TEXT_FONT.swap(null_mut(), Ordering::Relaxed);
    if !font.is_null() {
        DeleteObject(font);
    }
    let icon = APP_ICON.swap(null_mut(), Ordering::Relaxed);
    if !icon.is_null() {
        DestroyIcon(icon);
    }
    EDIT_WINDOW.store(null_mut(), Ordering::Relaxed);
    MAIN_WINDOW.store(null_mut(), Ordering::Relaxed);
}

fn main() {
    unsafe {
        let instance = GetModuleHandleW(null());
        let icon = load_embedded_icon();
        APP_ICON.store(icon, Ordering::Relaxed);

        let class = WndClassW {
            style: 0,
            wnd_proc: Some(window_proc),
            cb_cls_extra: 0,
            cb_wnd_extra: 0,
            instance,
            icon,
            cursor: LoadCursorW(null_mut(), 32512usize as *const u16),
            background: null_mut(),
            menu_name: null(),
            class_name: CLASS_NAME.as_ptr(),
        };
        if RegisterClassW(&class) == 0 {
            return;
        }

        let x = (GetSystemMetrics(SM_CXSCREEN) - WINDOWED_WIDTH) / 2;
        let y = (GetSystemMetrics(SM_CYSCREEN) - WINDOWED_HEIGHT) / 2;
        let hwnd = CreateWindowExW(
            WS_EX_APPWINDOW,
            CLASS_NAME.as_ptr(),
            WINDOW_TITLE.as_ptr(),
            WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX | WS_VISIBLE,
            x,
            y,
            WINDOWED_WIDTH,
            WINDOWED_HEIGHT,
            null_mut(),
            null_mut(),
            instance,
            null_mut(),
        );
        if hwnd.is_null() {
            return;
        }

        MAIN_WINDOW.store(hwnd, Ordering::Relaxed);
        SendMessageW(hwnd, WM_SETICON, ICON_SMALL, icon as Lparam);
        SendMessageW(hwnd, WM_SETICON, ICON_BIG, icon as Lparam);
        ShowWindow(hwnd, SW_SHOW);
        UpdateWindow(hwnd);
        focus_editor();

        let mut message: Message = zeroed();
        loop {
            let result = GetMessageW(&mut message, null_mut(), 0, 0);
            if result <= 0 {
                break;
            }
            TranslateMessage(&message);
            DispatchMessageW(&message);
        }
        remove_keyboard_hook();
    }
}
