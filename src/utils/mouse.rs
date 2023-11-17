use std::sync::{Arc, Mutex};
use mouse_rs::{Mouse, types::keys::Keys};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref MOUSE_LOCKED: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    pub static ref MOUSE: Arc<Mutex<Mouse>> = Arc::new(Mutex::new(Mouse::new()));
}

pub fn click_mouse() {
    let mouse = MOUSE.lock().unwrap();

    mouse.click(&Keys::LEFT).ok();
}

pub fn press_mouse() {
    let mouse = MOUSE.lock().unwrap();

    mouse.press(&Keys::LEFT).ok();
    *MOUSE_LOCKED.lock().unwrap() = true;
}

pub fn release_mouse() {
    let mouse = MOUSE.lock().unwrap();

    mouse.release(&Keys::LEFT).ok();
    *MOUSE_LOCKED.lock().unwrap() = false;
}