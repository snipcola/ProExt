use std::f32::consts::PI;
use imgui::Ui;
use mint::Vector2;

use crate::utils::config::Config;
use crate::ui::functions::{color_u32_to_f32, color_with_masked_alpha, rectangle};

pub fn render_headshot_line(ui: &mut Ui, window_width: i32, window_height: i32, fov: i32, view_angle_x: f32, config: Config) {
    let pos = Vector2 { x: window_width as f32 / 2.0, y: window_height as f32 / 2.0 - window_height as f32 / (2.0 * (fov as f32 * PI / 180.0).sin() / (90.0 * PI / 180.0).sin()) * (view_angle_x as f32 * PI / 180.0).sin() / (90.0 * PI / 180.0).sin() };

    rectangle(ui, Vector2 { x: pos.x - 21.0, y: pos.y - 1.0 }, Vector2 { x: 17.0, y: 3.0 }, color_with_masked_alpha(config.misc.headshot_line_color, 0xFF000000).into(), 1.0, 0, true);
    rectangle(ui, Vector2 { x: pos.x - 20.0, y: pos.y }, Vector2 { x: 17.0, y: 3.0 }, color_u32_to_f32(config.misc.headshot_line_color).into(), 1.0, 0, true);

    rectangle(ui, Vector2 { x: pos.x + 5.0, y: pos.y - 1.0 }, Vector2 { x: 17.0, y: 3.0 }, color_with_masked_alpha(config.misc.headshot_line_color, 0xFF000000).into(), 1.0, 0, true);
    rectangle(ui, Vector2 { x: pos.x + 6.0, y: pos.y }, Vector2 { x: 17.0, y: 3.0 }, color_u32_to_f32(config.misc.headshot_line_color).into(), 1.0, 0, true);
}