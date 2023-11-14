use std::{sync::{Arc, Mutex}, time::{Instant, Duration}};
use imgui::Ui;
use lazy_static::lazy_static;
use mint::Vector4;
use crate::{utils::{config::{Config, CONFIG, WindowPosition}, process_manager::read_memory_auto}, cheat::classes::offsets::{BOMB, BOMB_OFFSETS}, ui::main::color_u32_to_f32};

lazy_static! {
    pub static ref IS_PLANTED: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    pub static ref PLANT_TIME: Arc<Mutex<Option<Instant>>> = Arc::new(Mutex::new(None));
}

pub fn get_bomb_site(planted: bool, client_dll_address: u64) -> Option<String> {
    if planted {
        let mut site: u32 = 0;
        let mut planted_c4: usize = 0;
        
        if !read_memory_auto(client_dll_address + (*BOMB.lock().unwrap()) as u64, &mut planted_c4) {
            return None;
        }

        if !read_memory_auto(planted_c4 as u64, &mut planted_c4) {
            return None;
        }

        if !read_memory_auto(planted_c4 as u64 + (*BOMB_OFFSETS.lock().unwrap()).bomb_site as u64, &mut site) {
            return None;
        }

        if site == 1 {
            return Some("B".to_string());
        } else {
            return Some("A".to_string());
        }
    }

    return None;
}

pub fn get_bomb_planted(client_dll_address: u64) -> bool {
    let planted_address = client_dll_address + (*BOMB.lock().unwrap()) as u64 - 0x8;
    let mut is_bomb_planted: bool = false;
    
    read_memory_auto(planted_address as u64, &mut is_bomb_planted);

    return is_bomb_planted;
}

pub fn render_bomb_timer(ui: &mut Ui, client_dll_address: u64, config: Config) {
    let window_position = config.window_positions.bomb_timer;

    let is_bomb_planted = get_bomb_planted(client_dll_address);
    let bomb_site = get_bomb_site(is_bomb_planted, client_dll_address);
    let mut is_planted = IS_PLANTED.lock().unwrap();
    let mut plant_time = PLANT_TIME.lock().unwrap();

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