use std::{time::{Instant, Duration}, sync::{Arc, Mutex}};
use lazy_static::lazy_static;
use rand::{Rng, thread_rng};
use crate::utils::{config::Config, mouse::{MOUSE_LOCKED, release_mouse, click_mouse, press_mouse}};
use crate::ui::functions::hotkey_index_to_io;

lazy_static! {
    pub static ref SHOT_ENTITY: Arc<Mutex<Instant>> = Arc::new(Mutex::new(Instant::now()));
    pub static ref ON_ENTITY: Arc<Mutex<Option<Instant>>> = Arc::new(Mutex::new(None));
    pub static ref TRIES: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
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
                release_mouse();
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
            let offset = if config.triggerbot.tap_interval_offset == 0 { 0.0 } else { (thread_rng().gen_range(-(config.triggerbot.tap_interval_offset as f32) .. config.triggerbot.tap_interval_offset as f32) * 1000.0).trunc() / 1000.0 };
            let interval = Duration::from_secs_f32((config.triggerbot.tap_interval as f32 + offset).min(500.0).max(50.0) / 1000.0);

            if config.triggerbot.mode == 0 && shot_entity.elapsed() >= interval {
                click_mouse();
                *shot_entity = Instant::now();
            } else if !mouse_locked {
                press_mouse();
            }
        }
    } else {
        *on_entity = Some(Instant::now());
    }
}