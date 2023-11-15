use std::{time::{Instant, Duration}, sync::{Arc, Mutex}};
use mouse_rs::{Mouse, types::keys::Keys};
use lazy_static::lazy_static;
use crate::{utils::config::Config, ui::main::hotkey_index_to_io};

lazy_static! {
    pub static ref SHOT_ENTITY: Arc<Mutex<Instant>> = Arc::new(Mutex::new(Instant::now()));
    pub static ref ON_ENTITY: Arc<Mutex<Option<Instant>>> = Arc::new(Mutex::new(None));
    pub static ref TRIES: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
    pub static ref MOUSE_LOCKED: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    pub static ref MOUSE: Arc<Mutex<Mouse>> = Arc::new(Mutex::new(Mouse::new()));
}

pub fn click_mouse() {
    let mouse = MOUSE.lock().unwrap();

    mouse.click(&Keys::LEFT).ok();
}

pub fn lock_mouse() {
    let mouse = MOUSE.lock().unwrap();

    mouse.press(&Keys::LEFT).ok();
    *MOUSE_LOCKED.lock().unwrap() = true;
}

pub fn unlock_mouse() {
    let mouse = MOUSE.lock().unwrap();

    mouse.release(&Keys::LEFT).ok();
    *MOUSE_LOCKED.lock().unwrap() = false;
}

pub fn get_triggerbot_toggled(config: Config) -> bool {
    match hotkey_index_to_io(config.triggerbot.key) {
        Ok(triggerbot_button) => {
            return triggerbot_button.is_pressed();
        },
        Err(triggerbot_key) => {
            return triggerbot_key.is_pressed();
        }
    }
}

pub fn run_triggerbot((aiming_at_enemy, allow_shoot): (bool, bool), config: Config) {
    let mouse_locked = MOUSE_LOCKED.lock().unwrap().clone();
    let mut on_entity = ON_ENTITY.lock().unwrap();
    let mut shot_entity = SHOT_ENTITY.lock().unwrap();
    let mut tries = TRIES.lock().unwrap();

    if !aiming_at_enemy {
        if *tries > 500 {
            if mouse_locked {
                unlock_mouse();
            }

            *on_entity = None;
            *tries = 0;
        } else {
            *tries += 1;
        }

        return;
    }

    *tries = 0;

    if !allow_shoot {
        return;
    }

    if let Some(on_entity) = *on_entity {
        if on_entity.elapsed() >= Duration::from_millis(config.triggerbot.delay as u64) {
            if config.triggerbot.mode == 0 && shot_entity.elapsed() >= Duration::from_millis(config.triggerbot.tap_interval as u64) {
                click_mouse();
                *shot_entity = Instant::now();
            } else if !mouse_locked {
                lock_mouse();
            }
        }
    } else {
        *on_entity = Some(Instant::now());
    }
}