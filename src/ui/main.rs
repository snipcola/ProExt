use std::{sync::{Arc, Mutex}, collections::HashMap};
use glutin::event_loop::EventLoop;
use imgui::{Ui, Context};
use imgui_winit_support::WinitPlatform;
use lazy_static::lazy_static;
use rand::{Rng, thread_rng, distributions::Alphanumeric};

use crate::{ui::thread::run_event_loop, utils::messagebox::{create_messagebox, MessageBoxStyle}};
use crate::utils::config::ProgramConfig;
use crate::ui::windows::{create_window, find_window};
use crate::ui::windows::Window;
use crate::ui::windows::set_window_brush_to_transparent;
use crate::ui::imgui::init_imgui;

lazy_static! {
    pub static ref WINDOW_INFO: Arc<Mutex<Option<((i32, i32), (i32, i32))>>> = Arc::new(Mutex::new(None));
    pub static ref WINDOWS_ACTIVE: Arc<Mutex<HashMap<String, bool>>> = Arc::new(Mutex::new(HashMap::new()));
    pub static ref UI_FUNCTIONS: Arc<Mutex<HashMap<String, Box<dyn Fn(&mut Ui) + Send>>>> = Arc::new(Mutex::new(HashMap::new()));
    
    pub static ref TOGGLED: Arc<Mutex<bool>> = Arc::new(Mutex::new(true));
    pub static ref EXIT: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
}

pub fn init_gui() {
    let mut rng = thread_rng();
    let self_title_length = rng.gen_range(6 .. 12);
    let self_title: String = rng.sample_iter(&Alphanumeric).take(self_title_length).map(char::from).collect();

    let window_title = ProgramConfig::TargetProcess::Window::Title;
    let window_class = ProgramConfig::TargetProcess::Window::Class;

    let hwnd = match find_window(window_title, Some(window_class)) {
        Some(hwnd) => hwnd,
        None => return create_messagebox(MessageBoxStyle::Error, "Error", &format!("Failed to find {} window.", window_title))
    };

    let event_loop_window: Arc<Mutex<(EventLoop<()>, Window)>> = Arc::new(Mutex::new(create_window(&self_title, hwnd)));
    let winit_platform_imgui_context: Arc<Mutex<(WinitPlatform, Context)>> = Arc::new(Mutex::new(init_imgui(&event_loop_window.lock().unwrap().1)));

    let self_hwnd = match find_window(&self_title, None) {
        Some(hwnd) => hwnd,
        None => return create_messagebox(MessageBoxStyle::Error, "Error", &format!("Failed to find {} window.", ProgramConfig::Package::Name))
    };

    set_window_brush_to_transparent(self_hwnd);
    run_event_loop(event_loop_window, winit_platform_imgui_context, hwnd, self_hwnd);
}