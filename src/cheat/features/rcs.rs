// Copyright (c) 2024 Snipcola
// SPDX-License-Identifier: MIT

use std::sync::{Arc, Mutex};
use std::time::Instant;

use lazy_static::lazy_static;

use mint::Vector2;
use rand::{Rng, thread_rng};

use crate::utils::mouse::move_mouse;
use crate::utils::cheat::config::{Config, CONFIG, RCSConfigs, RCSConfig};

use crate::cheat::classes::entity::CUtlVector;
use crate::cheat::functions::{cache_to_punch, is_feature_toggled, WeaponType};

lazy_static! {
    pub static ref FEATURE_TOGGLED: Arc<Mutex<bool>> = Arc::new(Mutex::new(CONFIG.lock().unwrap().rcs.default));
    pub static ref TOGGLE_CHANGED: Arc<Mutex<Instant>> = Arc::new(Mutex::new(Instant::now()));
    pub static ref LAST_PUNCH: Arc<Mutex<Vector2<f32>>> = Arc::new(Mutex::new(Vector2 { x: 0.0, y: 0.0 }));
}

pub fn get_rcs_toggled(config: Config) -> bool {
    let feature = config.rcs;
    let mut toggled = FEATURE_TOGGLED.lock().unwrap();
    let mut changed = TOGGLE_CHANGED.lock().unwrap();

    return is_feature_toggled(feature.key, feature.mode, &mut toggled, &mut changed);
}

pub fn get_rcs_config(configs: RCSConfigs, weapon_type: WeaponType) -> RCSConfig {
    return match weapon_type {
        WeaponType::Pistol => configs.pistol,
        WeaponType::Rifle => configs.rifle,
        WeaponType::Submachine => configs.submachine,
        WeaponType::Sniper => configs.sniper,
        WeaponType::Shotgun => configs.shotgun,
        WeaponType::MachineGun => configs.machinegun,
        _ => configs.other
    };
}

pub fn get_rcs_yaw_pitch(config: RCSConfig) -> (f32, f32) {
    let yaw_offset = if config.yaw_offset == 0.0 { 0.0 } else { (thread_rng().gen_range(-config.yaw_offset .. config.yaw_offset) * 1000.0).trunc() / 1000.0 };
    let yaw = (config.yaw + yaw_offset).min(2.0).max(0.0);

    let pitch_offset = if config.pitch_offset == 0.0 { 0.0 } else { (thread_rng().gen_range(-config.pitch_offset .. config.pitch_offset) * 1000.0).trunc() / 1000.0 };
    let pitch = (config.pitch + pitch_offset).min(2.0).max(0.0);

    return (yaw, pitch);
}

pub fn calculate_rcs_position(config: Config, rcs_config: RCSConfig, current_punch: Vector2<f32>, shots_fired: u32) -> Option<(i32, i32)> {
    let (yaw, pitch) = get_rcs_yaw_pitch(rcs_config);
    let mut last_punch = LAST_PUNCH.lock().unwrap();
    let mut new_punch = Vector2 { x: 0.0, y: 0.0 };

    if shots_fired > rcs_config.start_bullet {
        new_punch.x = ((current_punch.y - last_punch.y) * 2.0 / (pitch * config.rcs.sensitivity)) * 50.0;
        new_punch.y = (-(current_punch.x - last_punch.x) * 2.0 / (yaw * config.rcs.sensitivity)) * 50.0;
        *last_punch = current_punch;
    } else {
        *last_punch = current_punch;
        return None;
    }

    return Some((new_punch.x as i32, new_punch.y as i32));
}

pub fn get_rcs_mouse(config: Config, rcs_config: RCSConfig, shots_fired: u32, aim_punch_cache: CUtlVector) -> Option<(i32, i32)> {
    if let Some(punch) = cache_to_punch(aim_punch_cache) {
        return calculate_rcs_position(config, rcs_config, punch, shots_fired);
    }

    return None;
}

pub fn run_rcs((x, y): (i32, i32)) {
    move_mouse(x, y, false);
}