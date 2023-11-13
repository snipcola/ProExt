use std::time::{Instant, Duration};
use mouse_rs::{Mouse, types::keys::Keys};

use crate::{utils::config::Config, cheat::classes::view::View, ui::main::is_enemy_at_crosshair};

pub fn run_triggerbot(local_entity_pawn_address: u64, local_entity_pawn_team_id: i32, game_address_entity_list: u64, game_view: View, window_info: ((i32, i32), (i32, i32)), config: Config, on_entity: &mut Option<Instant>, shot_entity: &mut Instant, tries: &mut u32) {
    let (aiming_at_enemy, allow_shoot) = is_enemy_at_crosshair(window_info, local_entity_pawn_address, local_entity_pawn_team_id, game_address_entity_list, game_view, config);

    if !aiming_at_enemy {
        if *tries > 500 {
            *on_entity = None;
            *tries = 0;
        } else {
            *tries += 1;
        }

        return;
    }

    *tries = 0;

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