use std::f32::consts::PI;

use imgui::Ui;
use mint::{Vector3, Vector2};
use crate::{utils::{config::Config, process_manager::read_memory_auto}, cheat::classes::{game::set_view_angle, entity::CUtlVector, bone::{BoneIndex, aim_position_to_bone_index, BoneJointPos}}, ui::main::{distance_between_vec2, color_u32_to_f32, color_with_masked_alpha}};

pub fn run_aimbot(config: Config, aimbot_info: (f32, f32), view_angle: Vector2<f32>, shots_fired: u64, aim_punch_cache: CUtlVector) {
    let (mut yaw, mut pitch) = aimbot_info;

    yaw = yaw * (1.0 - config.aimbot.smooth) + view_angle.y;
    pitch = pitch * (1.0 - config.aimbot.smooth) + view_angle.x;

    if shots_fired > config.aimbot.start_bullet as u64 {
        let mut punch_angle = Vector2 { x: 0.0, y: 0.0 };

        if aim_punch_cache.count <= 0 && aim_punch_cache.count > 0xFFFF {
            return;
        }

        if !read_memory_auto(aim_punch_cache.data + (aim_punch_cache.count - 1) * std::mem::size_of::<Vector3<f32>>() as u64, &mut punch_angle) {
            return;
        }

        yaw = yaw - punch_angle.y * config.aimbot.rcs_yaw;
        pitch = pitch - punch_angle.x * config.aimbot.rcs_pitch;
    }

    set_view_angle(yaw, pitch);
}

pub fn aimbot_check(bone_pos_list: [BoneJointPos; 30], window_width: i32, window_height: i32, aim_pos: &mut Option<Vector3<f32>>, max_aim_distance: &mut f32, b_spotted_by_mask: u64, local_b_spotted_by_mask: u64, local_player_controller_index: u64, i: u64, in_air: bool, config: Config) {
    let pos = Vector2 { x: window_width as f32 / 2.0, y: window_height as f32 / 2.0 };
    let bone_index = aim_position_to_bone_index(config.aimbot.bone);
    let distance_to_sight = distance_between_vec2(bone_pos_list[bone_index].screen_pos, pos);

    if config.aimbot.only_grounded && in_air {
        return;
    }

    if distance_to_sight < *max_aim_distance {
        *max_aim_distance = distance_to_sight;

        if !config.aimbot.only_visible || b_spotted_by_mask & (1 << local_player_controller_index) != 0 || local_b_spotted_by_mask & (1 << i) != 0 {
            *aim_pos = Some(bone_pos_list[bone_index].pos);

            if bone_index as usize == BoneIndex::Head as usize {
                if let Some(aim_pos) = aim_pos {
                    aim_pos.z -= -1.0;
                }
            }
        }
    }
}

pub fn render_fov_circle(ui: &mut Ui, window_width: i32, window_height: i32, fov: i32, color: (u32, u32, u32, u32), config: Config) {
    let center_point: Vector2<f32> = Vector2 { x: window_width as f32 / 2.0, y: window_height as f32 / 2.0 };
    let radius = (config.aimbot.fov / 180.0 * PI / 2.0).tan() / (fov as f32 / 180.0 * PI / 2.0).tan() * window_width as f32;

    if config.aimbot.fov_circle_outline_enabled {
        ui.get_background_draw_list().add_circle(center_point, radius, color_with_masked_alpha(color, 0xFF000000)).thickness(3.0).build();
    }

    ui.get_background_draw_list().add_circle(center_point, radius, color_u32_to_f32(color)).build();
}