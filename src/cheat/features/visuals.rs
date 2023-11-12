use std::f32::consts::PI;

use imgui::Ui;
use mint::Vector2;

use crate::{ui::main::color_u32_to_f32, utils::config::Config};

pub fn render_fov_circle(ui: &mut Ui, window_width: i32, window_height: i32, fov: i32, config: Config) {
    let center_point: Vector2<f32> = Vector2 { x: window_width as f32 / 2.0, y: window_height as f32 / 2.0 };
    let radius = (config.aim_fov / 180.0 * PI / 2.0).tan() / (fov as f32 / 180.0 * PI / 2.0).tan() * window_width as f32;

    ui.get_background_draw_list().add_circle(center_point, radius, color_u32_to_f32(config.aim_fov_range_color)).build();
}