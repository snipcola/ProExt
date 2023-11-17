use mint::{Vector3, Vector2};
use crate::cheat::classes::{entity::Entity, offsets::PAWN_OFFSETS, view::View};
use crate::utils::{config::Config, process_manager::{read_memory_auto, trace_address}};
use crate::cheat::classes::offsets::BOMB_OFFSETS;

pub fn is_enemy_at_crosshair(window_info: ((i32, i32), (i32, i32)), local_entity_pawn_address: u64, local_entity_pawn_team_id: i32, game_address_entity_list: u64, game_view: View, config: Config) -> (bool, bool) {
    let mut u_handle: u32 = 0;
    
    if !read_memory_auto(local_entity_pawn_address + (*PAWN_OFFSETS.lock().unwrap()).ent_index as u64, &mut u_handle) {
        return (false, false);
    }

    if !read_memory_auto(local_entity_pawn_address + (*PAWN_OFFSETS.lock().unwrap()).ent_index as u64, &mut u_handle) {
        return (false, false);
    }

    let list_entry: u64 = trace_address(game_address_entity_list, &[0x8 * (u_handle >> 9) + 0x10, 0x0]);

    if list_entry == 0 {
        return (false, false);
    }

    let mut pawn_address: u64 = 0;

    if !read_memory_auto(list_entry + 0x78 * (u_handle & 0x1FF) as u64, &mut pawn_address) {
        return (false, false);
    }

    let mut entity = Entity::default();

    if !entity.update_pawn(pawn_address, window_info, game_view) {
        return (false, false);
    }

    let allow_shoot = {
        if config.misc.enabled && config.misc.exclude_team {
            local_entity_pawn_team_id != entity.pawn.team_id && entity.pawn.health > 0
        } else {
            entity.pawn.health > 0
        }
    };

    return (true, allow_shoot);
}

pub fn is_enemy_in_fov(config: Config, aim_pos: Vector3<f32>, camera_pos: Vector3<f32>, view_angle: Vector2<f32>) -> Option<f32> {
    let pos = Vector3 { x: aim_pos.x - camera_pos.x, y: aim_pos.y - camera_pos.y, z: aim_pos.z - camera_pos.z };
    let distance = (pos.x.powf(2.0) + pos.y.powf(2.0)).sqrt();
    let yaw = pos.y.atan2(pos.x) * 57.295779513 - view_angle.y;
    let pitch = -(pos.z / distance).atan() * 57.295779513 - view_angle.x;
    let norm = (yaw.powf(2.0) + pitch.powf(2.0)).sqrt();
    
    if norm > config.aimbot.fov {
        return None;
    }

    return Some(norm);
}

pub fn get_bomb(bomb_address: u64) -> Option<u64> {
    let mut planted_bomb: u64 = 0;

    if !read_memory_auto(bomb_address, &mut planted_bomb) {
        return None;
    }

    if !read_memory_auto(planted_bomb, &mut planted_bomb) {
        return None;
    }

    return Some(planted_bomb);
}

pub fn get_bomb_planted(bomb_address: u64) -> bool {
    let mut is_bomb_planted: bool = false;
    
    if !read_memory_auto(bomb_address - 0x8, &mut is_bomb_planted) {
        return false;
    };

    return is_bomb_planted;
}

pub fn get_bomb_site(planted_bomb: u64) -> Option<String> {
    let mut site: u32 = 0;

    if !read_memory_auto(planted_bomb + (*BOMB_OFFSETS.lock().unwrap()).bomb_site as u64, &mut site) {
        return None;
    }

    if site == 1 {
        return Some("B".to_string());
    } else {
        return Some("A".to_string());
    }
}

pub fn get_bomb_position(planted_bomb: u64) -> Option<Vector3<f32>> {
    let mut bomb_node = 0;

    if !read_memory_auto(planted_bomb + (*PAWN_OFFSETS.lock().unwrap()).game_scene_node as u64, &mut bomb_node) {
        return None;
    }

    let mut bomb_pos = Vector3 { x: 0.0, y: 0.0, z: 0.0 };

    if !read_memory_auto(bomb_node + (*PAWN_OFFSETS.lock().unwrap()).vec_abs_origin as u64, &mut bomb_pos) {
        return None;
    }

    return Some(bomb_pos);
}