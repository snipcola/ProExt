use std::{time::{Instant, Duration}, sync::{Arc, Mutex}};
use lazy_static::lazy_static;
use rand::{Rng, thread_rng};
use crate::utils::{config::Config, mouse::{MOUSE_LOCKED, click_mouse, press_mouse}};
use crate::ui::functions::hotkey_index_to_io;

lazy_static! {
    pub static ref TB_SHOT_ENTITY: Arc<Mutex<Instant>> = Arc::new(Mutex::new(Instant::now()));
    pub static ref TB_LOCKED_ENTITY: Arc<Mutex<Option<(Instant, u64)>>> = Arc::new(Mutex::new(None));
    pub static ref TB_OFF_ENTITY: Arc<Mutex<Option<Instant>>> = Arc::new(Mutex::new(None));
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

pub fn run_triggerbot(address: u64, config: Config) {
    let mouse_locked = MOUSE_LOCKED.lock().unwrap().clone();
    let mut shot_entity = TB_SHOT_ENTITY.lock().unwrap();
    let mut locked_entity = TB_LOCKED_ENTITY.lock().unwrap();

    if locked_entity.is_none() {
        *locked_entity = Some((Instant::now(), address));
    }

    if let Some((locked_on, entity_address)) = *locked_entity {
        if entity_address != address {
            *locked_entity = None;
            return;
        }

        let delay_offset = if config.triggerbot.delay_offset == 0 { 0.0 } else { (thread_rng().gen_range(-(config.triggerbot.delay_offset as f32) .. config.triggerbot.delay_offset as f32) * 1000.0).trunc() / 1000.0 };
        let delay = Duration::from_secs_f32((config.triggerbot.delay as f32 + delay_offset).min(500.0).max(0.0) / 1000.0);

        if locked_on.elapsed() < delay {
            return;
        }
    }
    
    let interval_offset = if config.triggerbot.tap_interval_offset == 0 { 0.0 } else { (thread_rng().gen_range(-(config.triggerbot.tap_interval_offset as f32) .. config.triggerbot.tap_interval_offset as f32) * 1000.0).trunc() / 1000.0 };
    let interval = Duration::from_secs_f32((config.triggerbot.tap_interval as f32 + interval_offset).min(500.0).max(50.0) / 1000.0);

    if config.triggerbot.mode == 0 && shot_entity.elapsed() >= interval {
        click_mouse();
        *shot_entity = Instant::now();
    } else if config.triggerbot.mode == 1 && !mouse_locked {
        press_mouse();
    }
}