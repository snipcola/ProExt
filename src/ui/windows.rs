use glium::{glutin::{ContextBuilder, window::WindowBuilder, platform::windows::WindowBuilderExtWindows, event_loop::EventLoop}, Display};
use imgui::{Context, FontSource};
use imgui_winit_support::{WinitPlatform, HiDpiMode};

use windows::{Win32::{Foundation::{HWND, RECT, POINT}, UI::WindowsAndMessaging::IsWindow}, core::HSTRING};
use windows::Win32::UI::WindowsAndMessaging::{GetClientRect, GetForegroundWindow, SetForegroundWindow, FindWindowW};
use windows::Win32::Graphics::Gdi::ClientToScreen;
use windows::core::PCWSTR;

pub fn find_window(title: &str, class: Option<&str>) -> Option<HWND> {
    unsafe {
        let hwnd = match class {
            Some(class) => FindWindowW(&HSTRING::from(class.to_string()), &HSTRING::from(title.to_string())),
            _ => FindWindowW(PCWSTR::null(), &HSTRING::from(title.to_string()))
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
            _ => {
                return None
            }
        }
    }
}

pub fn is_window_focused(window: HWND) -> bool {
    return unsafe { GetForegroundWindow() } == window;
}

pub fn focus_window(window: HWND) -> bool {
    return unsafe { SetForegroundWindow(window).into() };
}

pub fn create_window(title: &str, window_hwnd: HWND) -> (EventLoop<()>, Display) {
    let event_loop = EventLoop::new();
    let context = ContextBuilder::new().with_vsync(true);
    let builder = WindowBuilder::new()
        .with_owner_window(window_hwnd.0)
        .with_title(title)
        .with_transparent(true)
        .with_decorations(false)
        .with_resizable(false)
        .with_skip_taskbar(true)
        .with_drag_and_drop(false)
        .with_undecorated_shadow(false);
    let display = Display::new(builder, context, &event_loop).unwrap();

    return (event_loop, display);
}

pub fn init_imgui(display: &Display) -> (WinitPlatform, Context) {
    let mut imgui_context = Context::create();
    imgui_context.set_ini_filename(None);
    imgui_context.set_log_filename(None);
    imgui_context.io_mut().config_windows_move_from_title_bar_only = true;

    let mut winit_platform = WinitPlatform::init(&mut imgui_context);
    let gl_window = display.gl_window();
    winit_platform.attach_window(imgui_context.io_mut(), gl_window.window(), HiDpiMode::Default);
    imgui_context.fonts().add_font(&[FontSource::DefaultFontData { config: None }]);
    imgui_context.io_mut().font_global_scale = (1.0 / winit_platform.hidpi_factor()) as f32;

    return (winit_platform, imgui_context);
}