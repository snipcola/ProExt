use std::f32::consts::PI;

use imgui::{Ui, ImColor32};
use mint::Vector2;

use crate::{ui::main::{color_u32_to_f32, color_with_masked_alpha, rectangle, is_enemy_at_crosshair}, utils::config::Config, cheat::classes::view::View};

pub fn render_fov_circle(ui: &mut Ui, window_width: i32, window_height: i32, fov: i32, config: Config) {
    let center_point: Vector2<f32> = Vector2 { x: window_width as f32 / 2.0, y: window_height as f32 / 2.0 };
    let radius = (config.aim_fov / 180.0 * PI / 2.0).tan() / (fov as f32 / 180.0 * PI / 2.0).tan() * window_width as f32;

    ui.get_background_draw_list().add_circle(center_point, radius, color_u32_to_f32(config.aim_fov_range_color)).build();
}

pub fn render_head_shot_line(ui: &mut Ui, window_width: i32, window_height: i32, fov: i32, view_angle_x: f32, config: Config) {
    let pos = Vector2 { x: window_width as f32 / 2.0, y: window_height as f32 / 2.0 - window_height as f32 / (2.0 * f32::sin(fov as f32 * PI / 180.0) / f32::sin(90.0 * PI / 180.0)) * f32::sin(view_angle_x as f32 * PI / 180.0) / f32::sin(90.0 * PI / 180.0) };

    rectangle(ui, Vector2 { x: pos.x - 21.0, y: pos.y - 1.0 }, Vector2 { x: 17.0, y: 3.0 }, color_with_masked_alpha(config.head_shot_line_color, 0xFF000000).into(), 1.0, 0, true);
    rectangle(ui, Vector2 { x: pos.x - 20.0, y: pos.y }, Vector2 { x: 17.0, y: 3.0 }, color_u32_to_f32(config.head_shot_line_color).into(), 1.0, 0, true);

    rectangle(ui, Vector2 { x: pos.x + 5.0, y: pos.y - 1.0 }, Vector2 { x: 17.0, y: 3.0 }, color_with_masked_alpha(config.head_shot_line_color, 0xFF000000).into(), 1.0, 0, true);
    rectangle(ui, Vector2 { x: pos.x + 6.0, y: pos.y }, Vector2 { x: 17.0, y: 3.0 }, color_u32_to_f32(config.head_shot_line_color).into(), 1.0, 0, true);
}

pub fn render_crosshair(ui: &mut Ui, pos: Vector2<f32>, window_info: ((i32, i32), (i32, i32)), local_entity_pawn_address: u64, local_entity_pawn_team_id: i32, game_address_entity_list: u64, game_view: View, config: Config) {
    let (aiming_at_enemy, allow_shoot) = is_enemy_at_crosshair(window_info, local_entity_pawn_address, local_entity_pawn_team_id, game_address_entity_list, game_view, config);
    let color = {
        if aiming_at_enemy && allow_shoot {
            config.cross_hair_target_color
        } else {
            config.cross_hair_color
        }
    };

    let (border_width, dot_size, gap) = (2.0, 1.0 as f32, config.cross_hair_lines_space as f32 / 2.0);
    let (outline_gap, thickness) = (gap - 1.0, config.cross_hair_lines_thickness as f32);

    let offset_1 = Vector2 { x: config.cross_hair_dot_size as f32, y: config.cross_hair_dot_size as f32 };
    let offset_2 = Vector2 { x: offset_1.x + 1.0, y: offset_1.y + 1.0 };

    if config.cross_hair_outline {
        if config.cross_hair_dot {
            ui.get_background_draw_list().add_rect(Vector2 { x: pos.x - offset_1.x, y: pos.y - offset_1.y }, Vector2 { x: pos.x + offset_2.x, y: pos.y + offset_2.y }, ImColor32::from(color_with_masked_alpha(color, 0xFF000000))).filled(true).build();
        }

        if config.cross_hair_lines {
            ui.get_background_draw_list().add_rect(Vector2 { x: pos.x - (outline_gap + border_width + config.cross_hair_lines_width as f32), y: pos.y - thickness }, Vector2 { x: pos.x - outline_gap, y: pos.y + 1.0 + thickness }, ImColor32::from(color_with_masked_alpha(color, 0xFF000000))).filled(true).build();
            ui.get_background_draw_list().add_rect(Vector2 { x: pos.x + (outline_gap + dot_size), y: pos.y - thickness }, Vector2 { x: pos.x + (outline_gap + dot_size + border_width + config.cross_hair_lines_width as f32), y: pos.y + 1.0 + thickness }, ImColor32::from(color_with_masked_alpha(color, 0xFF000000))).filled(true).build();
            ui.get_background_draw_list().add_rect(Vector2 { x: pos.x - thickness, y: pos.y - (outline_gap + border_width + config.cross_hair_lines_height as f32) }, Vector2 { x: pos.x + 1.0 + thickness, y: pos.y - outline_gap }, ImColor32::from(color_with_masked_alpha(color, 0xFF000000))).filled(true).build();
            ui.get_background_draw_list().add_rect(Vector2 { x: pos.x - thickness, y: pos.y + outline_gap + dot_size }, Vector2 { x: pos.x + 1.0 + thickness, y: pos.y + (outline_gap + dot_size + border_width + config.cross_hair_lines_height as f32) }, ImColor32::from(color_with_masked_alpha(color, 0xFF000000))).filled(true).build();
        }

        if config.cross_hair_circle {
            ui.get_background_draw_list().add_circle(pos, config.cross_hair_circle_radius as f32, color_with_masked_alpha(color, 0xFF000000)).thickness(3.0).build();
        }
    }

    if config.cross_hair_dot {
        ui.get_background_draw_list().add_rect(Vector2 { x: pos.x - offset_1.x + dot_size, y: pos.y - offset_1.y + dot_size }, Vector2 { x: pos.x + offset_1.x, y: pos.y + offset_1.y }, color_u32_to_f32(color)).filled(true).build();
    }

    if config.cross_hair_lines {
        ui.get_background_draw_list().add_rect(Vector2 { x: pos.x - (gap + config.cross_hair_lines_width as f32), y: pos.y - thickness + 1.0 }, Vector2 { x: pos.x - gap, y: pos.y + thickness }, color_u32_to_f32(color)).filled(true).build();
        ui.get_background_draw_list().add_rect(Vector2 { x: pos.x + gap + dot_size, y: pos.y - thickness + 1.0 }, Vector2 { x: pos.x + (gap + dot_size + config.cross_hair_lines_width as f32), y: pos.y + thickness }, color_u32_to_f32(color)).filled(true).build();
        ui.get_background_draw_list().add_rect(Vector2 { x: pos.x - thickness + 1.0, y: pos.y - (gap + config.cross_hair_lines_height as f32) }, Vector2 { x: pos.x + thickness, y: pos.y - gap }, color_u32_to_f32(color)).filled(true).build();
        ui.get_background_draw_list().add_rect(Vector2 { x: pos.x - thickness + 1.0, y: pos.y + gap + dot_size }, Vector2 { x: pos.x + thickness, y: pos.y + (gap + dot_size + config.cross_hair_lines_height as f32) }, color_u32_to_f32(color)).filled(true).build();
    }

    if config.cross_hair_circle {
        ui.get_background_draw_list().add_circle(pos, config.cross_hair_circle_radius as f32, color_u32_to_f32(color)).thickness(1.0).build();
    }
}