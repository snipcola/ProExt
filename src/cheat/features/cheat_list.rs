use std::sync::{Arc, Mutex};
use imgui::Ui;
use mint::Vector4;
use lazy_static::lazy_static;
use crate::{utils::config::{CONFIG, Config}, ui::main::WINDOWS_ACTIVE};

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

            let blue = Vector4 { x: 0.0, y: 255.0, z: 255.0, w: 255.0 };
            let green = Vector4 { x: 0.0, y: 255.0, z: 0.0, w: 255.0 };
            let red = Vector4 { x: 255.0, y: 0.0, z: 0.0, w: 255.0 };

            if config.esp.enabled {
                ui.text_colored(blue, "ESP");
            }

            if pawn && config.aimbot.enabled {
                ui.text_colored(blue, "Aimbot");

                if aimbot_toggled {
                    ui.same_line();
                    ui.text_colored(green, "(Toggled)");
                }
            }

            if pawn && config.triggerbot.enabled {
                ui.text_colored(blue, "Triggerbot");

                if triggerbot_toggled {
                    ui.same_line();
                    ui.text_colored(green, "(Toggled)");
                }
            }

            if pawn && config.crosshair.enabled {
                ui.text_colored(blue, "Crosshair");
            }

            if pawn && config.radar.enabled {
                ui.text_colored(blue, "Radar");
            }

            if config.misc.enabled {
                ui.text_colored(blue, "Misc");

                if config.misc.exclude_team {
                    ui.text_colored(green, "- Exclude Team");
                }

                if pawn && config.misc.no_flash_enabled {
                    ui.text_colored(green, "- No Flash");
                    ui.same_line();
                    ui.text_colored(red, "(Risky)");
                }

                if pawn && config.misc.bunny_hop_enabled {                    
                    ui.text_colored(green, "- Bunny Hop");                    
                    ui.same_line();
                    ui.text_colored(red, "(Risky)");
                }
            }

            if config.settings.enabled {
                ui.text_colored(blue, "Settings");

                if config.settings.bypass_capture {
                    ui.text_colored(green, "- Bypass Capture");
                }

                if config.settings.discord_rpc_enabled {
                    ui.text_colored(green, "- Discord RPC");
                }
            }
        });
}