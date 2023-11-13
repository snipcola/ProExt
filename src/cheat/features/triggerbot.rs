use std::time::{Instant, Duration};
use mouse_rs::{Mouse, types::keys::Keys};

use crate::utils::config::Config;

pub fn run_triggerbot((aiming_at_enemy, allow_shoot): (bool, bool), config: Config, on_entity: &mut Option<Instant>, shot_entity: &mut Instant, tries: &mut u32) {
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