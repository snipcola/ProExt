use glutin::{event_loop::EventLoop, WindowedContext, PossiblyCurrent, window::WindowBuilder, platform::windows::WindowBuilderExtWindows, ContextBuilder};

use windows::{Win32::{Foundation::{HWND, RECT, POINT}, UI::WindowsAndMessaging::{IsWindow, SetWindowDisplayAffinity, WINDOW_DISPLAY_AFFINITY, GCLP_HBRBACKGROUND, SetClassLongPtrW}}, core::HSTRING};
use windows::Win32::UI::WindowsAndMessaging::{GetClientRect, GetForegroundWindow, FindWindowW};
use windows::Win32::Graphics::Gdi::ClientToScreen;
use windows::core::PCWSTR;

pub type Window = WindowedContext<PossiblyCurrent>;

pub fn find_window(title: &str, class: Option<&str>) -> Option<HWND> {
    unsafe {
        let hwnd = match class {
            Some(class) => FindWindowW(&HSTRING::from(class.to_string()), &HSTRING::from(title.to_string())),
            None => FindWindowW(PCWSTR::null(), &HSTRING::from(title.to_string()))
        };
        
        if IsWindow(hwnd).into() {
            return Some(hwnd);
        }
    }

    return None;
}

pub fn get_window_info(hwnd: HWND) -> Option<((i32, i32), (i32, i32))> {
    let mut client_rect: RECT = RECT { left: 0, top: 0, right: 0, bottom: 0 };
    let mut window_rect: RECT = RECT { left: 0, top: 0, right: 0, bottom: 0 };
    let mut top_left = POINT { x: window_rect.left, y: window_rect.top };
    let mut bottom_right = POINT { x: window_rect.right, y: window_rect.bottom };

    unsafe {
        match GetClientRect(hwnd, &mut client_rect as *mut RECT) {
            Ok(_) => {
                ClientToScreen(hwnd, &mut top_left);
                window_rect.left = top_left.x;
                window_rect.top = top_left.y;

                ClientToScreen(hwnd, &mut bottom_right);
                window_rect.right = bottom_right.x;
                window_rect.bottom = bottom_right.y;

                let client_width = client_rect.right - client_rect.left;
                let client_height = client_rect.bottom - client_rect.top;

                return Some(((window_rect.left, window_rect.top), (client_width, client_height)));
            },
            Err(_) => {
                return None
            }
        }
    }
}

pub fn hide_window_from_capture(hwnd: HWND, toggle: bool) -> bool {
    let mut affinity = WINDOW_DISPLAY_AFFINITY(0x00000000);

    if toggle {
        affinity = WINDOW_DISPLAY_AFFINITY(0x00000011);
    }

    return unsafe {
        match SetWindowDisplayAffinity(hwnd, affinity) {
            Ok(_) => true,
            Err(_) => false
        }
    };
}

pub fn is_window_focused(window: HWND) -> bool {
    return unsafe { GetForegroundWindow() } == window;
}

pub fn create_window(title: &str, hwnd: HWND) -> (EventLoop<()>, Window) {
    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new()
        .with_owner_window(hwnd.0)
        .with_title(title)
        .with_visible(false)
        .with_transparent(true)
        .with_decorations(false)
        .with_resizable(false)
        .with_skip_taskbar(true)
        .with_drag_and_drop(false)
        .with_undecorated_shadow(false);

    let window = unsafe {
        ContextBuilder::new()
            .with_vsync(true)
            .build_windowed(window_builder, &event_loop)
            .unwrap()
            .make_current()
            .unwrap()
    };

    return (event_loop, window);
}

pub fn get_glow_context(window: &Window) -> glow::Context {
    unsafe { glow::Context::from_loader_function(|s| window.get_proc_address(s).cast()) }
}

pub fn set_window_brush_to_transparent(hwnd: HWND) {
    unsafe { SetClassLongPtrW(hwnd, GCLP_HBRBACKGROUND, 0) };
}