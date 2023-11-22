use std::f32::consts::PI;
use imgui::{Ui, ImColor32};
use mint::{Vector2, Vector3};

use crate::utils::config::Config;
use crate::ui::functions::{color_u32_to_f32, color_with_masked_alpha};

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
        .bring_to_front_on_focus(false)
        .draw_background(false)
        .size([config.radar.range * 2.0, config.radar.range * 2.0], imgui::Condition::Always)
        .position([0.0, 0.0], imgui::Condition::Always)
        .build(|| {
            let (full_window_pos, full_window_size) = (ui.window_pos(), ui.window_size());
            let window_pos = Vector2 { x: full_window_pos[0] + config.radar.range, y: full_window_pos[1] + config.radar.range };

            // Window Mask
            ui.get_window_draw_list().add_rect(Vector2 { x: full_window_pos[0], y: full_window_pos[1] }, Vector2 { x: full_window_pos[0] + full_window_size[0], y: full_window_pos[1] + full_window_size[1] }, ImColor32::from_rgba_f32s(0.0, 0.0, 0.0, config.radar.alpha)).filled(true).build();
            
            // Cross Line
            if config.radar.crossline_enabled {
                let vertical1 = Vector2 { x: window_pos.x - config.radar.range, y: window_pos.y };
                let vertical2 = Vector2 { x: window_pos.x + config.radar.range, y: window_pos.y };

                let horizontal1 = Vector2 { x: window_pos.x, y: window_pos.y - config.radar.range };
                let horizontal2 = Vector2 { x: window_pos.x, y: window_pos.y + config.radar.range };

                let color = color_u32_to_f32(config.radar.crossline_color);
                
                ui.get_window_draw_list().add_line(vertical1, vertical2, color).build();
                ui.get_window_draw_list().add_line(horizontal1, horizontal2, color).build();
            }

            // Points
            let circle_size = 7.0 * config.radar.point_size;
            let arrow_size = 11.0 * config.radar.point_size;
            let arc_arrow_size = 7.0 * config.radar.point_size;

            for (pos, yaw) in points {
                let distance = ((local_pos.x - pos.x).powf(2.0) + (local_pos.y - pos.y).powf(2.0)).sqrt() / config.radar.proportion * config.radar.range * 2.0;
                let angle = (local_yaw - (pos.y - local_pos.y).atan2(pos.x - local_pos.x) * 180.0 / PI) * PI / 180.0;
                let point_pos = Vector2 { x: window_pos.x + distance * angle.sin(), y: window_pos.y - distance * angle.cos() };

                if point_pos.x < window_pos.x - config.radar.range || point_pos.x > window_pos.x + config.radar.range || point_pos.y > window_pos.y + config.radar.range || point_pos.y < window_pos.y - config.radar.range {
                    continue;
                }

                if config.radar.mode == 0 {
                    ui.get_window_draw_list().add_circle(point_pos, circle_size, color_u32_to_f32(config.radar.color)).filled(true).build();
                    
                    if config.radar.outline {
                        ui.get_window_draw_list().add_circle(point_pos, circle_size, color_with_masked_alpha(config.radar.color, 0xFF000000)).thickness(0.1).build();
                    }
                } else if config.radar.mode == 1 {
                    let angle2 = (local_yaw - yaw) + 180.0;
                    let re_point = revolve_coordinates_system(angle2, window_pos, point_pos);
                    
                    let a = revolve_coordinates_system(-angle2, window_pos, Vector2 { x: re_point.x, y: re_point.y + arrow_size });
                    let b = revolve_coordinates_system(-angle2, window_pos, Vector2 { x: re_point.x - arrow_size / 1.5, y: re_point.y - arrow_size / 2.0 });
                    let c = revolve_coordinates_system(-angle2, window_pos, Vector2 { x: re_point.x + arrow_size / 1.5, y: re_point.y - arrow_size / 2.0 });

                    ui.get_window_draw_list().add_polyline(vec![a, b, point_pos, c], color_u32_to_f32(config.radar.color)).filled(true).build();
                    
                    if config.radar.outline {
                        ui.get_window_draw_list().add_polyline(vec![a, b, point_pos, c], color_with_masked_alpha(config.radar.color, 0xFF000000)).thickness(0.1).build();
                    }
                } else {
                    let angle2 = (local_yaw - yaw) - 90.0;
                    let triangle_point = Vector2 { x: point_pos.x + (arc_arrow_size + arc_arrow_size / 3.0) * (-angle2 * PI / 180.0).cos(), y: point_pos.y - (arc_arrow_size + arc_arrow_size / 3.0) * (-angle2 * PI / 180.0).sin() };
                    let triangle_point_2 = Vector2 { x: point_pos.x + arc_arrow_size * (-(angle2 - 30.0) * PI / 180.0).cos(), y: point_pos.y - arc_arrow_size * (-(angle2 - 30.0) * PI / 180.0).sin() };
                    let triangle_point_3 = Vector2 { x: point_pos.x + arc_arrow_size * (-(angle2 + 30.0) * PI / 180.0).cos(), y: point_pos.y - arc_arrow_size * (-(angle2 + 30.0) * PI / 180.0).sin() };

                    ui.get_window_draw_list().add_circle(point_pos, 0.85 * arc_arrow_size, color_u32_to_f32(config.radar.color)).thickness(30.0).filled(true).build();
                    
                    if config.radar.outline {
                        ui.get_window_draw_list().add_circle(point_pos, 0.95 * arc_arrow_size, color_with_masked_alpha(config.radar.color, 0xFF000000)).thickness(0.1).build();
                    }
                    
                    ui.get_window_draw_list().add_polyline(vec![triangle_point, triangle_point_2, triangle_point_3], ImColor32::from_rgba(220, 220, 220, 255)).filled(true).build();
                }
            }
        });
}