use std::{time::{Instant, Duration}, sync::{Arc, Mutex}};
use lazy_static::lazy_static;
use rand::{Rng, thread_rng};
use crate::{utils::{config::{Config, CONFIG, TriggerbotConfigs, TriggerbotConfig}, mouse::{MOUSE_LOCKED, click_mouse, press_mouse}}, cheat::functions::{is_feature_toggled, WeaponType}};

lazy_static! {
    pub static ref FEATURE_TOGGLED: Arc<Mutex<bool>> = Arc::new(Mutex::new(CONFIG.lock().unwrap().triggerbot.default));
    pub static ref TOGGLE_CHANGED: Arc<Mutex<Instant>> = Arc::new(Mutex::new(Instant::now()));

    pub static ref TB_SHOT_ENTITY: Arc<Mutex<Instant>> = Arc::new(Mutex::new(Instant::now()));
    pub static ref TB_LOCKED_ENTITY: Arc<Mutex<Option<(Instant, u64)>>> = Arc::new(Mutex::new(None));
    pub static ref TB_OFF_ENTITY: Arc<Mutex<Option<Instant>>> = Arc::new(Mutex::new(None));
}

pub fn get_triggerbot_toggled(config: Config) -> bool {
    let feature = config.triggerbot;
    let mut toggled = FEATURE_TOGGLED.lock().unwrap();
    let mut changed = TOGGLE_CHANGED.lock().unwrap();

    return is_feature_toggled(feature.key, feature.mode, &mut toggled, &mut changed);
}

pub fn get_triggerbot_config(configs: TriggerbotConfigs, weapon_type: WeaponType) -> TriggerbotConfig {
    return match weapon_type {
        WeaponType::Pistol => configs.pistol,
        WeaponType::Rifle => configs.rifle,
        WeaponType::Submachine => configs.submachine,
        WeaponType::Sniper => configs.sniper,
        WeaponType::Shotgun => configs.shotgun,
        WeaponType::MachineGun => configs.machinegun,
        WeaponType::Knife => configs.knife,
        _ => configs.other
    };
}

pub fn run_triggerbot(address: u64, config: TriggerbotConfig) {
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

        let delay_offset = if config.delay_offset == 0 { 0.0 } else { (thread_rng().gen_range(-(config.delay_offset as f32) .. config.delay_offset as f32) * 1000.0).trunc() / 1000.0 };
        let delay = Duration::from_secs_f32((config.delay as f32 + delay_offset).min(500.0).max(0.0) / 1000.0);

        if locked_on.elapsed() < delay {
            return;
        }
    }
    
    let interval_offset = if config.tap_interval_offset == 0 { 0.0 } else { (thread_rng().gen_range(-(config.tap_interval_offset as f32) .. config.tap_interval_offset as f32) * 1000.0).trunc() / 1000.0 };
    let interval = Duration::from_secs_f32((config.tap_interval as f32 + interval_offset).min(500.0).max(50.0) / 1000.0);

    if config.action == 0 && shot_entity.elapsed() >= interval {
        click_mouse();
        *shot_entity = Instant::now();
    } else if config.action == 1 && !mouse_locked {
        press_mouse();
    }
}