use std::{sync::{Arc, Mutex}, path::PathBuf};

use colored::Colorize;
use imgui::{Ui, TabBar, TabItem};
use lazy_static::lazy_static;
use mint::Vector4;

use crate::utils::config::{CONFIG, CONFIG_DIR, CONFIGS, load_config, Config, delete_config, TOGGLE_KEY, PACKAGE_NAME, WindowPosition};
use crate::ui::functions::color_edit_u32_tuple;
use crate::ui::main::TOGGLED;

lazy_static! {
    static ref NEW_CONFIG_NAME: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
    static ref SELECTED_CONFIG: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
}

pub fn render_menu(ui: &mut Ui) {
    let toggle_key = *TOGGLE_KEY;
    let toggled = *TOGGLED.lock().unwrap();

    let mut config = CONFIG.lock().unwrap();
    let configs = CONFIGS.lock().unwrap().clone();
    let config_dir = CONFIG_DIR.lock().unwrap().clone();

    let mut new_config_name = NEW_CONFIG_NAME.lock().unwrap();
    let mut selected_config = SELECTED_CONFIG.lock().unwrap();

    let window_position = config.window_positions.menu;

    if let Some(config_name) = &*selected_config {
        if !(*configs).contains(config_name) {
            *selected_config = None;
        }
    };

    if toggled {
        ui.window(&*PACKAGE_NAME)
            .collapsible(false)
            .always_auto_resize(true)
            .position([window_position.x, window_position.y], imgui::Condition::Appearing)
            .build(|| {
                let window_pos = ui.window_pos();
                (*config).window_positions.menu = WindowPosition { x: window_pos[0], y: window_pos[1] };

                TabBar::new("Cheat").build(&ui, || {
                    // ESP
                    TabItem::new("ESP").build(&ui, || {
                        // Enabled
                        ui.checkbox("ESP", &mut (*config).esp.enabled);
                        
                        if (*config).esp.enabled {
                            ui.separator();

                            // Box
                            ui.checkbox("Box##ESP", &mut (*config).esp.box_enabled);
                            
                            if (*config).esp.box_enabled {
                                ui.same_line();
                                color_edit_u32_tuple(ui, "##ColorESPBox", &mut (*config).esp.box_color);
                                ui.same_line();
                                ui.combo_simple_string("##ModeESPBox", &mut (*config).esp.box_mode, &["Normal", "Dynamic"]);
                                
                                // Box Rounding
                                ui.slider_config("Rounding##ESPBox", 0, 25).display_format("%d").build(&mut (*config).esp.box_rounding);

                                // Box Target
                                ui.checkbox("Target##ESPBox", &mut (*config).esp.box_target_enabled);

                                if (*config).esp.box_target_enabled {
                                    ui.same_line();
                                    color_edit_u32_tuple(ui, "##TargetColorESPBox", &mut (*config).esp.box_target_color);
                                }

                                // Filled Box
                                ui.checkbox("Filled##ESPBox", &mut (*config).esp.filled_box_enabled);

                                if (*config).esp.filled_box_enabled {
                                    ui.same_line();
                                    color_edit_u32_tuple(ui, "##FilledColorESPBox", &mut (*config).esp.filled_box_color);

                                    // Filled Box Alpha
                                    ui.same_line();
                                    ui.slider_config("##AlphaESPBoxFilled", 0.1, 1.0).display_format("%.1f").build(&mut (*config).esp.filled_box_alpha);
                                }

                                ui.separator();
                            }

                            // Skeleton
                            ui.checkbox("Skeleton##ESP", &mut (*config).esp.skeleton_enabled);
                            
                            if (*config).esp.skeleton_enabled {
                                ui.same_line();
                                color_edit_u32_tuple(ui, "##ColorESPSkeleton", &mut (*config).esp.skeleton_color);
                            }

                            // Head
                            ui.checkbox("Head##ESP", &mut (*config).esp.head_enabled);
                            
                            if (*config).esp.head_enabled {
                                ui.same_line();
                                color_edit_u32_tuple(ui, "##ColorESPHead", &mut (*config).esp.head_color);
                                ui.same_line();
                                ui.combo_simple_string("##ModeESPMode", &mut (*config).esp.head_mode, &["Outline", "Filled"]);
                            }

                            // Eye
                            ui.checkbox("Eye##ESP", &mut (*config).esp.eye_ray_enabled);
                            
                            if (*config).esp.eye_ray_enabled {
                                ui.same_line();
                                color_edit_u32_tuple(ui, "##ColorESPEyeRay", &mut (*config).esp.eye_ray_color);
                            }

                            ui.separator();

                            // Health
                            ui.checkbox("Health##ESP", &mut (*config).esp.health_bar_enabled);
                            
                            if (*config).esp.health_bar_enabled {
                                ui.same_line();
                                color_edit_u32_tuple(ui, "##FirstColorESPHealth", &mut (*config).esp.health_bar_first_color);
                                ui.same_line();
                                color_edit_u32_tuple(ui, "##SecondColorESPHealth", &mut (*config).esp.health_bar_second_color);
                                ui.same_line();
                                color_edit_u32_tuple(ui, "##BarThirdColorESPHealth", &mut (*config).esp.health_bar_third_color);
                                ui.same_line();
                                ui.combo_simple_string("##ModeESPHealth", &mut (*config).esp.health_bar_mode, &["Vertical", "Horizontal"]);

                                // Health Rounding
                                ui.slider_config("Rounding##ESPHealth", 0, 25).display_format("%d").build(&mut (*config).esp.health_bar_rounding);
                            }

                            ui.separator();

                            // Player Name
                            ui.checkbox("Name##ESP", &mut (*config).esp.name_enabled);
                            
                            if (*config).esp.name_enabled {
                                ui.same_line();
                                color_edit_u32_tuple(ui, "##ColorESPPlayer", &mut (*config).esp.name_color);
                            }

                            // Weapon Name
                            ui.checkbox("Weapon##ESP", &mut (*config).esp.weapon_name_enabled);
                            
                            if (*config).esp.weapon_name_enabled {
                                ui.same_line();
                                color_edit_u32_tuple(ui, "##ColorESPWeapon", &mut (*config).esp.weapon_name_color);
                            }
                            
                            // Distance
                            ui.checkbox("Distance##ESP", &mut (*config).esp.distance_enabled);
                            
                            if (*config).esp.distance_enabled {
                                ui.same_line();
                                color_edit_u32_tuple(ui, "##ColorESPDistance", &mut (*config).esp.distance_color);
                            }

                            ui.separator();

                            // Bomb
                            ui.checkbox("Bomb##ESP", &mut (*config).esp.bomb_enabled);

                            if (*config).esp.bomb_enabled {
                                ui.same_line();
                                color_edit_u32_tuple(ui, "##ColorESPBomb", &mut (*config).esp.bomb_color);

                                // Bomb Rounding
                                ui.slider_config("Rounding##ESPBomb", 0, 25).display_format("%d").build(&mut (*config).esp.bomb_rounding);
								
                                // Filled Bomb
                                ui.checkbox("Filled##ESPBomb", &mut (*config).esp.filled_bomb_enabled);

                                if (*config).esp.filled_bomb_enabled {
                                    ui.same_line();
                                    color_edit_u32_tuple(ui, "##FilledColorESPBomb", &mut (*config).esp.filled_bomb_color);

                                    // Filled Bomb Alpha
                                    ui.same_line();
                                    ui.slider_config("##AlphaESPBombFilled", 0.1, 1.0).display_format("%.1f").build(&mut (*config).esp.filled_bomb_alpha);
                                }

                                ui.separator();
                            }

                            // Snap Line
                            ui.checkbox("Snapline##ESP", &mut (*config).esp.snap_line_enabled);
                            
                            if (*config).esp.snap_line_enabled {
                                ui.same_line();
                                color_edit_u32_tuple(ui, "##ColorESPSnapline", &mut (*config).esp.snap_line_color);
                                ui.same_line();
                                ui.combo_simple_string("##ModeESPSnapline", &mut (*config).esp.snap_line_mode, &["Top", "Center", "Bottom"]);
                            }
                        }
                    });

                    // AimBot
                    TabItem::new("Aimbot").build(&ui, || {
                        // Aimbot
                        ui.checkbox("Aimbot", &mut (*config).aimbot.enabled);

                        if (*config).aimbot.enabled {
                            // Aim Key
                            ui.same_line();
                            ui.combo_simple_string("##KeyAimbot", &mut (*config).aimbot.key, &["Alt", "Left Mouse", "Middle Mouse", "Right Mouse", "Shift", "Control"]);
                            ui.combo_simple_string("Mode##Aimbot", &mut (*config).aimbot.mode, &["Hold", "Toggle"]);
                            ui.separator();

                            // Circle
                            ui.checkbox("Circle##Aimbot", &mut (*config).aimbot.fov_circle_enabled);
                            
                            if (*config).aimbot.fov_circle_enabled {
                                ui.same_line();
                                color_edit_u32_tuple(ui, "##ColorAimbotCircle", &mut (*config).aimbot.fov_circle_color);

                                // Circle Target
                                ui.checkbox("Target##AimbotCircle", &mut (*config).aimbot.fov_circle_target_enabled);

                                if (*config).aimbot.fov_circle_target_enabled {
                                    ui.same_line();
                                    color_edit_u32_tuple(ui, "##ColorAimbotCircleTarget", &mut (*config).aimbot.fov_circle_target_color);
                                }

                                // Outline
                                ui.checkbox("Outline##Crosshair", &mut (*config).aimbot.fov_circle_outline_enabled);
                                ui.separator();
                            }

                            // Only Visible & Only Grounded
                            ui.checkbox("Only Visible##Aimbot", &mut (*config).aimbot.only_visible);
                            ui.checkbox("Only Grounded##Aimbot", &mut (*config).aimbot.only_grounded);
                            ui.separator();

                            // Bone, FOV, & Smooth
                            ui.combo_simple_string("Bone##Aimbot", &mut (*config).aimbot.bone, &["Head", "Neck", "Spine"]);
                            ui.slider_config("Fov##Aimbot", 0.5, 89.0).display_format("%.1f").build(&mut (*config).aimbot.fov);
                            ui.slider_config("Smooth##Aimbot", 0.0, 0.9).display_format("%.1f").build(&mut (*config).aimbot.smooth);
                            ui.separator();

                            // Start Bullet, Yaw, & Pitch
                            ui.slider_config("Start Bullet##Aimbot", 1, 6).display_format("%d").build(&mut (*config).aimbot.start_bullet);
                            ui.slider_config("Yaw##Aimbot", 0.0, 2.0).display_format("%.1f").build(&mut (*config).aimbot.rcs_yaw);
                            ui.slider_config("Pitch##Aimbot", 0.0, 2.0).display_format("%.1f").build(&mut (*config).aimbot.rcs_pitch);
                        }
                    });

                    // TriggerBot
                    TabItem::new("Triggerbot").build(&ui, || {
                        // Triggerbot
                        ui.checkbox("Triggerbot", &mut (*config).triggerbot.enabled);
                        
                        if (*config).triggerbot.enabled {
                            // Trigger Key
                            ui.same_line();
                            ui.combo_simple_string("##KeyTriggerbot", &mut (*config).triggerbot.key, &["Alt", "Left Mouse", "Middle Mouse", "Right Mouse", "Shift", "Control"]);
                            
                            // Mode
                            ui.combo_simple_string("Mode##Triggerbot", &mut (*config).triggerbot.mode, &["Tap", "Hold"]);

                            if (*config).triggerbot.mode == 0 {
                                // Interval
                                ui.slider_config("Interval##Triggerbot", 100, 500).display_format("%d").build(&mut (*config).triggerbot.tap_interval);
                            }

                            ui.separator();

                            // Always Activated
                            ui.checkbox("Always##Triggerbot", &mut (*config).triggerbot.always_activated);
                            ui.separator();

                            // Delay
                            ui.slider_config("Delay##Triggerbot", 15, 500).display_format("%d").build(&mut (*config).triggerbot.delay);
                        }
                    });

                    // Crosshair
                    TabItem::new("Crosshair").build(&ui, || {
                        // Crosshair
                        ui.checkbox("Crosshair", &mut (*config).crosshair.enabled);
                        
                        if (*config).crosshair.enabled {
                            // Crosshair Color
                            ui.same_line();
                            color_edit_u32_tuple(ui, "##ColorCrosshair", &mut (*config).crosshair.color);

                            // Target Crosshair
                            ui.checkbox("Target##Crosshair", &mut (*config).crosshair.target_enabled);
                            
                            if (*config).crosshair.target_enabled {
                                ui.same_line();
                                color_edit_u32_tuple(ui, "##ColorCrosshairTarget", &mut (*config).crosshair.target_color);
                            }

                            // Outline
                            ui.checkbox("Outline##Crosshair", &mut (*config).crosshair.outline_enabled);
                            ui.separator();

                            // Dot
                            ui.checkbox("Dot##Crosshair", &mut (*config).crosshair.dot_enabled);
                            
                            if (*config).crosshair.dot_enabled {
                                ui.same_line();
                                ui.slider_config("##SizeCrosshairDot", 1, 10).display_format("%d").build(&mut (*config).crosshair.dot_size);
                            }

                            // Circle
                            ui.checkbox("Circle##Crosshair", &mut (*config).crosshair.circle_enabled);

                            if (*config).crosshair.circle_enabled {
                                ui.same_line();
                                ui.slider_config("##RadiusCrosshairCircle", 1, 30).display_format("%d").build(&mut (*config).crosshair.circle_radius);
                            }

                            // Lines
                            ui.checkbox("Lines##Crosshair", &mut (*config).crosshair.lines_enabled);
                            
                            if (*config).crosshair.lines_enabled {
                                ui.slider_config("Width##CrosshairLines", 1, 20).display_format("%d").build(&mut (*config).crosshair.lines_width);
                                ui.slider_config("Height##CrosshairLines", 1, 20).display_format("%d").build(&mut (*config).crosshair.lines_height);
                                ui.slider_config("Space##CrosshairLines", 1, 10).display_format("%d").build(&mut (*config).crosshair.lines_space);
                                ui.slider_config("Thickness##CrosshairLines", 1, 10).display_format("%d").build(&mut (*config).crosshair.lines_thickness);
                            }
                        }
                    });

                    TabItem::new("Radar").build(&ui, || {
                        // Radar
                        ui.checkbox("Radar", &mut (*config).radar.enabled);
                        
                        if (*config).radar.enabled {
                            // Radar Type
                            ui.same_line();
                            color_edit_u32_tuple(ui, "##ColorRadar", &mut (*config).radar.color);
                            ui.same_line();
                            ui.combo_simple_string("##ModeRadar", &mut (*config).radar.mode, &["Circle", "Arrow", "Both"]);

                            // Radar Alpha
                            ui.slider_config("Alpha##Radar", 0.0, 1.0).display_format("%.1f").build(&mut (*config).radar.alpha);
                            ui.separator();

                            // Cross Line
                            ui.checkbox("Crossline##Radar", &mut (*config).radar.crossline_enabled);
                            
                            if (*config).radar.crossline_enabled {
                                ui.same_line();
                                color_edit_u32_tuple(ui, "##ColorRadarCrossline", &mut (*config).radar.crossline_color);
                            }

                            ui.separator();

                            // Point Size, Proportion, & Radar Range
                            ui.slider_config("Point Size##Radar", 1.0, 2.0).display_format("%.1f").build(&mut (*config).radar.point_size);
                            ui.slider_config("Proportion##Radar", 500.0, 3500.0).display_format("%.1f").build(&mut (*config).radar.proportion);
                            ui.slider_config("Range##Radar", 100.0, 300.0).display_format("%.1f").build(&mut (*config).radar.range);
                        }
                    });

                    TabItem::new("Misc").build(&ui, || {
                        // Misc
                        ui.checkbox("Misc", &mut (*config).misc.enabled);

                        if (*config).misc.enabled {
                            ui.separator();

                            // Watermark & Cheat List
                            ui.checkbox("Watermark##Misc", &mut (*config).misc.watermark_enabled);
                            ui.same_line();
                            ui.checkbox("Cheat List##Misc", &mut (*config).misc.cheat_list_enabled);
                            ui.separator();

                            // Bomb Timer
                            ui.checkbox("Bomb Timer##Misc", &mut (*config).misc.bomb_timer_enabled);

                            if (*config).misc.bomb_timer_enabled {
                                ui.same_line();
                                color_edit_u32_tuple(ui, "##ColorMiscBombTimerDisabled", &mut (*config).misc.bomb_timer_color_disabled);
                                ui.same_line();
                                color_edit_u32_tuple(ui, "##ColorMiscBombTimerEnabled", &mut (*config).misc.bomb_timer_color_enabled);
                            }

                            // Spectator List
                            ui.same_line();
                            ui.checkbox("Spectator List##Misc", &mut (*config).misc.spectator_list_enabled);

                            if (*config).misc.spectator_list_enabled {
                                ui.same_line();
                                color_edit_u32_tuple(ui, "##ColorMiscSpectatorList", &mut (*config).misc.spectator_list_color);
                            }

                            ui.separator();

                            // Exclude Team & Show on Spectate
                            ui.checkbox("Exclude Team##Misc", &mut (*config).misc.exclude_team);
                            ui.same_line();
                            ui.checkbox("Show On Spectate##Misc", &mut (*config).misc.show_on_spectate);
                            ui.separator();

                            // Bypass Capture & Headshot Line
                            ui.checkbox("Bypass Capture##Misc", &mut (*config).misc.bypass_capture);
                            ui.same_line();
                            ui.checkbox("Headshot Line##Misc", &mut (*config).misc.headshot_line_enabled);
                            
                            if (*config).misc.headshot_line_enabled {
                                ui.same_line();
                                color_edit_u32_tuple(ui, "##ColorMiscHeadshotLine", &mut (*config).misc.headshot_line_color);
                            }

                            ui.separator();

                            // Risky
                            ui.text_colored(Vector4 { x: 255.0, y: 0.0, z: 0.0, w: 255.0 }, "Risky");
                            ui.separator();

                            // No Flash & Bunny Hop
                            ui.checkbox("No Flash##Misc", &mut (*config).misc.no_flash_enabled);
                            ui.same_line();
                            ui.checkbox("Bunny Hop##Misc", &mut (*config).misc.bunny_hop_enabled);
                        }
                    });

                    TabItem::new("Config").build(&ui, || {
                        // New Config Input & Button
                        ui.input_text("##NameConfig", &mut *new_config_name).build();
                        ui.same_line();

                        if ui.button("Create##Config") {
                            let directory_pathbuf = PathBuf::from(&*config_dir);
                            
                            if let Some(config_path) = directory_pathbuf.join(format!("{}.conf.json", *new_config_name)).to_str() {
                                match (*config).save_config(config_path) {
                                    Err(str) => { println!("{} Failed to create new config: {} {}", "[ FAIL ]".bold().red(), format!("{}.conf.json", *new_config_name).bold(), format!("({})", str).bold()); },
                                    _ => {}
                                }
                            }
                        };
                        ui.separator();

                        // Current Configs
                        for config in &*configs {
                            if ui.selectable_config(config).build() {
                                *selected_config = Some(config.to_string());
                            };
                        };

                        ui.separator();

                        if let Some(config) = &*selected_config {
                            ui.text(format!("Selected: {}", config));
                            ui.separator();
                        };

                        if ui.button("Load##Config") {
                            if let Some(config_name) = &*selected_config {
                                let directory_pathbuf = PathBuf::from(&*config_dir);
                            
                                if let Some(config_path) = directory_pathbuf.join(config_name).to_str() {
                                    match load_config(config_path) {
                                        Ok(new_config) => { *config = new_config; },
                                        Err(str) => { println!("{} Failed to load config: {} {}", "[ FAIL ]".bold().red(), format!("{}", config_name).bold(), format!("({})", str).bold()); }
                                    }
                                }
                            };
                        };

                        ui.same_line();

                        if ui.button("Save##Config") {
                            if let Some(config_name) = &*selected_config {
                                let directory_pathbuf = PathBuf::from(&*config_dir);
                            
                                if let Some(config_path) = directory_pathbuf.join(config_name).to_str() {
                                    match (*config).save_config(config_path) {
                                        Err(str) => { println!("{} Failed to save config: {} {}", "[ FAIL ]".bold().red(), format!("{}", config_name).bold(), format!("({})", str).bold()); },
                                        _ => {}
                                    }
                                }
                            };
                        };

                        ui.same_line();

                        if ui.button("Delete##Config") {
                            if let Some(config_name) = &*selected_config {
                                if config_name != "default.conf.json" {
                                    let directory_pathbuf = PathBuf::from(&*config_dir);
                            
                                    if let Some(config_path) = directory_pathbuf.join(config_name).to_str() {
                                        match delete_config(config_path) {
                                            Err(str) => { println!("{} Failed to delete config: {} {}", "[ FAIL ]".bold().red(), format!("{}", config_name).bold(), format!("({})", str).bold()); },
                                            _ => {}
                                        }
                                    }
                                }
                            };
                        };

                        ui.separator();
                        
                        if ui.button("Reset##Config") {
                            *config = Config::default();
                        };
                    });
                });

                ui.separator();
                ui.text(format!("[{:?}] Toggle", toggle_key));
            });
    }
}