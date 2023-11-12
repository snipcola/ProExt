use std::{sync::{Arc, Mutex}, path::PathBuf};

use colored::Colorize;
use imgui::{Ui, TabBar, TabItem};
use lazy_static::lazy_static;

use crate::utils::config::{CONFIG, CONFIG_DIR, CONFIGS, load_config, Config, delete_config, TOGGLE_KEY};
use crate::ui::main::TOGGLED;
use crate::ui::main::color_edit_u32_tuple;

lazy_static! {
    static ref NEW_CONFIG_NAME: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
    static ref SELECTED_CONFIG: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
}

pub fn render_menu(ui: &mut Ui) {
    let toggle_key = *TOGGLE_KEY;
    let toggled = *TOGGLED.lock().unwrap();

    let mut config = CONFIG.lock().unwrap();
    let configs = CONFIGS.lock().unwrap();
    let config_dir = CONFIG_DIR.lock().unwrap();

    let mut new_config_name = NEW_CONFIG_NAME.lock().unwrap();
    let mut selected_config = SELECTED_CONFIG.lock().unwrap();

    if let Some(config_name) = &*selected_config {
        if !(*configs).contains(config_name) {
            *selected_config = None;
        }
    };

    if toggled {
        ui.window("CS2External")
            .collapsible(false)
            .always_auto_resize(true)
            .build(|| {
                TabBar::new("Cheat").build(&ui, || {
                    // ESP
                    TabItem::new("ESP").build(&ui, || {
                        // Enabled
                        ui.checkbox("ESP", &mut (*config).esp_enabled);
                        
                        if (*config).esp_enabled {
                            ui.separator();

                            // Box
                            ui.checkbox("Box", &mut (*config).show_box_esp);
                            ui.same_line();
                            color_edit_u32_tuple(ui, "##BoxColor", &mut (*config).box_color);
                            ui.same_line();
                            ui.combo_simple_string("##BoxType", &mut (*config).box_type, &["Normal", "Dynamic"]);
                            
                            if (*config).show_box_esp {
                                // Box Visible
                                ui.checkbox("Box Visible", &mut (*config).box_visible);
                                ui.same_line();
                                color_edit_u32_tuple(ui, "##BoxVisibleColor", &mut (*config).box_visible_color);
                                
                                // Box Rounding
                                ui.slider_config("Box Rounding", 0, 25).display_format("%d").build(&mut (*config).box_rounding);
                                ui.separator();
                            }

                            // Filled Box
                            ui.checkbox("Filled Box", &mut (*config).show_filled_box_esp);
                            ui.same_line();
                            color_edit_u32_tuple(ui, "##FilledBoxColor", &mut (*config).filled_box_color);

                            if (*config).show_filled_box_esp {
                                // Filled Box Alpha
                                ui.slider_config("Filled Box Alpha", 0.1, 1.0).display_format("%.1f").build(&mut (*config).filled_box_alpha);
                                ui.separator();
                            }

                            // Skeleton
                            ui.checkbox("Skeleton", &mut (*config).show_skeleton_esp);
                            ui.same_line();
                            color_edit_u32_tuple(ui, "##BoneColor", &mut (*config).skeleton_color);

                            // Head
                            ui.checkbox("Head", &mut (*config).show_head_esp);
                            ui.same_line();
                            color_edit_u32_tuple(ui, "##HeadColor", &mut (*config).head_color);
                            ui.same_line();
                            ui.combo_simple_string("##HeadType", &mut (*config).head_type, &["Outline", "Filled"]);

                            // Eye Ray
                            ui.checkbox("Eye Ray", &mut (*config).show_eye_ray);
                            ui.same_line();
                            color_edit_u32_tuple(ui, "##EyeRay", &mut (*config).eye_ray_color);
                            ui.separator();

                            // Health Bar
                            ui.checkbox("Health Bar", &mut (*config).show_health_bar);
                            ui.same_line();
                            ui.combo_simple_string("##HealthBarType", &mut (*config).health_bar_type, &["Vertical", "Horizontal"]);

                            if (*config).show_health_bar {
                                // Health Bar Rounding
                                ui.slider_config("Health Bar Rounding", 0, 25).display_format("%d").build(&mut (*config).health_bar_rounding);
                            }

                            ui.separator();

                            // Player Name
                            ui.checkbox("Player Name", &mut (*config).show_player_name);
                            ui.same_line();
                            color_edit_u32_tuple(ui, "##PlayerNameColor", &mut (*config).player_name_color);

                            // Weapon Name
                            ui.checkbox("Weapon Name", &mut (*config).show_weapon_esp);
                            ui.same_line();
                            color_edit_u32_tuple(ui, "##WeaponNameColor", &mut (*config).weapon_name_color);
                            
                            // Distance
                            ui.checkbox("Distance", &mut (*config).show_distance);
                            ui.same_line();
                            color_edit_u32_tuple(ui, "##DistanceColor", &mut (*config).distance_color);
                            ui.separator();

                            // Snap Line
                            ui.checkbox("Snap Line", &mut (*config).show_snap_line);
                            ui.same_line();
                            color_edit_u32_tuple(ui, "##LineToEnemyColor", &mut (*config).snap_line_color);
                        }
                    });

                    // AimBot
                    TabItem::new("Aimbot").build(&ui, || {
                        // Aimbot
                        ui.checkbox("Aimbot", &mut (*config).aim_bot);

                        if (*config).aim_bot {
                            // Aim Key
                            ui.same_line();
                            ui.combo_simple_string("##AimKey", &mut (*config).aim_bot_hot_key, &["Alt", "Left Mouse", "Middle Mouse", "Right Mouse", "Shift", "Control"]);
                            ui.combo_simple_string("Aimbot Mode", &mut (*config).aimbot_mode, &["Hold", "Toggle"]);
                            ui.separator();

                            // Fov & Fov Circle
                            ui.slider_config("Fov", 0.5, 89.0).display_format("%.1f").build(&mut (*config).aim_fov);
                            ui.checkbox("Fov Circle", &mut (*config).show_aim_fov_range);
                            ui.same_line();
                            color_edit_u32_tuple(ui, "##FovCircleColor", &mut (*config).aim_fov_range_color);
                            ui.separator();

                            // Only Visible & Only Grounded
                            ui.checkbox("Only Visible", &mut (*config).visible_check);
                            ui.checkbox("Only Grounded", &mut (*config).ground_check);
                            ui.separator();

                            // Fov, Smooth, Bone
                            ui.combo_simple_string("Bone", &mut (*config).aim_position, &["Head", "Neck", "Spine"]);
                            ui.slider_config("Smooth", 0.0, 0.9).display_format("%.1f").build(&mut (*config).smooth);
                            ui.separator();

                            // Start Bullet, RCS Yaw, & RCS Pitch
                            ui.slider_config("Start Bullet", 1, 6).display_format("%d").build(&mut (*config).rcs_bullet);
                            ui.slider_config("RCS Yaw", 0.0, 2.0).display_format("%.1f").build(&mut (*config).rcs_scale.0);
                            ui.slider_config("RCS Pitch", 0.0, 2.0).display_format("%.1f").build(&mut (*config).rcs_scale.1);
                        }
                    });

                    // TriggerBot
                    TabItem::new("TriggerBot").build(&ui, || {
                        // TriggerBot, TriggerKey,
                        ui.checkbox("TriggerBot", &mut (*config).trigger_bot);
                        ui.combo_simple_string("TriggerKey", &mut (*config).triggerbot_hot_key, &["Alt", "Left Mouse", "Middle Mouse", "Right Mouse", "Shift", "Control"]);
                        ui.separator();

                        // TriggerMode
                        ui.combo_simple_string("TriggerMode", &mut (*config).triggerbot_mode, &["Hold", "Toggle"]);
                        ui.separator();

                        // Delay
                        ui.slider_config("Delay", 15, 500).display_format("%d").build(&mut (*config).trigger_delay);
                    });

                    TabItem::new("Radar").build(&ui, || {
                        // Radar & RadarType
                        ui.checkbox("Radar", &mut (*config).show_radar);
                        ui.combo_simple_string("RadarType", &mut (*config).radar_type, &["Circle", "Arrow", "CircleWithArrow"]);
                        ui.separator();

                        // CrossLine
                        ui.checkbox("CrossLine", &mut (*config).show_radar_cross_line);
                        ui.same_line();
                        color_edit_u32_tuple(ui, "##CrossLineColor", &mut (*config).radar_cross_line_color);

                        // PointSize, Proportion, & RadarRange
                        ui.slider_config("PointSize", 1.0, 2.0).display_format("%.1f").build(&mut (*config).radar_point_size_proportion);
                        ui.slider_config("Proportion", 500.0, 3500.0).display_format("%.1f").build(&mut (*config).proportion);
                        ui.slider_config("RadarRange", 100.0, 300.0).display_format("%.1f").build(&mut (*config).radar_range);
                    });

                    TabItem::new("Misc").build(&ui, || {
                        // AntiFlashbang & BunnyHop
                        ui.checkbox("AntiFlashbang", &mut (*config).anti_flashbang);
                        ui.same_line();
                        ui.checkbox("BunnyHop", &mut (*config).bunny_hop);
                        ui.separator();

                        // TeamCheck & ShowWhenSpec
                        ui.checkbox("TeamCheck", &mut (*config).team_check);
                        ui.same_line();
                        ui.checkbox("ShowWhenSpec", &mut (*config).show_when_spec);
                        ui.separator();

                        // OBSBypass & Headshot Line
                        ui.checkbox("OBSBypass", &mut (*config).obs_bypass);
                        ui.same_line();
                        ui.checkbox("Headshot Line", &mut (*config).show_head_shot_line);
                        ui.same_line();
                        color_edit_u32_tuple(ui, "##HeadshotLineColor", &mut (*config).head_shot_line_color);
                    });

                    TabItem::new("Config").build(&ui, || {
                        // New Config Input & Button
                        ui.input_text("New Config Name", &mut *new_config_name).build();
                        if ui.button("Create Config") {
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
                            ui.text(format!("Selected Config: {}", config));
                            ui.separator();
                        };

                        if ui.button("Load Selected") {
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

                        if ui.button("Save Selected") {
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

                        // Destructive Actions
                        if ui.button("Delete Selected") {
                            if let Some(config_name) = &*selected_config {
                                if config_name == "default.conf.json" {
                                    return;
                                }

                                let directory_pathbuf = PathBuf::from(&*config_dir);
                            
                                if let Some(config_path) = directory_pathbuf.join(config_name).to_str() {
                                    match delete_config(config_path) {
                                        Err(str) => { println!("{} Failed to delete config: {} {}", "[ FAIL ]".bold().red(), format!("{}", config_name).bold(), format!("({})", str).bold()); },
                                        _ => {}
                                    }
                                }
                            };
                        };

                        ui.separator();
                        
                        if ui.button("Reset to Default") {
                            *config = Config::default();
                        };
                    });
                });

                ui.separator();
                ui.text(format!("[{:?}] Toggle", toggle_key));
            });
    }
}