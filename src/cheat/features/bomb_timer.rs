use std::{sync::{Arc, Mutex}, time::{Instant, Duration}};
use imgui::Ui;
use lazy_static::lazy_static;
use mint::Vector4;
use crate::{utils::{config::{Config, CONFIG, WindowPosition}, process_manager::read_memory_auto}, ui::functions::color_u32_to_f32, cheat::classes::offsets::BOMB_OFFSETS};

lazy_static! {
    pub static ref IS_PLANTED: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    pub static ref PLANT_TIME: Arc<Mutex<Option<Instant>>> = Arc::new(Mutex::new(None));
}

pub fn get_bomb_node_address(bomb_address: u64) -> Option<u64> {
    let mut bomb_node: u64 = 0;

    if !read_memory_auto(bomb_address, &mut bomb_node) {
        return None;
    }

    if !read_memory_auto(bomb_node, &mut bomb_node) {
        return None;
    }

    return Some(bomb_node);
}

pub fn get_bomb_planted(bomb_address: u64) -> bool {
    let planted_address = bomb_address - 0x8;
    let mut is_bomb_planted: bool = false;
    
    if !read_memory_auto(planted_address as u64, &mut is_bomb_planted) {
        return false;
    };

    return is_bomb_planted;
}

pub fn get_bomb_site(bomb_node_address: u64) -> Option<String> {
    let mut site: u32 = 0;

    if !read_memory_auto(bomb_node_address + (*BOMB_OFFSETS.lock().unwrap()).bomb_site as u64, &mut site) {
        return None;
    }

    if site == 1 {
        return Some("B".to_string());
    } else {
        return Some("A".to_string());
    }
}

pub fn render_bomb_timer(ui: &mut Ui, bomb_address: u64, config: Config) {
    let window_position = config.window_positions.bomb_timer;

    let is_bomb_planted = get_bomb_planted(bomb_address);
    let mut is_planted = IS_PLANTED.lock().unwrap();
    let mut plant_time = PLANT_TIME.lock().unwrap();
    let bomb_node_address = match is_bomb_planted {
        true => get_bomb_node_address(bomb_address),
        false => None
    };

    let bomb_site = match is_bomb_planted {
        true => match bomb_node_address {
            Some(address) => get_bomb_site(address),
            _ => None
        },
        false => None
    };

    if is_bomb_planted && !*is_planted && ((*plant_time).is_none() || plant_time.unwrap().elapsed() > Duration::from_secs(60)) {
        *is_planted = true;
        *plant_time = Some(Instant::now());
    }
    
    if *is_planted && !is_bomb_planted {
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

    ui.window("Bomb Timer")
        .resizable(false)
        .collapsible(false)
        .scroll_bar(false)
        .title_bar(false)
        .always_auto_resize(true)
        .position([window_position.x, window_position.y], imgui::Condition::Appearing)
        .build(|| {
            let window_pos = ui.window_pos();
            let mut config_mut = CONFIG.lock().unwrap();
            (*config_mut).window_positions.bomb_timer = WindowPosition { x: window_pos[0], y: window_pos[1] };
            drop(config_mut);

            let disabled = color_u32_to_f32(config.misc.bomb_timer_color_disabled);
            let enabled = color_u32_to_f32(config.misc.bomb_timer_color_enabled);

            let disabled_color = Vector4 { x: disabled.0, y: disabled.1, z: disabled.2, w: disabled.3 };
            let enabled_color = Vector4 { x: enabled.0, y: enabled.1, z: enabled.2, w: enabled.3 };

            ui.text("Bomb Timer");
            ui.separator();

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
}