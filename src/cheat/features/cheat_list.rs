use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;

use imgui::Ui;
use mint::Vector4;

use crate::config::ProgramConfig;
use crate::utils::cheat::config::{CONFIG, Config};

use crate::ui::functions::color_u32_to_f32;

lazy_static! {
    pub static ref CHEAT_LIST_RESET_POSITION: Arc<Mutex<Option<[f32; 2]>>> = Arc::new(Mutex::new(None));
}

pub fn render_cheat_list(ui: &mut Ui, config: Config, pawn: bool, aimbot_toggled: bool, triggerbot_toggled: bool, rcs_toggled: bool, esp_toggled: bool, crosshair_toggled: bool, radar_toggled: bool) {
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
            (*CONFIG.lock().unwrap()).window_positions.cheat_list = ui.window_pos();

            let cheat_list_one_f32 = color_u32_to_f32(config.misc.cheat_list_color_one);
            let cheat_list_one_color = Vector4 { x: cheat_list_one_f32.0, y: cheat_list_one_f32.1, z: cheat_list_one_f32.2, w: cheat_list_one_f32.3 };

            let cheat_list_two_f32 = color_u32_to_f32(config.misc.cheat_list_color_two);
            let cheat_list_two_color = Vector4 { x: cheat_list_two_f32.0, y: cheat_list_two_f32.1, z: cheat_list_two_f32.2, w: cheat_list_two_f32.3 };

            if config.esp.enabled {
                ui.text_colored(cheat_list_one_color, "ESP");

                if !config.esp.always {
                    ui.same_line();
                    ui.text_colored(cheat_list_one_color, format!("({})", ProgramConfig::Keys::Available[config.esp.key]));
                }

                if esp_toggled {
                    ui.same_line();
                    ui.text_colored(cheat_list_two_color, "(Toggled)");
                }
            }

            if pawn && config.rcs.enabled {
                ui.text_colored(cheat_list_one_color, "RCS");

                if !config.rcs.always {
                    ui.same_line();
                    ui.text_colored(cheat_list_one_color, format!("({})", ProgramConfig::Keys::Available[config.rcs.key]));
                }

                if rcs_toggled {
                    ui.same_line();
                    ui.text_colored(cheat_list_two_color, "(Toggled)");
                }
            }

            if pawn && config.aimbot.enabled {
                ui.text_colored(cheat_list_one_color, "Aimbot");

                if !config.aimbot.always {
                    ui.same_line();
                    ui.text_colored(cheat_list_one_color, format!("({})", ProgramConfig::Keys::Available[config.aimbot.key]));
                }

                if aimbot_toggled {
                    ui.same_line();
                    ui.text_colored(cheat_list_two_color, "(Toggled)");
                }
            }

            if pawn && config.triggerbot.enabled {
                ui.text_colored(cheat_list_one_color, "Triggerbot");

                if !config.triggerbot.always {
                    ui.same_line();
                    ui.text_colored(cheat_list_one_color, format!("({})", ProgramConfig::Keys::Available[config.triggerbot.key]));
                }

                if triggerbot_toggled {
                    ui.same_line();
                    ui.text_colored(cheat_list_two_color, "(Toggled)");
                }
            }

            if config.crosshair.enabled {
                ui.text_colored(cheat_list_one_color, "Crosshair");

                if !config.crosshair.always {
                    ui.same_line();
                    ui.text_colored(cheat_list_one_color, format!("({})", ProgramConfig::Keys::Available[config.crosshair.key]));
                }

                if crosshair_toggled {
                    ui.same_line();
                    ui.text_colored(cheat_list_two_color, "(Toggled)");
                }
            }

            if config.radar.enabled {
                ui.text_colored(cheat_list_one_color, "Radar");

                if !config.radar.always {
                    ui.same_line();
                    ui.text_colored(cheat_list_one_color, format!("({})", ProgramConfig::Keys::Available[config.radar.key]));
                }

                if radar_toggled {
                    ui.same_line();
                    ui.text_colored(cheat_list_two_color, "(Toggled)");
                }
            }

            if config.settings.enabled {
                ui.text_colored(cheat_list_one_color, "Settings");

                if config.settings.bypass_capture {
                    ui.text_colored(cheat_list_two_color, "- Bypass Capture");
                }

                if config.settings.exclude_team {
                    ui.text_colored(cheat_list_two_color, "- Exclude Team");
                }
            }
        });
}