use std::ops::BitAnd;
use imgui::{Ui, StyleColor};
use mint::Vector4;

use crate::{utils::{process_manager::read_memory_auto, config::{CONFIG, WindowPosition, Config}}, ui::{functions::color_u32_to_f32, main::WINDOWS_ACTIVE}, cheat::classes::offsets::Offsets};

pub fn is_spectating(entity_controller_address: u64, game_entity_list_entry: u64, local_entity_pawn_address: u64) -> bool {
    let mut pawn: u32 = 0;
    let mut cs_player_pawn: usize = 0;
    let mut observer_services: usize = 0;

    if !read_memory_auto(entity_controller_address + Offsets::CBasePlayerController::m_hPawn as u64, &mut pawn) {
        return false;
    }

    if !read_memory_auto(game_entity_list_entry + 120 * pawn.bitand(0x1FF) as u64, &mut cs_player_pawn) {
        return false;
    }

    if !read_memory_auto(cs_player_pawn as u64 + Offsets::C_BasePlayerPawn::m_pObserverServices as u64, &mut observer_services) {
        return false;
    }

    if observer_services != 0 {
        let mut observer_target: u32 = 0;
        let mut controller: usize = 0;
        
        if !read_memory_auto(observer_services as u64 + Offsets::CPlayer_ObserverServices::m_hObserverTarget as u64, &mut observer_target) {
            return false;
        }

        if !read_memory_auto(game_entity_list_entry + 120 * observer_target.bitand(0x1FF) as u64, &mut controller) {
            return false;
        }

        if controller as u64 == local_entity_pawn_address {
            return true;
        }
    }

    return false;
}

pub fn render_spectator_list(ui: &mut Ui, spectators: Vec<String>, config: Config) {
    let window_position = config.window_positions.spectator_list;
    let titlebar_color = ui.push_style_color(StyleColor::TitleBgActive, [0.01, 0.01, 0.01, 1.0]);

    ui.window("Spectators")
        .collapsible(false)
        .always_auto_resize(true)
        .position([window_position.x, window_position.y], imgui::Condition::Appearing)
        .build(|| {
            let window_active = ui.is_window_hovered();
            (*WINDOWS_ACTIVE.lock().unwrap()).insert("spectator_list".to_string(), window_active);
            
            let window_pos = ui.window_pos();
            let mut config_mut = CONFIG.lock().unwrap();
            (*config_mut).window_positions.spectator_list = WindowPosition { x: window_pos[0], y: window_pos[1] };
            drop(config_mut);

            let spectator_list_color_f32 = color_u32_to_f32(config.misc.spectator_list_color);
            let spectator_list_color = Vector4 { x: spectator_list_color_f32.0, y: spectator_list_color_f32.1, z: spectator_list_color_f32.2, w: spectator_list_color_f32.3 };

            if spectators.len() == 0 {
                ui.text_colored(spectator_list_color, "No spectators.");
            } else {
                ui.text_colored(spectator_list_color, "Spectating:");

                for spectator in spectators {
                    ui.text_colored(spectator_list_color, format!("- {}", spectator));
                }
            }
        });

    titlebar_color.end();
}