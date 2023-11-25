use std::{time::{Instant, Duration}, sync::{Arc, Mutex}};
use lazy_static::lazy_static;
use rand::{Rng, thread_rng};
use crate::utils::{config::{Config, ProgramConfig}, mouse::{MOUSE_LOCKED, release_mouse, click_mouse, press_mouse}};
use crate::ui::functions::hotkey_index_to_io;

lazy_static! {
    pub static ref TB_SHOT_ENTITY: Arc<Mutex<Instant>> = Arc::new(Mutex::new(Instant::now()));
    pub static ref TB_ON_ENTITY: Arc<Mutex<Option<Instant>>> = Arc::new(Mutex::new(None));
    pub static ref TB_LOCKED_ENTITY: Arc<Mutex<Option<Instant>>> = Arc::new(Mutex::new(None));
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
    let mut shot_entity = TB_SHOT_ENTITY.lock().unwrap();
    let mut on_entity = TB_ON_ENTITY.lock().unwrap();
    let mut locked_entity = TB_LOCKED_ENTITY.lock().unwrap();

    if !aiming_at_enemy && (locked_entity.is_none() || locked_entity.is_some() && locked_entity.unwrap().elapsed().as_millis() > ProgramConfig::CheatDelays::TriggerbotOffEntity.as_millis()) {
        if mouse_locked {
            release_mouse();
        }

        *on_entity = None;
        *locked_entity = None;

        return;
    }

    if !allow_shoot {
        return;
    }

    *locked_entity = Some(Instant::now());
    
    if on_entity.is_none() {
        *on_entity = Some(Instant::now());
    }

    if let Some(on_entity) = *on_entity {
        let delay_offset = if config.triggerbot.delay_offset == 0 { 0.0 } else { (thread_rng().gen_range(-(config.triggerbot.delay_offset as f32) .. config.triggerbot.delay_offset as f32) * 1000.0).trunc() / 1000.0 };
        let delay = Duration::from_secs_f32((config.triggerbot.delay as f32 + delay_offset).min(500.0).max(15.0) / 1000.0);

        if on_entity.elapsed() >= delay {
            let interval_offset = if config.triggerbot.tap_interval_offset == 0 { 0.0 } else { (thread_rng().gen_range(-(config.triggerbot.tap_interval_offset as f32) .. config.triggerbot.tap_interval_offset as f32) * 1000.0).trunc() / 1000.0 };
            let interval = Duration::from_secs_f32((config.triggerbot.tap_interval as f32 + interval_offset).min(500.0).max(50.0) / 1000.0);

            if config.triggerbot.mode == 0 && shot_entity.elapsed() >= interval {
                click_mouse();
                *shot_entity = Instant::now();
            } else if config.triggerbot.mode == 1 && !mouse_locked {
                press_mouse();
            }
        }
    }
}