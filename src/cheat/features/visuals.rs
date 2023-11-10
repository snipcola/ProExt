use std::f32::consts::PI;

use imgui::Ui;
use mint::Vector2;

use crate::{ui::main::color_u32_to_f32, utils::config::Config};

pub fn render_fov_circle(ui: &mut Ui, window_width: i32, window_height: i32, fov: i32, config: Config) {
    let center_point: Vector2<f32> = Vector2 { x: window_width as f32 / 2.0, y: window_height as f32 / 2.0 };
    let radius = (config.aim_fov / 180.0 * PI / 2.0).tan() / (fov as f32 / 180.0 * PI / 2.0).tan() * window_width as f32;

    ui.get_background_draw_list().add_circle(center_point, radius, color_u32_to_f32(config.aim_fov_range_color)).build();
}

pub fn render_fov(ui: &mut Ui, window_width: i32, window_height: i32, fov: i32, config: Config) {
    let mut line_end_point: [Vector2<f32>; 2] = [Vector2 { x: 0.0, y: 0.0 }, Vector2 { x: 0.0, y: 0.0 }];
    let pos: Vector2<f32> = Vector2 { x: window_width as f32 / 2.0, y: window_height as f32 / 2.0 };
    let radian = (fov as f32 / 2.0).to_radians();
    let color = color_u32_to_f32(config.fov_line_color);

    line_end_point[0].y = pos.y - config.fov_line_size;
    line_end_point[1].y = line_end_point[0].y;
    
    let length = config.fov_line_size * radian.tan();

    line_end_point[0].x = pos.x - length;
    line_end_point[1].x = pos.x + length;

    ui.get_background_draw_list().add_line(pos, line_end_point[0], color).thickness(1.5).build();
    ui.get_background_draw_list().add_line(pos, line_end_point[1], color).thickness(1.5).build();
}

pub fn render_crosshair(ui: &mut Ui, window_width: i32, window_height: i32, config: Config) {
    let sight_pos: Vector2<f32> = Vector2 { x: window_width as f32 / 2.0, y: window_height as f32 / 2.0 };
    let color = color_u32_to_f32(config.crosshair_color);

    let line1_first = Vector2 { x: sight_pos.x - config.crosshair_size, y: sight_pos.y };
    let line1_second = Vector2 { x: sight_pos.x + config.crosshair_size, y: sight_pos.y };

    let line2_first = Vector2 { x: sight_pos.x, y: sight_pos.y - config.crosshair_size };
    let line2_second = Vector2 { x: sight_pos.x, y: sight_pos.y + config.crosshair_size };

    ui.get_background_draw_list().add_line(line1_first, line1_second, color).build();
    ui.get_background_draw_list().add_line(line2_first, line2_second, color).build();
}