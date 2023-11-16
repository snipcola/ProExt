use std::{sync::{Arc, Mutex}, time::{Instant, Duration}};
use imgui::Ui;
use lazy_static::lazy_static;
use mint::{Vector4, Vector3};
use crate::{utils::{config::{Config, CONFIG, WindowPosition}, process_manager::read_memory_auto}, ui::functions::color_u32_to_f32, cheat::classes::offsets::{BOMB_OFFSETS, PAWN_OFFSETS}};

lazy_static! {
    pub static ref IS_PLANTED: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    pub static ref PLANT_TIME: Arc<Mutex<Option<Instant>>> = Arc::new(Mutex::new(None));

    pub static ref BOMB_POSITION: Arc<Mutex<Option<Vector3<f32>>>> = Arc::new(Mutex::new(None));
    pub static ref BOMB_SITE: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
}

pub fn get_planted_bomb(bomb_address: u64) -> Option<u64> {
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

pub fn render_bomb_timer(ui: &mut Ui, enabled: bool, bomb_address: u64, config: Config) {
    let window_position = config.window_positions.bomb_timer;

    let mut is_planted = IS_PLANTED.lock().unwrap();
    let mut plant_time = PLANT_TIME.lock().unwrap();
    
    let is_bomb_planted = get_bomb_planted(bomb_address);
    let planted_bomb = match is_bomb_planted {
        true => get_planted_bomb(bomb_address),
        false => None
    };

    let bomb_site = match is_bomb_planted {
        true => match planted_bomb {
            Some(bomb) => get_bomb_site(bomb),
            _ => None
        },
        false => None
    };
    
    let bomb_pos = match is_bomb_planted {
        true => match planted_bomb {
            Some(bomb) => get_bomb_position(bomb),
            _ => None
        },
        false => None
    };

    if is_bomb_planted && !*is_planted && ((*plant_time).is_none() || plant_time.unwrap().elapsed() > Duration::from_secs(60)) {
        *is_planted = true;
        *plant_time = Some(Instant::now());

        if let Some(bomb_pos) = bomb_pos {
            *BOMB_POSITION.lock().unwrap() = Some(bomb_pos);
        }

        if let Some(bomb_site) = bomb_site.clone() {
            *BOMB_SITE.lock().unwrap() = Some(bomb_site);
        }
    }
    
    if *is_planted && !is_bomb_planted {
        *is_planted = false;
        *plant_time = None;

        *BOMB_POSITION.lock().unwrap() = None;
        *BOMB_SITE.lock().unwrap() = None;
    }

    if !enabled {
        return;
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