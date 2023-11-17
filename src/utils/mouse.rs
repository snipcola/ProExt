use std::sync::{Arc, Mutex};
use windows::Win32::UI::Input::KeyboardAndMouse::{mouse_event, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, MOUSEEVENTF_MOVE};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref MOUSE_LOCKED: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
}

pub fn click_mouse() {
    unsafe {
        mouse_event(MOUSEEVENTF_LEFTDOWN, 0, 0, 0, 0);
        mouse_event(MOUSEEVENTF_LEFTUP, 0, 0, 0, 0);
    }
}

pub fn press_mouse() {
    unsafe { mouse_event(MOUSEEVENTF_LEFTDOWN, 0, 0, 0, 0) };
    *MOUSE_LOCKED.lock().unwrap() = true;
}

pub fn release_mouse() {
    unsafe { mouse_event(MOUSEEVENTF_LEFTUP, 0, 0, 0, 0) };
    *MOUSE_LOCKED.lock().unwrap() = false;
}

pub fn move_mouse(x: i32, y: i32) {
    unsafe { mouse_event(MOUSEEVENTF_MOVE, x, y, 0, 0) };
}