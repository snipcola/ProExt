use std::sync::{Arc, Mutex};
use std::time::Instant;
use imgui::{Ui, ImColor32};
use mint::Vector2;
use lazy_static::lazy_static;

use crate::cheat::functions::{is_feature_toggled, WeaponType};
use crate::utils::config::{Config, CONFIG, CrosshairConfigs, CrosshairConfig};
use crate::ui::functions::{color_u32_to_f32, color_with_masked_alpha};

lazy_static! {
    pub static ref FEATURE_TOGGLED: Arc<Mutex<bool>> = Arc::new(Mutex::new(CONFIG.lock().unwrap().crosshair.default));
    pub static ref TOGGLE_CHANGED: Arc<Mutex<Instant>> = Arc::new(Mutex::new(Instant::now()));
}

pub fn get_crosshair_toggled(config: Config) -> bool {
    let feature = config.crosshair;
    let mut toggled = FEATURE_TOGGLED.lock().unwrap();
    let mut changed = TOGGLE_CHANGED.lock().unwrap();

    return is_feature_toggled(feature.key, feature.mode, &mut toggled, &mut changed);
}

pub fn get_crosshair_config(configs: CrosshairConfigs, weapon_type: WeaponType) -> CrosshairConfig {
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

pub fn render_crosshair(ui: &mut Ui, pos: Vector2<f32>, aiming_at_enemy: bool, config: CrosshairConfig) {
    let color = {
        if config.target_enabled && aiming_at_enemy {
            config.target_color
        } else {
            config.color
        }
    };

    let (border_width, dot_size, gap) = (2.0, 1.0 as f32, config.lines_space as f32 / 2.0);
    let (outline_gap, thickness) = (gap - 1.0, config.lines_thickness as f32);

    let offset_1 = Vector2 { x: config.dot_size as f32, y: config.dot_size as f32 };
    let offset_2 = Vector2 { x: offset_1.x + 1.0, y: offset_1.y + 1.0 };

    // Outlines
    if config.outline_enabled {
        if config.dot_enabled {
            ui.get_background_draw_list().add_rect(Vector2 { x: pos.x - offset_1.x, y: pos.y - offset_1.y }, Vector2 { x: pos.x + offset_2.x, y: pos.y + offset_2.y }, ImColor32::from(color_with_masked_alpha(color, 0xFF000000))).filled(true).build();
        }

        if config.lines_enabled {
            ui.get_background_draw_list().add_rect(Vector2 { x: pos.x - (outline_gap + border_width + config.lines_width as f32), y: pos.y - thickness }, Vector2 { x: pos.x - outline_gap, y: pos.y + 1.0 + thickness }, ImColor32::from(color_with_masked_alpha(color, 0xFF000000))).filled(true).build();
            ui.get_background_draw_list().add_rect(Vector2 { x: pos.x + (outline_gap + dot_size), y: pos.y - thickness }, Vector2 { x: pos.x + (outline_gap + dot_size + border_width + config.lines_width as f32), y: pos.y + 1.0 + thickness }, ImColor32::from(color_with_masked_alpha(color, 0xFF000000))).filled(true).build();
            ui.get_background_draw_list().add_rect(Vector2 { x: pos.x - thickness, y: pos.y - (outline_gap + border_width + config.lines_height as f32) }, Vector2 { x: pos.x + 1.0 + thickness, y: pos.y - outline_gap }, ImColor32::from(color_with_masked_alpha(color, 0xFF000000))).filled(true).build();
            ui.get_background_draw_list().add_rect(Vector2 { x: pos.x - thickness, y: pos.y + outline_gap + dot_size }, Vector2 { x: pos.x + 1.0 + thickness, y: pos.y + (outline_gap + dot_size + border_width + config.lines_height as f32) }, ImColor32::from(color_with_masked_alpha(color, 0xFF000000))).filled(true).build();
        }

        if config.circle_enabled {
            ui.get_background_draw_list().add_circle(pos, config.circle_radius as f32, color_with_masked_alpha(color, 0xFF000000)).thickness(3.0).build();
        }
    }

    // Crosshairs
    if config.dot_enabled {
        ui.get_background_draw_list().add_rect(Vector2 { x: pos.x - offset_1.x + dot_size, y: pos.y - offset_1.y + dot_size }, Vector2 { x: pos.x + offset_1.x, y: pos.y + offset_1.y }, color_u32_to_f32(color)).filled(true).build();
    }

    if config.lines_enabled {
        ui.get_background_draw_list().add_rect(Vector2 { x: pos.x - (gap + config.lines_width as f32), y: pos.y - thickness + 1.0 }, Vector2 { x: pos.x - gap, y: pos.y + thickness }, color_u32_to_f32(color)).filled(true).build();
        ui.get_background_draw_list().add_rect(Vector2 { x: pos.x + gap + dot_size, y: pos.y - thickness + 1.0 }, Vector2 { x: pos.x + (gap + dot_size + config.lines_width as f32), y: pos.y + thickness }, color_u32_to_f32(color)).filled(true).build();
        ui.get_background_draw_list().add_rect(Vector2 { x: pos.x - thickness + 1.0, y: pos.y - (gap + config.lines_height as f32) }, Vector2 { x: pos.x + thickness, y: pos.y - gap }, color_u32_to_f32(color)).filled(true).build();
        ui.get_background_draw_list().add_rect(Vector2 { x: pos.x - thickness + 1.0, y: pos.y + gap + dot_size }, Vector2 { x: pos.x + thickness, y: pos.y + (gap + dot_size + config.lines_height as f32) }, color_u32_to_f32(color)).filled(true).build();
    }

    if config.circle_enabled {
        ui.get_background_draw_list().add_circle(pos, config.circle_radius as f32, color_u32_to_f32(color)).build();
    }
}