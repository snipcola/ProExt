use mint::{Vector3, Vector2};
use crate::{utils::{config::Config, process_manager::read_memory_auto}, cheat::classes::{game::set_view_angle, entity::CUtlVector, bone::{BoneIndex, aim_position_to_bone_index, BoneJointPos}}, ui::main::distance_between_vec2};

pub fn run_aimbot(config: Config, aim_pos: Vector3<f32>, camera_pos: Vector3<f32>, view_angle: Vector2<f32>, shots_fired: u64, aim_punch_cache: CUtlVector) {
    let pos = Vector3 { x: aim_pos.x - camera_pos.x, y: aim_pos.y - camera_pos.y, z: aim_pos.z - camera_pos.z };
    let distance = f32::sqrt(f32::powf(pos.x, 2.0) + f32::powf(pos.y, 2.0));
    let mut yaw = f32::atan2(pos.y, pos.x) * 57.295779513 - view_angle.y;
    let mut pitch = -f32::atan(pos.z / distance) * 57.295779513 - view_angle.x;
    let norm = f32::sqrt(f32::powf(yaw, 2.0) + f32::powf(pitch, 2.0));

    if norm > config.aim_fov {
        return;
    }

    yaw = yaw * (1.0 - config.smooth) + view_angle.y;
    pitch = pitch * (1.0 - config.smooth) + view_angle.x;

    if shots_fired > config.rcs_bullet as u64 {
        let mut punch_angle = Vector2 { x: 0.0, y: 0.0 };

        if aim_punch_cache.count <= 0 && aim_punch_cache.count > 0xFFFF {
            return;
        }

        if !read_memory_auto(aim_punch_cache.data + (aim_punch_cache.count - 1) * std::mem::size_of::<Vector3<f32>>() as u64, &mut punch_angle) {
            return;
        }

        yaw = yaw - punch_angle.y * config.rcs_scale.0;
        pitch = pitch - punch_angle.x * config.rcs_scale.1;
    }

    set_view_angle(yaw, pitch);
}

pub fn aimbot_check(bone_pos_list: [BoneJointPos; 30], window_width: i32, window_height: i32, aim_pos: &mut Option<Vector3<f32>>, max_aim_distance: &mut f32, b_spotted_by_mask: u64, local_b_spotted_by_mask: u64, local_player_controller_index: u64, i: u64, config: Config) {
    let pos = Vector2 { x: window_width as f32 / 2.0, y: window_height as f32 / 2.0 };
    let bone_index = aim_position_to_bone_index(config.aim_position);
    let distance_to_sight = distance_between_vec2(bone_pos_list[bone_index].screen_pos, pos);

    if distance_to_sight < *max_aim_distance {
        *max_aim_distance = distance_to_sight;

        if !config.visible_check || b_spotted_by_mask & (1 << local_player_controller_index) != 0 || local_b_spotted_by_mask & (1 << i) != 0 {
            *aim_pos = Some(bone_pos_list[bone_index].pos);

            if bone_index as usize == BoneIndex::Head as usize {
                if let Some(aim_pos) = aim_pos {
                    aim_pos.z -= -1.0;
                }
            }
        }
    }
}