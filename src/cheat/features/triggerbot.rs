use std::time::{Instant, Duration};
use winapi::um::winuser::{mouse_event, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP};

use crate::{utils::{process_manager::{read_memory_auto, trace_address}, config::Config}, cheat::classes::{entity::Entity, offsets::PAWN_OFFSETS, game::Game}};

pub fn run_triggerbot(local_entity: Entity, game: Game, config: Config, window_info: ((i32, i32), (i32, i32)), started: Instant) -> bool {
    let mut u_handle: u32 = 0;

    if !read_memory_auto(local_entity.pawn.address + (*PAWN_OFFSETS.lock().unwrap()).i_id_ent_index as u64, &mut u_handle) {
        return false;
    }

    let list_entry: u64 = trace_address(game.address.entity_list, &[0x8 * (u_handle >> 9) + 0x10, 0x0]);

    if list_entry == 0 {
        return false;
    }

    let mut pawn_address: u64 = 0;

    if !read_memory_auto(list_entry + 0x78 * (u_handle & 0x1FF) as u64, &mut pawn_address) {
        return false;
    }

    let mut entity = Entity::default();

    if !entity.update_pawn(pawn_address, window_info, game.view) {
        return false;
    }

    let allow_shoot = {
        if config.team_check {
            local_entity.pawn.team_id != entity.pawn.team_id && entity.pawn.health > 0
        } else {
            entity.pawn.health > 0
        }
    };

    if !allow_shoot {
        return false;
    }

    if started.elapsed() >= Duration::from_millis(config.trigger_delay as u64) {
        unsafe {
            mouse_event(MOUSEEVENTF_LEFTDOWN, 0, 0, 0, 0);
            mouse_event(MOUSEEVENTF_LEFTUP, 0, 0, 0, 0);
        };

        return true;
    }

    return false;
}