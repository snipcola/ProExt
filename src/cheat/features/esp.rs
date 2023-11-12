use std::f32::consts::PI;
use imgui::{Ui, ImColor32};
use mint::{Vector3, Vector2, Vector4};
use crate::{cheat::classes::{bone::{BoneJointPos, bone_joint_list, BoneIndex}, view::View}, utils::config::Config, ui::main::{color_u32_to_f32, rectangle, stroke_text, distance_between_vec3, mix_colors}};

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
                ui.get_background_draw_list().add_line(previous.screen_pos, current.screen_pos, color_u32_to_f32(config.skeleton_color)).thickness(1.3).build();
            }

            previous = current;
        }
    }
}

pub fn render_eye_ray(ui: &mut Ui, bone_pos_list: [BoneJointPos; 30], view_angle: Vector2<f32>, config: Config, view: View, window_info: ((i32, i32), (i32, i32))) {
    let line_length = f32::cos(view_angle.x * PI / 180.0) * 30.0;
    let head = bone_pos_list[BoneIndex::Head as usize];
    let mut pos = Vector2 { x: 0.0, y: 0.0 };

    if !view.world_to_screen(Vector3 { x: head.pos.x + f32::cos(view_angle.y * PI / 180.0) * line_length, y: head.pos.y + f32::sin(view_angle.y * PI / 180.0) * line_length, z: head.pos.z - f32::sin(view_angle.x * PI / 180.0) * 30.0 }, &mut pos, window_info) {
        return;
    }

    ui.get_background_draw_list().add_line(head.screen_pos, pos, color_u32_to_f32(config.eye_ray_color)).thickness(1.3).build();
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

        min.x = f32::min(joint.screen_pos.x, min.x);
        min.y = f32::min(joint.screen_pos.y, min.y);

        max.x = f32::max(joint.screen_pos.x, max.x);
        max.y = f32::max(joint.screen_pos.y, max.y);
    }

    return Vector4 { x: min.x, y: min.y, z: (max.x - min.x), w: (max.y - min.y) };
}

pub fn render_snap_line(ui: &mut Ui, rect: Vector4<f32>, config: Config, window_width: i32) {
    ui.get_background_draw_list().add_line(Vector2 { x: rect.x + rect.z / 2.0, y: rect.y }, Vector2 { x: window_width as f32 / 2.0, y: 0.0 }, color_u32_to_f32(config.snap_line_color)).thickness(1.2).build();
}

pub fn render_box(ui: &mut Ui, rect: Vector4<f32>, config: Config) {
    rectangle(ui, Vector2 { x: rect.x, y: rect.y }, Vector2 { x: rect.z, y: rect.w }, color_u32_to_f32(config.box_color).into(), 1.3, config.box_rounding);
}

pub fn render_weapon_name(ui: &mut Ui, weapon_name: &str, pos: Vector2<f32>, config: Config) {
    stroke_text(ui, weapon_name.to_string(), pos, color_u32_to_f32(config.weapon_name_color).into(), false);
}

pub fn render_distance(ui: &mut Ui, pawn_pos: Vector3<f32>, local_pawn_pos: Vector3<f32>, rect: Vector4<f32>, config: Config) {
    let distance = distance_between_vec3(pawn_pos, local_pawn_pos) as u32 / 100;
    stroke_text(ui, format!("{}m", distance), Vector2 { x: rect.x + rect.z + 4.0, y: rect.y }, color_u32_to_f32(config.distance_color).into(), false);
}

pub fn render_player_name(ui: &mut Ui, player_name: &str, rect: Vector4<f32>, config: Config) {
    if config.show_health_bar && config.health_bar_type == 1 {
        stroke_text(ui, player_name.to_string(), Vector2 { x: rect.x + rect.z / 2.0, y: rect.y - 13.0 - 14.0 }, color_u32_to_f32(config.player_name_color).into(), true);
    } else {
        stroke_text(ui, player_name.to_string(), Vector2 { x: rect.x + rect.z / 2.0, y: rect.y - 14.0 }, color_u32_to_f32(config.player_name_color).into(), true);
    }
}

pub fn render_health_bar(ui: &mut Ui, current_health: f32, rect_pos: Vector2<f32>, rect_size: Vector2<f32>, config: Config) {
    let max_health = 100.0;

    let background_color = ImColor32::from_rgba(90, 90, 90, 220);
    let frame_color = ImColor32::from_rgba(45, 45, 45, 220);

    let first_stage_color = ImColor32::from_rgba(96, 246, 113, 220);
    let second_stage_color = ImColor32::from_rgba(247, 214, 103, 220);
    let third_stage_color = ImColor32::from_rgba(255, 95, 95, 220);

    let in_range = |value: f32, min: f32, max: f32| -> bool {
        value > min && value <= max
    };

    let proportion = current_health / max_health;
    let (height, width) = ((rect_size.y * proportion), (rect_size.x * proportion));
    let color = {
        if in_range(proportion, 0.5, 1.0) {
            mix_colors(first_stage_color, second_stage_color, f32::powf(proportion, 2.5) * 3.0 - 1.0)
        } else {
            mix_colors(second_stage_color, third_stage_color, f32::powf(proportion, 2.5) * 4.0)
        }
    };
    
    ui.get_background_draw_list().add_rect(rect_pos, Vector2 { x: rect_pos.x + rect_size.x, y: rect_pos.y + rect_size.y }, background_color).filled(true).build();
    
    if config.health_bar_type == 0 {
        // Vertical
        ui.get_background_draw_list().add_rect(Vector2 { x: rect_pos.x, y: rect_pos.y + rect_size.y - height }, Vector2 { x: rect_pos.x + rect_size.x, y: rect_pos.y + rect_size.y }, color).filled(true).rounding(config.health_bar_rounding as f32).build();
        ui.get_background_draw_list().add_rect(rect_pos, Vector2 { x: rect_pos.x + rect_size.x, y: rect_pos.y + rect_size.y }, frame_color).thickness(1.0).rounding(config.health_bar_rounding as f32).build();
    } else {
        // Horizontal
        ui.get_background_draw_list().add_rect(rect_pos, Vector2 { x: rect_pos.x + width, y: rect_pos.y + rect_size.y }, color).filled(true).rounding(config.health_bar_rounding as f32).build();
        ui.get_background_draw_list().add_rect(rect_pos, Vector2 { x: rect_pos.x + rect_size.x, y: rect_pos.y + rect_size.y }, frame_color).rounding(config.health_bar_rounding as f32).thickness(1.0).build();   
    }
}