// Copyright (c) 2023 Vytrol <vytrol@proton.me>
// SPDX-License-Identifier: MIT

use std::f32::consts::PI;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use lazy_static::lazy_static;

use imgui::{Ui, ImColor32};
use mint::{Vector3, Vector2, Vector4};

use crate::utils::cheat::config::{Config, CONFIG};
use crate::cheat::functions::{is_feature_toggled, calculate_distance};

use crate::cheat::classes::bone::{BoneJointPos, bone_joint_list, BoneIndex};
use crate::cheat::classes::view::View;

use crate::ui::functions::{color_u32_to_f32, color_with_masked_alpha, rectangle, stroke_text, mix_colors, color_with_alpha, text, rectangle_gradient};

lazy_static! {
    pub static ref FEATURE_TOGGLED: Arc<Mutex<bool>> = Arc::new(Mutex::new(CONFIG.lock().unwrap().esp.default));
    pub static ref TOGGLE_CHANGED: Arc<Mutex<Instant>> = Arc::new(Mutex::new(Instant::now()));
}

pub fn get_esp_toggled(config: Config) -> bool {
    let feature = config.esp;
    let mut toggled = FEATURE_TOGGLED.lock().unwrap();
    let mut changed = TOGGLE_CHANGED.lock().unwrap();

    return is_feature_toggled(feature.key, feature.mode, &mut toggled, &mut changed);
}

pub fn render_bones(ui: &mut Ui, bone_pos_list: [BoneJointPos; 30], config: Config) {
    let mut previous: BoneJointPos = BoneJointPos { pos: Vector3 { x: 0.0, y: 0.0, z: 0.0 }, screen_pos: Vector2 { x: 0.0, y: 0.0 }, is_visible: false };

    for bone_joint in bone_joint_list::LIST {
        previous.pos = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
        
        for joint in bone_joint {
            let current = bone_pos_list[joint as usize];

            if (previous.pos == Vector3 { x: 0.0, y: 0.0, z: 0.0 }) {
                previous = current;
                continue;
            }

            if previous.is_visible && current.is_visible {
                if config.esp.outline {
                    ui.get_background_draw_list().add_line(previous.screen_pos, current.screen_pos, color_with_masked_alpha(config.esp.skeleton_color, 0xFF000000)).thickness(config.esp.thickness + 1.0).build();
                }

                ui.get_background_draw_list().add_line(previous.screen_pos, current.screen_pos, color_u32_to_f32(config.esp.skeleton_color)).thickness(config.esp.thickness).build();
            }

            previous = current;
        }
    }
}

pub fn render_head(ui: &mut Ui, bone_pos_list: [BoneJointPos; 30], config: Config) {
    let head = bone_pos_list[BoneIndex::Head as usize];
    let neck = bone_pos_list[BoneIndex::Neck0 as usize];
    
    let center_pos = head.screen_pos;
    let radius = (head.screen_pos.y - neck.screen_pos.y).abs() + 2.0;

    if radius > 200.0 {
        return;
    }

    if config.esp.outline {
        ui.get_background_draw_list().add_circle(center_pos, radius, color_with_masked_alpha(config.esp.head_color, 0xFF000000)).thickness(config.esp.thickness + 1.0).build();
    }

    if config.esp.head_mode == 0 {
        ui.get_background_draw_list().add_circle(center_pos, radius, color_u32_to_f32(config.esp.head_color)).thickness(config.esp.thickness).build();
    } else {
        ui.get_background_draw_list().add_circle(center_pos, radius, color_u32_to_f32(config.esp.head_color)).thickness(config.esp.thickness).filled(true).build();
    }
}

pub fn render_eye_ray(ui: &mut Ui, bone_pos_list: [BoneJointPos; 30], view_angle: Vector2<f32>, config: Config, view: View, window_info: ((i32, i32), (i32, i32))) {
    let line_length = (view_angle.x * PI / 180.0).cos() * 30.0;
    let head = bone_pos_list[BoneIndex::Head as usize];
    let mut pos = Vector2 { x: 0.0, y: 0.0 };

    if !view.world_to_screen(Vector3 { x: head.pos.x + (view_angle.y * PI / 180.0).cos() * line_length, y: head.pos.y + (view_angle.y * PI / 180.0).sin() * line_length, z: head.pos.z - (view_angle.x * PI / 180.0).sin() * 30.0 }, &mut pos, window_info) {
        return;
    }

    if config.esp.outline {
        ui.get_background_draw_list().add_line(head.screen_pos, pos, color_with_masked_alpha(config.esp.eye_ray_color, 0xFF000000)).thickness(config.esp.thickness + 1.0).build();
    }

    ui.get_background_draw_list().add_line(head.screen_pos, pos, color_u32_to_f32(config.esp.eye_ray_color)).thickness(config.esp.thickness).build();
}

fn calculate_size(initial: Vector2<f32>, distance: f32, max_distance: f32) -> Vector2<f32> {
    let scale = 1.0 - (distance / max_distance);
    return Vector2 { x: initial.x * scale, y: initial.y * scale };
}

pub fn get_2d_box_non_player(size: Vector2<f32>, screen_pos: Vector2<f32>, distance: f32) -> Vector4<f32> {
    let size = calculate_size(size, distance, 300.0);
    let pos = Vector2 { x: screen_pos.x - size.x / 2.0, y: screen_pos.y - size.y / 2.0 };
    return Vector4 { x: pos.x, y: pos.y, z: size.x, w: size.y };
}

pub fn get_2d_box(bone_pos_list: [BoneJointPos; 30], screen_pos: Vector2<f32>) -> Vector4<f32> {
    let head = bone_pos_list[BoneIndex::Head as usize];
    let size = Vector2 { x: ((screen_pos.y - head.screen_pos.y) * 1.09) * 0.6, y: (screen_pos.y - head.screen_pos.y) * 1.09 };
    let pos = Vector2 { x: screen_pos.x - size.x / 2.0, y: head.screen_pos.y - size.y * 0.08 };

    return Vector4 { x: pos.x, y: pos.y, z: size.x, w: size.y };
}

pub fn get_2d_bone_rect(bone_pos_list: [BoneJointPos; 30]) -> Vector4<f32> {
    let mut min = bone_pos_list[0].screen_pos;
    let mut max = bone_pos_list[0].screen_pos;

    for joint in bone_pos_list {
        if !joint.is_visible {
            continue;
        }

        min.x = joint.screen_pos.x.min(min.x);
        min.y = joint.screen_pos.y.min(min.y);

        max.x = joint.screen_pos.x.max(max.x);
        max.y = joint.screen_pos.y.max(max.y);
    }

    return Vector4 { x: min.x, y: min.y, z: (max.x - min.x), w: (max.y - min.y) };
}

pub fn render_snap_line(ui: &mut Ui, rect: Vector4<f32>, config: Config, window_width: i32, window_height: i32) {
    let from_line = Vector2 { x: rect.x + rect.z / 2.0, y: rect.y };
    let to_line = {
        if config.esp.snap_line_mode == 0 {
            Vector2 { x: window_width as f32 / 2.0, y: 0.0 }
        } else if config.esp.snap_line_mode == 1 {
            Vector2 { x: window_width as f32 / 2.0, y: window_height as f32 / 2.0 }
        } else {
            Vector2 { x: window_width as f32 / 2.0, y: window_height as f32 }
        }
    };

    if config.esp.outline {
        ui.get_background_draw_list().add_line(from_line, to_line, color_with_masked_alpha(config.esp.snap_line_color, 0xFF000000)).thickness(config.esp.thickness + 1.0).build();
    }

    ui.get_background_draw_list().add_line(from_line, to_line, color_u32_to_f32(config.esp.snap_line_color)).thickness(config.esp.thickness).build();
}

pub fn render_box_bomb(ui: &mut Ui, rect: Vector4<f32>, config: Config) {
    if config.esp.outline {
        rectangle(ui, Vector2 { x: rect.x, y: rect.y }, Vector2 { x: rect.z, y: rect.w }, color_with_masked_alpha(config.esp.bomb_color, 0xFF000000).into(), config.esp.thickness + 1.0, config.esp.rounding, false);
    }
    
    rectangle(ui, Vector2 { x: rect.x, y: rect.y }, Vector2 { x: rect.z, y: rect.w }, color_u32_to_f32(config.esp.bomb_color).into(), config.esp.thickness, config.esp.rounding, false);

    if config.esp.filled_bomb_enabled {
        rectangle(ui, Vector2 { x: rect.x, y: rect.y }, Vector2 { x: rect.z, y: rect.w }, color_with_alpha(config.esp.filled_bomb_color, config.esp.filled_bomb_alpha).into(), config.esp.thickness - 0.3, config.esp.rounding, true);
    }
}

pub fn render_box(ui: &mut Ui, rect: Vector4<f32>, enemy_visible: bool, is_friendly: bool, config: Config) {
    if config.esp.outline {
        rectangle(ui, Vector2 { x: rect.x, y: rect.y }, Vector2 { x: rect.z, y: rect.w }, color_with_masked_alpha(config.esp.box_color, 0xFF000000).into(), config.esp.thickness + 1.0, config.esp.rounding, false);
    }
    
    let box_color = if is_friendly && config.esp.box_friendly_enabled { config.esp.box_friendly_color } else { if config.esp.box_target_enabled && enemy_visible { config.esp.box_target_color } else { config.esp.box_color } };
    rectangle(ui, Vector2 { x: rect.x, y: rect.y }, Vector2 { x: rect.z, y: rect.w }, color_u32_to_f32(box_color).into(), config.esp.thickness, config.esp.rounding, false);

    if config.esp.filled_box_enabled {
        let filled_box_color_one = if is_friendly && config.esp.box_friendly_enabled { color_with_alpha(config.esp.box_friendly_color, config.esp.filled_box_alpha) } else { if config.esp.box_target_enabled && enemy_visible { color_with_alpha(config.esp.box_target_color, config.esp.filled_box_alpha) } else { color_with_alpha(config.esp.filled_box_color_one, config.esp.filled_box_alpha) } };
        let filled_box_color_two = if is_friendly && config.esp.box_friendly_enabled { color_with_alpha(config.esp.box_friendly_color, config.esp.filled_box_alpha) } else { if config.esp.box_target_enabled && enemy_visible { color_with_alpha(config.esp.box_target_color, config.esp.filled_box_alpha) } else { color_with_alpha(config.esp.filled_box_color_two, config.esp.filled_box_alpha) } };

        rectangle_gradient(ui, Vector2 { x: rect.x, y: rect.y }, Vector2 { x: rect.z, y: rect.w }, filled_box_color_one.into(), filled_box_color_two.into(), config.esp.thickness - 0.3, config.esp.rounding, true);
    }
}

pub fn render_weapon_name(ui: &mut Ui, weapon_name: &str, rect: Vector4<f32>, config: Config) {
    let mut y_offset: f32 = 0.0;

    if config.esp.bar_mode == 1 {
        if config.esp.ammo_bar_enabled {
            y_offset += 6.0;
        }
    }
    
    if config.esp.outline {
        stroke_text(ui, weapon_name.to_string(), Vector2 { x: rect.x + rect.z / 2.0, y: rect.y + rect.w + y_offset + 2.0 }, color_u32_to_f32(config.esp.weapon_name_color).into(), true);
    } else {
        text(ui, weapon_name.to_string(), Vector2 { x: rect.x + rect.z / 2.0, y: rect.y + rect.w + y_offset + 2.0 }, color_u32_to_f32(config.esp.weapon_name_color).into(), true);
    }
}

pub fn render_distance(ui: &mut Ui, pawn_pos: Vector3<f32>, local_pawn_pos: Vector3<f32>, rect: Vector4<f32>, config: Config) {
    let distance = calculate_distance(pawn_pos, local_pawn_pos);
    let mut x_offset: f32 = 0.0;

    if config.esp.bar_mode == 0 {
        if config.esp.ammo_bar_enabled {
            x_offset += 6.0;
        }
    }

    if config.esp.outline {
        stroke_text(ui, format!("{}m", distance), Vector2 { x: rect.x + rect.z + x_offset + 4.0, y: rect.y }, color_u32_to_f32(config.esp.distance_color).into(), false);
    } else {
        text(ui, format!("{}m", distance), Vector2 { x: rect.x + rect.z + x_offset + 4.0, y: rect.y }, color_u32_to_f32(config.esp.distance_color).into(), false);
    }
}

pub fn render_bomb_name(ui: &mut Ui, name: &str, rect: Vector4<f32>, config: Config) {
    if config.esp.outline {
        stroke_text(ui, name.to_string(), Vector2 { x: rect.x + rect.z / 2.0, y: rect.y - 18.0 }, color_u32_to_f32(config.esp.bomb_color).into(), true);
    } else {
        text(ui, name.to_string(), Vector2 { x: rect.x + rect.z / 2.0, y: rect.y - 18.0 }, color_u32_to_f32(config.esp.bomb_color).into(), true);
    }
}

pub fn render_name(ui: &mut Ui, name: &str, rect: Vector4<f32>, config: Config) {
    let mut y_offset: f32 = 0.0;

    if config.esp.bar_mode == 1 {
        if config.esp.health_bar_enabled {
            y_offset += 6.0;
        }

        if config.esp.armor_bar_enabled {
            y_offset += 6.0;
        }
    }

    if config.esp.outline {
        stroke_text(ui, name.to_string(), Vector2 { x: rect.x + rect.z / 2.0, y: rect.y - y_offset - 14.0 }, color_u32_to_f32(config.esp.name_color).into(), true);
    } else {
        text(ui, name.to_string(), Vector2 { x: rect.x + rect.z / 2.0, y: rect.y - y_offset - 14.0 }, color_u32_to_f32(config.esp.name_color).into(), true);
    }
}

pub fn render_health_bar(ui: &mut Ui, current_health: f32, rect: Vector4<f32>, config: Config) {
    let height_horizontal = 10.0 - (rect.z / 100.0).max(6.0).min(8.0);
    let (rect_pos, rect_size) = {
        if config.esp.bar_mode == 0 {
            // Vertical
            (Vector2 { x: rect.x - (height_horizontal * 1.1), y: rect.y }, Vector2 { x: height_horizontal, y: rect.w })
        } else {
            // Horizontal
            (Vector2 { x: rect.x, y: rect.y - (height_horizontal * 1.1) }, Vector2 { x: rect.z, y: height_horizontal })
        }
    };
    
    let background_color = ImColor32::from_rgba(90, 90, 90, 220);
    let frame_color = ImColor32::from_rgba(45, 45, 45, 220);

    let first_stage_color = ImColor32::from(color_u32_to_f32(config.esp.health_bar_first_color));
    let second_stage_color = ImColor32::from(color_u32_to_f32(config.esp.health_bar_second_color));
    let third_stage_color = ImColor32::from(color_u32_to_f32(config.esp.health_bar_third_color));

    let in_range = |value: f32, min: f32, max: f32| -> bool {
        value > min && value <= max
    };

    let max_health = 100.0;
    let proportion = current_health / max_health;
    let (height, width) = ((rect_size.y * proportion), (rect_size.x * proportion));

    let color = {
        if in_range(proportion, 0.5, 1.0) {
            mix_colors(first_stage_color, second_stage_color, proportion.powf(2.5) * 3.0 - 1.0)
        } else {
            mix_colors(second_stage_color, third_stage_color, proportion.powf(2.5) * 4.0)
        }
    };
    
    rectangle(ui, rect_pos, rect_size, background_color, config.esp.thickness * 0.4, config.esp.rounding, true);
    
    if config.esp.bar_mode == 0 {
        // Vertical
        ui.get_background_draw_list().add_rect(Vector2 { x: rect_pos.x, y: rect_pos.y + rect_size.y - height }, Vector2 { x: rect_pos.x + rect_size.x, y: rect_pos.y + rect_size.y }, color).filled(true).rounding(config.esp.rounding as f32).build();
    } else {
        // Horizontal
        ui.get_background_draw_list().add_rect(rect_pos, Vector2 { x: rect_pos.x + width, y: rect_pos.y + rect_size.y }, color).filled(true).rounding(config.esp.rounding as f32).build();
    }

    if config.esp.outline {
        rectangle(ui, rect_pos, rect_size, frame_color, config.esp.thickness * 0.4, config.esp.rounding, false);
    }
}

pub fn render_armor_bar(ui: &mut Ui, armor: f32, rect: Vector4<f32>, config: Config) {
    let height_horizontal = 10.0 - (rect.z / 100.0).max(6.0).min(8.0);
    let mut multiply_factor = 1.1;

    if config.esp.health_bar_enabled {
        multiply_factor += 1.1;
    }

    let (rect_pos, rect_size) = {
        if config.esp.bar_mode == 0 {
            // Vertical
            (Vector2 { x: rect.x - (height_horizontal * multiply_factor), y: rect.y }, Vector2 { x: height_horizontal, y: rect.w })
        } else {
            // Horizontal
            (Vector2 { x: rect.x, y: rect.y - (height_horizontal * multiply_factor) }, Vector2 { x: rect.z, y: height_horizontal })
        }
    };

    let background_color = ImColor32::from_rgba(90, 90, 90, 220);
    let frame_color = ImColor32::from_rgba(45, 45, 45, 220);
    let color = ImColor32::from(color_u32_to_f32(config.esp.armor_bar_color));

    let max_armor = 100.0;
    let proportion = armor / max_armor;
    let (height, width) = ((rect_size.y * proportion), (rect_size.x * proportion));

    rectangle(ui, rect_pos, rect_size, background_color, config.esp.thickness * 0.4, config.esp.rounding, true);
    
    if armor > 0.0 {
        if config.esp.bar_mode == 0 {
            // Vertical
            ui.get_background_draw_list().add_rect(Vector2 { x: rect_pos.x, y: rect_pos.y + rect_size.y - height }, Vector2 { x: rect_pos.x + rect_size.x, y: rect_pos.y + rect_size.y }, color).filled(true).rounding(config.esp.rounding as f32).build();
        } else {
            // Horizontal
            ui.get_background_draw_list().add_rect(rect_pos, Vector2 { x: rect_pos.x + width, y: rect_pos.y + rect_size.y }, color).filled(true).rounding(config.esp.rounding as f32).build();
        }
    }

    if config.esp.outline {
        rectangle(ui, rect_pos, rect_size, frame_color, config.esp.thickness * 0.4, config.esp.rounding, false);
    }
}

pub fn render_ammo_bar(ui: &mut Ui, ammo: f32, max_ammo: f32, rect: Vector4<f32>, config: Config) {
    let height_horizontal = 10.0 - (rect.z / 100.0).max(6.0).min(8.0);
    let (rect_pos, rect_size) = {
        if config.esp.bar_mode == 0 {
            // Vertical
            (Vector2 { x: rect.x + rect.z + (height_horizontal * 0.1), y: rect.y }, Vector2 { x: height_horizontal, y: rect.w })
        } else {
            // Horizontal
            (Vector2 { x: rect.x, y: rect.y + rect.w + (height_horizontal * 0.1) }, Vector2 { x: rect.z, y: height_horizontal })
        }
    };
    
    let background_color = ImColor32::from_rgba(90, 90, 90, 220);
    let frame_color = ImColor32::from_rgba(45, 45, 45, 220);
    let color = ImColor32::from(color_u32_to_f32(config.esp.ammo_bar_color));

    let proportion = ammo / max_ammo;
    let (height, width) = ((rect_size.y * proportion), (rect_size.x * proportion));
    
    rectangle(ui, rect_pos, rect_size, background_color, config.esp.thickness * 0.4, config.esp.rounding, true);
    
    if config.esp.bar_mode == 0 {
        // Vertical
        ui.get_background_draw_list().add_rect(Vector2 { x: rect_pos.x, y: rect_pos.y + rect_size.y - height }, Vector2 { x: rect_pos.x + rect_size.x, y: rect_pos.y + rect_size.y }, color).filled(true).rounding(config.esp.rounding as f32).build();
    } else {
        // Horizontal
        ui.get_background_draw_list().add_rect(rect_pos, Vector2 { x: rect_pos.x + width, y: rect_pos.y + rect_size.y }, color).filled(true).rounding(config.esp.rounding as f32).build();
    }

    if config.esp.outline {
        rectangle(ui, rect_pos, rect_size, frame_color, config.esp.thickness * 0.4, config.esp.rounding, false);
    }
}

pub fn render_bomb(ui: &mut Ui, pos: Vector3<f32>, local_pawn_pos: Vector3<f32>, screen_pos: Vector2<f32>, bomb_site: &str, config: Config) {
    let distance = calculate_distance(pos, local_pawn_pos);
    let rect = get_2d_box_non_player(Vector2 { x: 20.0, y: 20.0 }, screen_pos, distance as f32);

    render_box_bomb(ui, rect, config);

    if config.esp.name_enabled {
        render_bomb_name(ui, &format!("Bomb ({})", bomb_site), rect, config);
    }

    if config.esp.distance_enabled {
        render_distance(ui, pos, local_pawn_pos, rect, config);
    }
}

pub fn render_headshot_line(ui: &mut Ui, window_width: i32, window_height: i32, fov: i32, view_angle_x: f32, config: Config) {
    let pos = Vector2 { x: window_width as f32 / 2.0, y: window_height as f32 / 2.0 - window_height as f32 / (2.0 * (fov as f32 * PI / 180.0).sin() / (90.0 * PI / 180.0).sin()) * (view_angle_x as f32 * PI / 180.0).sin() / (90.0 * PI / 180.0).sin() };

    rectangle(ui, Vector2 { x: pos.x - 21.0, y: pos.y - 1.0 }, Vector2 { x: 17.0, y: 3.0 }, color_with_masked_alpha(config.esp.headshot_line_color, 0xFF000000).into(), 1.0, 0, true);
    rectangle(ui, Vector2 { x: pos.x - 20.0, y: pos.y }, Vector2 { x: 17.0, y: 3.0 }, color_u32_to_f32(config.esp.headshot_line_color).into(), 1.0, 0, true);

    rectangle(ui, Vector2 { x: pos.x + 5.0, y: pos.y - 1.0 }, Vector2 { x: 17.0, y: 3.0 }, color_with_masked_alpha(config.esp.headshot_line_color, 0xFF000000).into(), 1.0, 0, true);
    rectangle(ui, Vector2 { x: pos.x + 6.0, y: pos.y }, Vector2 { x: 17.0, y: 3.0 }, color_u32_to_f32(config.esp.headshot_line_color).into(), 1.0, 0, true);
}