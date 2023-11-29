use std::{sync::{Arc, Mutex}, time::Instant};
use windows::Win32::UI::{Input::KeyboardAndMouse::{SendInput, INPUT, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, MOUSEEVENTF_MOVE, MOUSEINPUT, INPUT_MOUSE, MOUSE_EVENT_FLAGS}, WindowsAndMessaging::GetCursorPos};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref MOUSE_LOCKED: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    pub static ref LAST_MOVED: Arc<Mutex<Instant>> = Arc::new(Mutex::new(Instant::now()));
    pub static ref STARTED: Instant = Instant::now();
}

pub fn create_mouse_input(flags: MOUSE_EVENT_FLAGS, dx: i32, dy: i32, data: u32, extra_info: usize) -> INPUT {
    return INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
            mi: MOUSEINPUT { dwFlags: flags, dx, dy, mouseData: data, dwExtraInfo: extra_info, time: 0 }
        }
    };
}

pub fn send_input(input: INPUT) {
    unsafe {
        SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
    }
}

pub fn click_mouse() {
    send_input(create_mouse_input(MOUSEEVENTF_LEFTDOWN, 0, 0, 0, 0));
    send_input(create_mouse_input(MOUSEEVENTF_LEFTUP, 0, 0, 0, 0));
}

pub fn press_mouse() {
    let mut mouse_locked = MOUSE_LOCKED.lock().unwrap();

    if !*mouse_locked {
        send_input(create_mouse_input(MOUSEEVENTF_LEFTDOWN, 0, 0, 0, 0));
        *mouse_locked = true;
    }
}

pub fn release_mouse() {
    let mut mouse_locked = MOUSE_LOCKED.lock().unwrap();

    if *mouse_locked {
        send_input(create_mouse_input(MOUSEEVENTF_LEFTUP, 0, 0, 0, 0));
        *mouse_locked = false;
    }
}

pub fn move_mouse(x: i32, y: i32) {
    send_input(create_mouse_input(MOUSEEVENTF_MOVE, x, y, 0, 0));
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