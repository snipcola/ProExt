use std::f32::consts::PI;

use imgui::Ui;
use mint::{Vector3, Vector2, Vector4};
use crate::{cheat::classes::{bone::{BoneJointPos, bone_joint_list, BoneIndex}, view::View}, utils::config::Config, ui::main::{color_u32_to_f32, rectangle}};

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
                ui.get_background_draw_list().add_line(previous.screen_pos, current.screen_pos, color_u32_to_f32(config.bone_color)).thickness(1.3).build();
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

pub fn render_line_to_enemy(ui: &mut Ui, rect: Vector4<f32>, config: Config, window_width: i32) {
    ui.get_background_draw_list().add_line(Vector2 { x: rect.x + rect.z / 2.0, y: rect.y }, Vector2 { x: window_width as f32 / 2.0, y: 0.0 }, color_u32_to_f32(config.line_to_enemy_color)).thickness(1.2).build();
}

pub fn render_box(ui: &mut Ui, rect: Vector4<f32>, config: Config) {
    rectangle(ui, Vector2 { x: rect.x, y: rect.y }, Vector2 { x: rect.z, y: rect.w }, color_u32_to_f32(config.box_color).into(), 1.3);
}