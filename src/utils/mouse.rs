use std::{sync::{Arc, Mutex}, time::Instant};
use windows::Win32::UI::{Input::KeyboardAndMouse::{mouse_event, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, MOUSEEVENTF_MOVE}, WindowsAndMessaging::GetCursorPos};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref MOUSE_LOCKED: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    pub static ref LAST_MOVED: Arc<Mutex<Instant>> = Arc::new(Mutex::new(Instant::now()));
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
    *LAST_MOVED.lock().unwrap() = Instant::now();
}

pub fn get_mouse_position() -> Option<(i32, i32)> {
    unsafe {
        let mut position = std::mem::zeroed();
        
        if !GetCursorPos(&mut position).is_ok() {
            return None;
        }

        return Some((position.x, position.y));
    }
}