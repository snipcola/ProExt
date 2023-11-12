use std::f32::consts::PI;

use imgui::Ui;
use mint::Vector2;

use crate::{ui::main::{color_u32_to_f32, color_with_masked_alpha, rectangle}, utils::config::Config};

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