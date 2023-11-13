use std::sync::{Arc, Mutex};

use imgui::Ui;
use mint::Vector4;
use crate::utils::config::{CONFIG, WindowPosition, Config};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CHEAT_LIST_WINDOW_WIDTH: Arc<Mutex<f32>> = Arc::new(Mutex::new(0.0));
    pub static ref CHEAT_LIST_WINDOW_HEIGHT: Arc<Mutex<f32>> = Arc::new(Mutex::new(0.0));
}

pub fn render_cheat_list(ui: &mut Ui, config: Config, pawn: bool, aimbot_toggled: bool, triggerbot_toggled: bool, bunnyhop_toggled: bool) {
    let window_position = config.window_positions.cheat_list;

    ui.window("Cheat List")
        .resizable(false)
        .collapsible(false)
        .scroll_bar(false)
        .title_bar(false)
        .always_auto_resize(true)
        .position([window_position.x, window_position.y], imgui::Condition::FirstUseEver)
        .build(|| {
            let window_pos = ui.window_pos();
            let mut config_mut = CONFIG.lock().unwrap();
            (*config_mut).window_positions.cheat_list = WindowPosition { x: window_pos[0], y: window_pos[1] };
            drop(config_mut);

            let blue = Vector4 { x: 0.0, y: 255.0, z: 255.0, w: 255.0 };
            let green = Vector4 { x: 0.0, y: 255.0, z: 0.0, w: 255.0 };
            let red = Vector4 { x: 255.0, y: 0.0, z: 0.0, w: 255.0 };

            ui.text("Cheat List");

            if config.esp.enabled {
                ui.text_colored(blue, "ESP");

                if config.esp.box_enabled {
                    ui.text_colored(green, "- Box");
                }

                if config.esp.box_target_enabled {
                    ui.text_colored(green, "- Target");
                }

                if config.esp.filled_box_enabled {
                    ui.text_colored(green, "- Filled");
                }

                if config.esp.skeleton_enabled {
                    ui.text_colored(green, "- Skeleton");
                }

                if config.esp.head_enabled {
                    ui.text_colored(green, "- Head");
                }

                if config.esp.eye_ray_enabled {
                    ui.text_colored(green, "- Eye");
                }

                if config.esp.health_bar_enabled {
                    ui.text_colored(green, "- Health");
                }

                if config.esp.player_name_enabled {
                    ui.text_colored(green, "- Player");
                }

                if config.esp.weapon_name_enabled {
                    ui.text_colored(green, "- Weapon");
                }

                if pawn && config.esp.distance_enabled {
                    ui.text_colored(green, "- Distance");
                }

                if config.esp.snap_line_enabled {
                    ui.text_colored(green, "- Snapline");
                }
            }

            if pawn && config.aimbot.enabled {
                ui.text_colored(blue, "Aimbot");

                if aimbot_toggled {
                    ui.same_line();
                    ui.text_colored(green, "(Toggled)");
                }

                if config.aimbot.fov_circle_enabled {
                    ui.text_colored(green, "- Circle");
                }

                if config.aimbot.fov_circle_target_enabled {
                    ui.text_colored(green, "- Target");
                }

                if config.aimbot.fov_circle_outline_enabled {
                    ui.text_colored(green, "- Outline");
                }

                if config.aimbot.only_visible {
                    ui.text_colored(green, "- Only Visible");
                }

                if config.aimbot.only_grounded {
                    ui.text_colored(green, "- Only Grounded");
                }
            }

            if pawn && config.triggerbot.enabled {
                ui.text_colored(blue, "Triggerbot");

                if triggerbot_toggled {
                    ui.same_line();
                    ui.text_colored(green, "(Toggled)");
                }

                if config.triggerbot.always_activated {
                    ui.text_colored(green, "- Always Activated");
                }
            }

            if pawn && config.crosshair.enabled {
                ui.text_colored(blue, "Crosshair");

                if config.crosshair.target_enabled {
                    ui.text_colored(green, "- Target");
                }

                if config.crosshair.outline_enabled {
                    ui.text_colored(green, "- Outline");
                }

                if config.crosshair.dot_enabled {
                    ui.text_colored(green, "- Dot");
                }

                if config.crosshair.circle_enabled {
                    ui.text_colored(green, "- Circle");
                }

                if config.crosshair.lines_enabled {
                    ui.text_colored(green, "- Lines");
                }
            }

            if pawn && config.radar.enabled {
                ui.text_colored(blue, "Radar");

                if config.radar.crossline_enabled {
                    ui.text_colored(green, "- Crossline");
                }
            }

            if config.misc.enabled {
                ui.text_colored(blue, "Misc");

                if config.misc.watermark_enabled {
                    ui.text_colored(green, "- Watermark");
                }

                if config.misc.cheat_list_enabled {
                    ui.text_colored(green, "- Cheat List");
                }

                if config.misc.exclude_team {
                    ui.text_colored(green, "- Exclude Team");
                }

                if config.misc.show_on_spectate {
                    ui.text_colored(green, "- Show On Spectate");
                }

                if config.misc.bypass_capture {
                    ui.text_colored(green, "- Bypass Capture");
                }

                if pawn && config.misc.headshot_line_enabled {
                    ui.text_colored(green, "- Headshot Line");
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

                    if bunnyhop_toggled {
                        ui.same_line();
                        ui.text_colored(green, "(Toggled)");
                    }
                }
            }
        });
}