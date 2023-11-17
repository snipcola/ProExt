use std::{time::Instant, sync::{Arc, Mutex}, collections::HashMap};
use colored::Colorize;
use glium::{glutin::event_loop::EventLoop, Display};
use imgui::{Ui, Context};
use imgui_winit_support::WinitPlatform;
use lazy_static::lazy_static;

use crate::{ui::thread::{bind_ui_keys, run_event_loop}, cheat::thread::run_cheats_thread, utils::rpc::initialize_rpc};
use crate::utils::config::{PACKAGE_NAME, PACKAGE_VERSION, PACKAGE_AUTHORS, PROCESS_TITLE, PROCESS_CLASS};
use crate::ui::windows::{create_window, find_window, focus_window, init_imgui};

lazy_static! {
    pub static ref WINDOW_INFO: Arc<Mutex<Option<((i32, i32), (i32, i32))>>> = Arc::new(Mutex::new(None));
    pub static ref WINDOW_LAST_MOVED: Arc<Mutex<Instant>> = Arc::new(Mutex::new(Instant::now()));
    pub static ref WINDOW_FOCUSED: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    pub static ref UI_FUNCTIONS: Arc<Mutex<HashMap<String, Box<dyn Fn(&mut Ui) + Send>>>> = Arc::new(Mutex::new(HashMap::new()));
    
    pub static ref TOGGLED: Arc<Mutex<bool>> = Arc::new(Mutex::new(true));
    pub static ref EXIT: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
}

pub fn init_gui() {
    let title = &format!("{} v{} - {}", (*PACKAGE_NAME), (*PACKAGE_VERSION), (*PACKAGE_AUTHORS));
    let window_title = &*PROCESS_TITLE;
    let window_class = &*PROCESS_CLASS;

    let window_hwnd = match find_window(window_title, Some(window_class)) {
        Some(hwnd) => hwnd,
        None => {
            println!("{} Failed to find {} window", "[ FAIL ]".bold().red(), window_title.bold());
            return;
        }
    };

    let event_loop_display: Arc<Mutex<(EventLoop<()>, Display)>> = Arc::new(Mutex::new(create_window(title, window_hwnd)));
    let winit_platform_imgui_context: Arc<Mutex<(WinitPlatform, Context)>> = Arc::new(Mutex::new(init_imgui(&event_loop_display.lock().unwrap().1)));

    let self_hwnd = match find_window(title, None) {
        Some(hwnd) => hwnd,
        None => {
            println!("{} Failed to find {} window", "[ FAIL ]".bold().red(), title.bold());
            return;
        }
    };

    initialize_rpc();
    focus_window(self_hwnd);
    run_cheats_thread(window_hwnd, self_hwnd);
    bind_ui_keys(window_hwnd, self_hwnd);
    run_event_loop(event_loop_display, winit_platform_imgui_context, window_hwnd, self_hwnd);
}