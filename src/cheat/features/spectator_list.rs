use std::{sync::{Arc, Mutex}, ops::BitAnd};
use imgui::Ui;
use mint::Vector4;
use lazy_static::lazy_static;
use crate::{utils::{process_manager::rpm_offset, config::{CONFIG, Config}}, ui::{functions::color_u32_to_f32, main::WINDOWS_ACTIVE}, cheat::classes::offsets::Offsets};

lazy_static! {
    pub static ref SPECTATOR_LIST_RESET_POSITION: Arc<Mutex<Option<[f32; 2]>>> = Arc::new(Mutex::new(None));
}

pub fn is_spectating(entity_controller_address: u64, game_entity_list_entry: u64, local_entity_pawn_address: u64) -> bool {
    let mut pawn: u32 = 0;
    let mut cs_player_pawn: usize = 0;
    let mut observer_services: usize = 0;

    if !rpm_offset(entity_controller_address, Offsets::CBasePlayerController::m_hPawn as u64, &mut pawn) {
        return false;
    }

    if let Some(sum) = (120 as u64).checked_mul(pawn.bitand(0x1FF) as u64) {
        if !rpm_offset(game_entity_list_entry, sum, &mut cs_player_pawn) {
            return false;
        }
    } else {
        return false;
    }

    if !rpm_offset(cs_player_pawn as u64, Offsets::C_BasePlayerPawn::m_pObserverServices as u64, &mut observer_services) {
        return false;
    }

    if observer_services != 0 {
        let mut observer_target: u32 = 0;
        let mut controller: usize = 0;
        
        if !rpm_offset(observer_services as u64, Offsets::CPlayer_ObserverServices::m_hObserverTarget as u64, &mut observer_target) {
            return false;
        }

        if let Some(sum) = (120 as u64).checked_mul(observer_target.bitand(0x1FF) as u64) {
            if !rpm_offset(game_entity_list_entry, sum, &mut controller) {
                return false;
            }
        } else {
            return false;
        }

        if controller as u64 == local_entity_pawn_address {
            return true;
        }
    }

    return false;
}

pub fn render_spectator_list(ui: &mut Ui, spectators: Vec<String>, config: Config) {
    let mut reset_position = SPECTATOR_LIST_RESET_POSITION.lock().unwrap();
    let (window_position, condition) = if let Some(position) = *reset_position {
        *reset_position = None;
        (position, imgui::Condition::Always)
    } else {
        (config.window_positions.spectator_list, imgui::Condition::Once)
    };

    drop(reset_position);

    ui.window("Spectators")
        .collapsible(false)
        .always_auto_resize(true)
        .position(window_position, condition)
        .build(|| {
            (*WINDOWS_ACTIVE.lock().unwrap()).insert("spectator_list".to_string(), ui.is_window_hovered());
            (*CONFIG.lock().unwrap()).window_positions.spectator_list = ui.window_pos();

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
}