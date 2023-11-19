use std::{sync::{Arc, Mutex}, time::{Instant, Duration}};
use imgui::{Ui, StyleColor};
use lazy_static::lazy_static;
use mint::Vector4;
use crate::{utils::config::{Config, CONFIG, WindowPosition}, ui::{functions::color_u32_to_f32, main::WINDOWS_ACTIVE}};

lazy_static! {
    pub static ref IS_PLANTED: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    pub static ref PLANT_TIME: Arc<Mutex<Option<Instant>>> = Arc::new(Mutex::new(None));
}

pub fn render_bomb_timer(ui: &mut Ui, bomb_planted: bool, bomb_site: Option<String>, config: Config) {
    let window_position = config.window_positions.bomb_timer;

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

    let titlebar_color = ui.push_style_color(StyleColor::TitleBgActive, [0.01, 0.01, 0.01, 1.0]);

    ui.window("Bomb")
        .collapsible(false)
        .always_auto_resize(true)
        .position([window_position.x, window_position.y], imgui::Condition::Appearing)
        .build(|| {
            let window_active = ui.is_window_hovered();
            (*WINDOWS_ACTIVE.lock().unwrap()).insert("bomb_timer".to_string(), window_active);

            let window_pos = ui.window_pos();
            let mut config_mut = CONFIG.lock().unwrap();
            (*config_mut).window_positions.bomb_timer = WindowPosition { x: window_pos[0], y: window_pos[1] };
            drop(config_mut);

            let disabled = color_u32_to_f32(config.misc.bomb_timer_color_disabled);
            let enabled = color_u32_to_f32(config.misc.bomb_timer_color_enabled);

            let disabled_color = Vector4 { x: disabled.0, y: disabled.1, z: disabled.2, w: disabled.3 };
            let enabled_color = Vector4 { x: enabled.0, y: enabled.1, z: enabled.2, w: enabled.3 };

            if *is_planted && remaining_time.is_some() && plant_time.is_some() && bomb_site.is_some() && remaining_time.unwrap() > 0 {
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

    titlebar_color.end();
}