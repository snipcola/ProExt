use std::time::{Instant, Duration};
use mouse_rs::{Mouse, types::keys::Keys};

use crate::{utils::{process_manager::{read_memory_auto, trace_address}, config::Config}, cheat::classes::{entity::Entity, offsets::PAWN_OFFSETS, game::Game}};

pub fn run_triggerbot(local_entity: Entity, game: Game, config: Config, window_info: ((i32, i32), (i32, i32)), on_entity: &mut Option<Instant>, shot_entity: &mut Instant, tries: &mut u32) {
    let mut u_handle: u32 = 0;
    let mut check_tries = || {
        if *tries > 500 {
            *on_entity = None;
            *tries = 0;
        } else {
            *tries += 1;
        }
    };

    if !read_memory_auto(local_entity.pawn.address + (*PAWN_OFFSETS.lock().unwrap()).i_id_ent_index as u64, &mut u_handle) {
        check_tries();
        return;
    }

    let list_entry: u64 = trace_address(game.address.entity_list, &[0x8 * (u_handle >> 9) + 0x10, 0x0]);

    if list_entry == 0 {
        check_tries();
        return;
    }

    let mut pawn_address: u64 = 0;

    if !read_memory_auto(list_entry + 0x78 * (u_handle & 0x1FF) as u64, &mut pawn_address) {
        check_tries();
        return;
    }

    let mut entity = Entity::default();

    if !entity.update_pawn(pawn_address, window_info, game.view) {
        check_tries();
        return;
    }

    *tries = 0;

    let allow_shoot = {
        if config.team_check {
            local_entity.pawn.team_id != entity.pawn.team_id && entity.pawn.health > 0
        } else {
            entity.pawn.health > 0
        }
    };

    if !allow_shoot {
        return;
    }

    if let Some(on_entity) = on_entity {
        if on_entity.elapsed() >= Duration::from_millis(config.trigger_delay as u64) && shot_entity.elapsed() >= Duration::from_millis(config.trigger_interval as u64) {
            let mouse = Mouse::new();
            let _ = mouse.click(&Keys::LEFT);

            *shot_entity = Instant::now();
        }
    } else {
        *on_entity = Some(Instant::now());
    }
}