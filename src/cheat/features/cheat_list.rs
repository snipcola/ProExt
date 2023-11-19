use std::sync::{Arc, Mutex};

use imgui::Ui;
use mint::Vector4;
use crate::{utils::config::{CONFIG, WindowPosition, Config}, ui::main::WINDOWS_ACTIVE};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CHEAT_LIST_WINDOW_WIDTH: Arc<Mutex<f32>> = Arc::new(Mutex::new(0.0));
    pub static ref CHEAT_LIST_WINDOW_HEIGHT: Arc<Mutex<f32>> = Arc::new(Mutex::new(0.0));
}

pub fn render_cheat_list(ui: &mut Ui, config: Config, pawn: bool, aimbot_toggled: bool, triggerbot_toggled: bool) {
    let window_position = config.window_positions.cheat_list;

    ui.window("Cheats")
        .collapsible(false)
        .always_auto_resize(true)
        .position([window_position.x, window_position.y], imgui::Condition::Appearing)
        .build(|| {
            let window_active = ui.is_window_hovered();
            (*WINDOWS_ACTIVE.lock().unwrap()).insert("cheat_list".to_string(), window_active);

            let window_pos = ui.window_pos();
            let mut config_mut = CONFIG.lock().unwrap();
            (*config_mut).window_positions.cheat_list = WindowPosition { x: window_pos[0], y: window_pos[1] };
            drop(config_mut);

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

                if config.misc.bypass_capture {
                    ui.text_colored(green, "- Bypass Capture");
                }

                if config.misc.exclude_team {
                    ui.text_colored(green, "- Exclude Team");
                }

                if config.misc.discord_rpc_enabled {
                    ui.text_colored(green, "- Discord RPC");
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
        });
}