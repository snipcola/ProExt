use std::sync::{Arc, Mutex};
use imgui::Ui;
use mint::Vector4;
use lazy_static::lazy_static;
use crate::{utils::config::{CONFIG, Config}, ui::{main::WINDOWS_ACTIVE, functions::color_u32_to_f32}};

lazy_static! {
    pub static ref CHEAT_LIST_RESET_POSITION: Arc<Mutex<Option<[f32; 2]>>> = Arc::new(Mutex::new(None));
}

pub fn render_cheat_list(ui: &mut Ui, config: Config, pawn: bool, aimbot_toggled: bool, triggerbot_toggled: bool) {
    let mut reset_position = CHEAT_LIST_RESET_POSITION.lock().unwrap();
    let (window_position, condition) = if let Some(position) = *reset_position {
        *reset_position = None;
        (position, imgui::Condition::Always)
    } else {
        (config.window_positions.cheat_list, imgui::Condition::Once)
    };

    drop(reset_position);

    ui.window("Cheats")
        .collapsible(false)
        .always_auto_resize(true)
        .position(window_position, condition)
        .build(|| {
            (*WINDOWS_ACTIVE.lock().unwrap()).insert("cheat_list".to_string(), ui.is_window_hovered());
            (*CONFIG.lock().unwrap()).window_positions.cheat_list = ui.window_pos();

            let cheat_list_one_f32 = color_u32_to_f32(config.misc.cheat_list_color_one);
            let cheat_list_one_color = Vector4 { x: cheat_list_one_f32.0, y: cheat_list_one_f32.1, z: cheat_list_one_f32.2, w: cheat_list_one_f32.3 };

            let cheat_list_two_f32 = color_u32_to_f32(config.misc.cheat_list_color_two);
            let cheat_list_two_color = Vector4 { x: cheat_list_two_f32.0, y: cheat_list_two_f32.1, z: cheat_list_two_f32.2, w: cheat_list_two_f32.3 };

            if config.esp.enabled {
                ui.text_colored(cheat_list_one_color, "ESP");
            }

            if pawn && config.aimbot.enabled {
                ui.text_colored(cheat_list_one_color, "Aimbot");

                if aimbot_toggled {
                    ui.same_line();
                    ui.text_colored(cheat_list_two_color, "(Toggled)");
                }
            }

            if pawn && config.triggerbot.enabled {
                ui.text_colored(cheat_list_one_color, "Triggerbot");

                if triggerbot_toggled {
                    ui.same_line();
                    ui.text_colored(cheat_list_two_color, "(Toggled)");
                }
            }

            if pawn && config.crosshair.enabled {
                ui.text_colored(cheat_list_one_color, "Crosshair");
            }

            if pawn && config.radar.enabled {
                ui.text_colored(cheat_list_one_color, "Radar");
            }

            if config.misc.enabled {
                ui.text_colored(cheat_list_one_color, "Misc");

                if config.misc.exclude_team {
                    ui.text_colored(cheat_list_two_color, "- Exclude Team");
                }
            }

            if config.settings.enabled {
                ui.text_colored(cheat_list_one_color, "Settings");

                if config.settings.bypass_capture {
                    ui.text_colored(cheat_list_two_color, "- Bypass Capture");
                }

                if config.settings.discord_rpc_enabled {
                    ui.text_colored(cheat_list_two_color, "- Discord RPC");
                }
            }
        });
}