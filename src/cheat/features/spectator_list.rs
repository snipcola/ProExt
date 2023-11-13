use std::ops::BitAnd;
use imgui::Ui;
use mint::Vector4;

use crate::{utils::{process_manager::read_memory_auto, config::{CONFIG, WindowPosition, Config}}, ui::main::color_u32_to_f32};

pub fn is_spectating(entity_controller_address: u64, game_entity_list_entry: u64, local_entity_pawn_address: u64, entity_address: u64) -> bool {
    let mut m_h_pawn: u32 = 0;
    let mut p_cs_player_pawn: usize = 0;
    let mut m_p_observer_services: usize = 0;

    read_memory_auto(entity_controller_address + 0x5DC, &mut m_h_pawn);
    read_memory_auto(game_entity_list_entry + 120 * (m_h_pawn.bitand(0x1FF)) as u64, &mut p_cs_player_pawn);
    read_memory_auto(p_cs_player_pawn as u64 + 0x10C0, &mut m_p_observer_services);

    if m_p_observer_services != 0 {
        let mut m_h_observer_target: u32 = 0;
        let mut list_entry: usize = 0;
        let mut p_controller: usize = 0;
        
        read_memory_auto(m_p_observer_services as u64 + 0x44, &mut m_h_observer_target);
        read_memory_auto(entity_address + 0x8 * ((m_h_observer_target & 0x7FFF) >> 9) as u64 + 0x10, &mut list_entry);
        read_memory_auto(game_entity_list_entry + 120 * (m_h_observer_target.bitand(0x1FF)) as u64, &mut p_controller);

        if p_controller as u64 == local_entity_pawn_address {
            return true;
        }
    }

    return false;
}

pub fn render_spectator_list(ui: &mut Ui, spectators: Vec<String>, config: Config) {
    let window_position = config.window_positions.spectator_list;

    ui.window("Spectator List")
        .resizable(false)
        .collapsible(false)
        .scroll_bar(false)
        .title_bar(false)
        .always_auto_resize(true)
        .position([window_position.x, window_position.y], imgui::Condition::Appearing)
        .build(|| {
            let window_pos = ui.window_pos();
            let mut config_mut = CONFIG.lock().unwrap();
            (*config_mut).window_positions.spectator_list = WindowPosition { x: window_pos[0], y: window_pos[1] };
            drop(config_mut);

            let spectator_list_color_f32 = color_u32_to_f32(config.misc.spectator_list_color);
            let spectator_list_color = Vector4 { x: spectator_list_color_f32.0, y: spectator_list_color_f32.1, z: spectator_list_color_f32.2, w: spectator_list_color_f32.3 };

            ui.text("Spectator List");
            ui.separator();

            for spectator in spectators {
                ui.text_colored(spectator_list_color, spectator);
            }
        });
}