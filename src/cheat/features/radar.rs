// Copyright (c) 2023 Vytrol <vytrol@proton.me>
// SPDX-License-Identifier: MIT

use std::f32::consts::PI;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use lazy_static::lazy_static;

use imgui::{Ui, ImColor32};
use mint::{Vector2, Vector3};

use crate::utils::cheat::config::{Config, CONFIG};
use crate::cheat::functions::is_feature_toggled;

use crate::ui::functions::{color_u32_to_f32, color_with_masked_alpha};

lazy_static! {
    pub static ref FEATURE_TOGGLED: Arc<Mutex<bool>> = Arc::new(Mutex::new(CONFIG.lock().unwrap().esp.default));
    pub static ref TOGGLE_CHANGED: Arc<Mutex<Instant>> = Arc::new(Mutex::new(Instant::now()));
    pub static ref RADAR_RESET_POSITION: Arc<Mutex<Option<[f32; 2]>>> = Arc::new(Mutex::new(None));
}

pub fn get_radar_toggled(config: Config) -> bool {
    let feature = config.radar;
    let mut toggled = FEATURE_TOGGLED.lock().unwrap();
    let mut changed = TOGGLE_CHANGED.lock().unwrap();

    return is_feature_toggled(feature.key, feature.mode, &mut toggled, &mut changed);
}

pub fn revolve_coordinates_system(revolve_angle: f32, origin_pos: Vector2<f32>, dest_pos: Vector2<f32>) -> Vector2<f32> {
    let mut result_pos: Vector2<f32> = Vector2 { x: 0.0, y: 0.0 };

    if revolve_angle == 0.0 {
        return dest_pos;
    }

    result_pos.x = origin_pos.x + (dest_pos.x - origin_pos.x) * (revolve_angle * PI / 180.0).cos() + (dest_pos.y - origin_pos.y) * (revolve_angle * PI / 180.0).sin();
    result_pos.y = origin_pos.y - (dest_pos.x - origin_pos.x) * (revolve_angle * PI / 180.0).sin() + (dest_pos.y - origin_pos.y) * (revolve_angle * PI / 180.0).cos();

    return result_pos;
}

pub fn render_radar(ui: &mut Ui, config: Config, local_pos: Vector3<f32>, local_yaw: f32, points: Vec<(Vector3<f32>, f32, bool, bool)>) {
    let mut reset_position = RADAR_RESET_POSITION.lock().unwrap();
    let (window_position, condition) = if let Some(position) = *reset_position {
        *reset_position = None;
        (position, imgui::Condition::Always)
    } else {
        (config.window_positions.radar, imgui::Condition::Once)
    };

    drop(reset_position);

    ui.window("Radar")
        .collapsible(false)
        .resizable(false)
        .bg_alpha(0.0)
        .size([config.radar.range as f32 * 2.0, config.radar.range as f32 * 2.0], imgui::Condition::Always)
        .position(window_position, condition)
        .build(|| {
            (*CONFIG.lock().unwrap()).window_positions.radar = ui.window_pos();

            let (full_window_pos, full_window_size) = (ui.window_pos(), ui.window_size());
            let window_pos = Vector2 { x: full_window_pos[0] + config.radar.range as f32, y: full_window_pos[1] + config.radar.range as f32 };

            // Window Mask
            ui.get_window_draw_list().add_rect(Vector2 { x: full_window_pos[0], y: full_window_pos[1] }, Vector2 { x: full_window_pos[0] + full_window_size[0], y: full_window_pos[1] + full_window_size[1] }, ImColor32::from_rgba_f32s(0.0, 0.0, 0.0, config.radar.alpha)).filled(true).build();
            
            // Cross Line
            if config.radar.crossline_enabled {
                let vertical1 = Vector2 { x: window_pos.x - config.radar.range as f32, y: window_pos.y };
                let vertical2 = Vector2 { x: window_pos.x + config.radar.range as f32, y: window_pos.y };

                let horizontal1 = Vector2 { x: window_pos.x, y: window_pos.y - config.radar.range as f32 };
                let horizontal2 = Vector2 { x: window_pos.x, y: window_pos.y + config.radar.range as f32 };

                let color = color_u32_to_f32(config.radar.crossline_color);
                
                ui.get_window_draw_list().add_line(vertical1, vertical2, color).build();
                ui.get_window_draw_list().add_line(horizontal1, horizontal2, color).build();
            }

            // Points
            let circle_size = 7.0 * config.radar.point_size;
            let arrow_size = 11.0 * config.radar.point_size;
            let arc_arrow_size = 7.0 * config.radar.point_size;

            for (pos, yaw, is_visible, is_friendly) in points {
                let distance = ((local_pos.x - pos.x).powf(2.0) + (local_pos.y - pos.y).powf(2.0)).sqrt() / (config.radar.proportion * 100) as f32 * config.radar.range as f32 * 2.0;
                let angle = (local_yaw - (pos.y - local_pos.y).atan2(pos.x - local_pos.x) * 180.0 / PI) * PI / 180.0;
                let point_pos = Vector2 { x: window_pos.x + distance * angle.sin(), y: window_pos.y - distance * angle.cos() };

                if point_pos.x < window_pos.x - config.radar.range as f32 || point_pos.x > window_pos.x + config.radar.range as f32 || point_pos.y > window_pos.y + config.radar.range as f32 || point_pos.y < window_pos.y - config.radar.range as f32 {
                    continue;
                }

                let radar_color = if is_friendly && config.radar.friendly_enabled { config.radar.friendly_color } else { if is_visible && config.radar.target_enabled { config.radar.target_color } else { config.radar.color } };

                if config.radar.style == 0 {
                    ui.get_window_draw_list().add_circle(point_pos, circle_size, color_u32_to_f32(radar_color)).filled(true).build();
                    
                    if config.radar.outline {
                        ui.get_window_draw_list().add_circle(point_pos, circle_size, color_with_masked_alpha(radar_color, 0xFF000000)).thickness(0.1).build();
                    }
                } else if config.radar.style == 1 {
                    let angle2 = (local_yaw - yaw) + 180.0;
                    let re_point = revolve_coordinates_system(angle2, window_pos, point_pos);
                    
                    let a = revolve_coordinates_system(-angle2, window_pos, Vector2 { x: re_point.x, y: re_point.y + arrow_size });
                    let b = revolve_coordinates_system(-angle2, window_pos, Vector2 { x: re_point.x - arrow_size / 1.5, y: re_point.y - arrow_size / 2.0 });
                    let c = revolve_coordinates_system(-angle2, window_pos, Vector2 { x: re_point.x + arrow_size / 1.5, y: re_point.y - arrow_size / 2.0 });

                    ui.get_window_draw_list().add_polyline(vec![a, b, point_pos, c], color_u32_to_f32(radar_color)).filled(true).build();
                    
                    if config.radar.outline {
                        ui.get_window_draw_list().add_polyline(vec![a, b, point_pos, c], color_with_masked_alpha(radar_color, 0xFF000000)).thickness(0.1).build();
                    }
                } else {
                    let angle2 = (local_yaw - yaw) - 90.0;
                    let triangle_point = Vector2 { x: point_pos.x + (arc_arrow_size + arc_arrow_size / 3.0) * (-angle2 * PI / 180.0).cos(), y: point_pos.y - (arc_arrow_size + arc_arrow_size / 3.0) * (-angle2 * PI / 180.0).sin() };
                    let triangle_point_2 = Vector2 { x: point_pos.x + arc_arrow_size * (-(angle2 - 30.0) * PI / 180.0).cos(), y: point_pos.y - arc_arrow_size * (-(angle2 - 30.0) * PI / 180.0).sin() };
                    let triangle_point_3 = Vector2 { x: point_pos.x + arc_arrow_size * (-(angle2 + 30.0) * PI / 180.0).cos(), y: point_pos.y - arc_arrow_size * (-(angle2 + 30.0) * PI / 180.0).sin() };

                    ui.get_window_draw_list().add_circle(point_pos, 0.85 * arc_arrow_size, color_u32_to_f32(radar_color)).thickness(30.0).filled(true).build();
                    
                    if config.radar.outline {
                        ui.get_window_draw_list().add_circle(point_pos, 0.95 * arc_arrow_size, color_with_masked_alpha(radar_color, 0xFF000000)).thickness(0.1).build();
                    }
                    
                    ui.get_window_draw_list().add_polyline(vec![triangle_point, triangle_point_2, triangle_point_3], ImColor32::from_rgba(220, 220, 220, 255)).filled(true).build();
                }
            }
        });
}