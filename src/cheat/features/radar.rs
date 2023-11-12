use std::f32::consts::PI;

use imgui::{Ui, ImColor32};
use mint::{Vector2, Vector3};

use crate::utils::config::Config;
use crate::ui::main::color_u32_to_f32;

pub fn revolve_coordinates_system(revolve_angle: f32, origin_pos: Vector2<f32>, dest_pos: Vector2<f32>) -> Vector2<f32> {
    let mut result_pos: Vector2<f32> = Vector2 { x: 0.0, y: 0.0 };

    if revolve_angle == 0.0 {
        return dest_pos;
    }

    result_pos.x = origin_pos.x + (dest_pos.x - origin_pos.x) * (revolve_angle * PI / 180.0).cos() + (dest_pos.y - origin_pos.y) * (revolve_angle * PI / 180.0).sin();
    result_pos.y = origin_pos.y - (dest_pos.x - origin_pos.x) * (revolve_angle * PI / 180.0).sin() + (dest_pos.y - origin_pos.y) * (revolve_angle * PI / 180.0).cos();

    return result_pos;
}

pub fn render_radar(ui: &mut Ui, config: Config, local_pos: Vector3<f32>, local_yaw: f32, points: Vec<(Vector3<f32>, f32)>) {
    ui.window("Radar")
        .resizable(false)
        .collapsible(false)
        .scroll_bar(false)
        .title_bar(false)
        .movable(false)
        .draw_background(false)
        .bring_to_front_on_focus(false)
        .size([config.radar_range * 2.0, config.radar_range * 2.0], imgui::Condition::Always)
        .position([0.0, 0.0], imgui::Condition::FirstUseEver)
        .build(|| {
            let window_pos = Vector2 { x: ui.window_pos()[0] + config.radar_range, y: ui.window_pos()[1] + config.radar_range };
            
            // Cross Line
            if config.show_radar_cross_line {
                let vertical1 = Vector2 { x: window_pos.x - config.radar_range, y: window_pos.y };
                let vertical2 = Vector2 { x: window_pos.x + config.radar_range, y: window_pos.y };

                let horizontal1 = Vector2 { x: window_pos.x, y: window_pos.y - config.radar_range };
                let horizontal2 = Vector2 { x: window_pos.x, y: window_pos.y + config.radar_range };

                let color = color_u32_to_f32(config.radar_cross_line_color);
                
                ui.get_window_draw_list().add_line(vertical1, vertical2, color).build();
                ui.get_window_draw_list().add_line(horizontal1, horizontal2, color).build();
            }

            // Points
            let circle_size = 4.0 * config.radar_point_size_proportion;
            let arrow_size = 11.0 * config.radar_point_size_proportion;
            let arc_arrow_size = 7.0 * config.radar_point_size_proportion;
            let point_color = ImColor32::from_rgba(255, 0, 0, 255);

            for (pos, yaw) in points {
                let distance = f32::sqrt(f32::powf(local_pos.x - pos.x, 2.0) + f32::powf(local_pos.y - pos.y, 2.0)) / config.proportion * config.radar_range * 2.0;
                let angle = (local_yaw - f32::atan2(pos.y - local_pos.y, pos.x - local_pos.x) * 180.0 / PI) * PI / 180.0;
                let point_pos = Vector2 { x: window_pos.x + distance * f32::sin(angle), y: window_pos.y - distance * f32::cos(angle) };

                if point_pos.x < window_pos.x - config.radar_range || point_pos.x > window_pos.x + config.radar_range || point_pos.y > window_pos.y + config.radar_range || point_pos.y < window_pos.y - config.radar_range {
                    continue;
                }

                if config.radar_type == 0 {
                    ui.get_window_draw_list().add_circle(point_pos, circle_size, point_color).build();
                    ui.get_window_draw_list().add_circle(point_pos, circle_size, ImColor32::from_rgb(0, 0, 0)).filled(true).build();
                } else if config.radar_type == 1 {
                    let angle2 = (local_yaw - yaw) + 180.0;
                    let re_point = revolve_coordinates_system(angle2, window_pos, point_pos);
                    
                    let a = revolve_coordinates_system(-angle2, window_pos, Vector2 { x: re_point.x, y: re_point.y + arrow_size });
                    let b = revolve_coordinates_system(-angle2, window_pos, Vector2 { x: re_point.x - arrow_size / 1.5, y: re_point.y - arrow_size / 2.0 });
                    let c = revolve_coordinates_system(-angle2, window_pos, Vector2 { x: re_point.x + arrow_size / 1.5, y: re_point.y - arrow_size / 2.0 });

                    ui.get_window_draw_list().add_polyline(vec![a, b, point_pos, c], point_color).filled(true).build();
                    ui.get_window_draw_list().add_polyline(vec![a, b, point_pos, c], ImColor32::from_rgba(0, 0, 0, 255)).thickness(0.1).build();
                } else {
                    let angle2 = (local_yaw - yaw) - 90.0;
                    let triangle_point = Vector2 { x: point_pos.x + (arc_arrow_size + arc_arrow_size / 3.0) * f32::cos(-angle2 * PI / 180.0), y: point_pos.y - (arc_arrow_size + arc_arrow_size / 3.0) * f32::sin(-angle2 * PI / 180.0) };
                    let triangle_point_2 = Vector2 { x: point_pos.x + arc_arrow_size * f32::cos(-(angle2 - 30.0) * PI / 180.0), y: point_pos.y - arc_arrow_size * f32::sin(-(angle2 - 30.0) * PI / 180.0) };
                    let triangle_point_3 = Vector2 { x: point_pos.x + arc_arrow_size * f32::cos(-(angle2 + 30.0) * PI / 180.0), y: point_pos.y - arc_arrow_size * f32::sin(-(angle2 + 30.0) * PI / 180.0) };

                    ui.get_window_draw_list().add_circle(point_pos, 0.85 * arc_arrow_size, point_color).thickness(30.0).filled(true).build();
                    ui.get_window_draw_list().add_circle(point_pos, 0.95 * arc_arrow_size, ImColor32::from_rgba(0, 0, 0, 255)).thickness(0.1).build();
                    ui.get_window_draw_list().add_polyline(vec![triangle_point, triangle_point_2, triangle_point_3], ImColor32::from_rgba(220, 220, 220, 255)).filled(true).build();
                }
            }
        });
}