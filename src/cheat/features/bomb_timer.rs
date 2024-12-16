use std::sync::{Arc, Mutex};
use std::time::{Instant, Duration};

use lazy_static::lazy_static;

use imgui::Ui;
use mint::Vector4;

use crate::utils::cheat::config::{Config, CONFIG};
use crate::ui::functions::color_u32_to_f32;

lazy_static! {
    pub static ref IS_PLANTED: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    pub static ref PLANT_TIME: Arc<Mutex<Option<Instant>>> = Arc::new(Mutex::new(None));
    pub static ref BOMB_TIMER_RESET_POSITION: Arc<Mutex<Option<[f32; 2]>>> = Arc::new(Mutex::new(None));
}

pub fn render_bomb_timer(ui: &mut Ui, bomb_planted: bool, bomb_site: Option<String>, config: Config, no_pawn: bool) {
    let mut reset_position = BOMB_TIMER_RESET_POSITION.lock().unwrap();
    let (window_position, condition) = if let Some(position) = *reset_position {
        *reset_position = None;
        (position, imgui::Condition::Always)
    } else {
        (config.window_positions.bomb_timer, imgui::Condition::Once)
    };

    drop(reset_position);

    let mut is_planted = IS_PLANTED.lock().unwrap();
    let mut plant_time = PLANT_TIME.lock().unwrap();

    if bomb_planted && !*is_planted && ((*plant_time).is_none() || plant_time.unwrap().elapsed() > Duration::from_secs(60)) {
        *is_planted = true;
        *plant_time = Some(Instant::now());
    }
    
    if *is_planted && !bomb_planted {
        *is_planted = false;
        *plant_time = None;
    }

    let remaining_time: Option<u64> = {
        if let Some(plant_time) = *plant_time {
            let elapsed_time = plant_time.elapsed().as_secs() as u64;

            if elapsed_time < 40 {
                Some(40 - elapsed_time)
            } else {
                None
            }
        } else {
            None
        }
    };

    ui.window("Bomb")
        .collapsible(false)
        .always_auto_resize(true)
        .position(window_position, condition)
        .build(|| {
            (*CONFIG.lock().unwrap()).window_positions.bomb_timer = ui.window_pos();

            let disabled = color_u32_to_f32(config.misc.bomb_timer_color_disabled);
            let enabled = color_u32_to_f32(config.misc.bomb_timer_color_enabled);

            let disabled_color = Vector4 { x: disabled.0, y: disabled.1, z: disabled.2, w: disabled.3 };
            let enabled_color = Vector4 { x: enabled.0, y: enabled.1, z: enabled.2, w: enabled.3 };

            if no_pawn {
                ui.text_colored(disabled_color, "Couldn't fetch information.")
            } else if *is_planted && remaining_time.is_some() && plant_time.is_some() && bomb_site.is_some() && remaining_time.unwrap() > 0 {
                ui.text("The bomb has been planted at");
                ui.same_line();
                ui.text_colored(enabled_color, format!("Site {}.", bomb_site.unwrap()));

                ui.text_colored(enabled_color, format!("{} seconds", remaining_time.unwrap()));
                ui.same_line();
                ui.text("remaining!");
            } else {
                ui.text_colored(disabled_color, "The bomb has not been planted.")
            }
        });
}