use std::f32::consts::PI;

use imgui::Ui;
use mint::Vector2;

use crate::utils::config::CONFIG;
use crate::ui::main::{color_u32_to_f32, WINDOW_INFO};

pub fn revolve_coordinates_system(revolve_angle: f32, origin_pos: Vector2<f32>, dest_pos: Vector2<f32>) -> Vector2<f32> {
    let mut result_pos: Vector2<f32> = Vector2 { x: 0.0, y: 0.0 };

    if revolve_angle == 0.0 {
        return dest_pos;
    }

    result_pos.x = origin_pos.x + (dest_pos.x - origin_pos.x) * (revolve_angle * PI / 180.0).cos() + (dest_pos.y - origin_pos.y) * (revolve_angle * PI / 180.0).sin();
    result_pos.y = origin_pos.y - (dest_pos.x - origin_pos.x) * (revolve_angle * PI / 180.0).sin() + (dest_pos.y - origin_pos.y) * (revolve_angle * PI / 180.0).cos();

    return result_pos;
}

pub fn render_radar(ui: &mut Ui) {
    let config = *CONFIG.lock().unwrap();
    let window_info = (*WINDOW_INFO.lock().unwrap()).unwrap_or(((0, 0), (0, 0)));
    let window_scale_factor = window_info.1.0 as f32 / 970.0;
    let radar_range = config.radar_range * window_scale_factor;
    let open = config.show_radar;

    if open {
        ui.window("Radar")
            .resizable(false)
            .collapsible(false)
            .scroll_bar(false)
            .size([radar_range, radar_range], imgui::Condition::Always)
            .position([0.0, 0.0], imgui::Condition::FirstUseEver)
            .bg_alpha(0.1)
            .build(|| {
                // Cross Line
                if config.show_radar_cross_line {
                    let vertical1 = Vector2 { x: ui.window_pos()[0] + (radar_range / 2.0), y: ui.window_pos()[1] };
                    let vertical2 = Vector2 { x: ui.window_pos()[0] + (radar_range / 2.0), y: ui.window_pos()[1] + radar_range };

                    let horizontal1 = Vector2 { x: ui.window_pos()[0], y: ui.window_pos()[1] + (radar_range / 2.0) };
                    let horizontal2 = Vector2 { x: ui.window_pos()[0] + radar_range, y: ui.window_pos()[1] + (radar_range / 2.0) };

                    let color = color_u32_to_f32(config.radar_cross_line_color);
                    
                    ui.get_window_draw_list().add_line(vertical1, vertical2, color).build();
                    ui.get_window_draw_list().add_line(horizontal1, horizontal2, color).build();
                }
            });
    }
}