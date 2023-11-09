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
        ui.window("Menu")
            .always_auto_resize(true)
            .build(|| {
                TabBar::new("Cheat").build(&ui, || {
                    // ESP
                    TabItem::new("ESP").build(&ui, || {
                        // BoxESP
                        ui.checkbox("BoxESP", &mut (*config).show_box_esp);
                        ui.same_line();
                        color_edit_u32_tuple(ui, "##BoxColor", &mut (*config).box_color);
                        ui.combo_simple_string("BoxType", &mut (*config).box_type, &["Normal", "Dynamic"]);
                        ui.separator();

                        // BoneESP
                        ui.checkbox("BoneESP", &mut (*config).show_bone_esp);
                        ui.same_line();
                        color_edit_u32_tuple(ui, "##BoneColor", &mut (*config).bone_color);
                        ui.separator();

                        // EyeRay
                        ui.checkbox("EyeRay", &mut (*config).show_eye_ray);
                        ui.same_line();
                        color_edit_u32_tuple(ui, "##EyeRay", &mut (*config).eye_ray_color);
                        ui.separator();

                        // HealthBar
                        ui.checkbox("HealthBar", &mut (*config).show_health_bar);
                        ui.combo_simple_string("HealthBarType", &mut (*config).health_bar_type, &["Vertical", "Horizontal"]);
                        ui.separator();

                        // WeaponText, Distance, & PlayerName
                        ui.checkbox("WeaponText", &mut (*config).show_weapon_esp);
                        ui.checkbox("Distance", &mut (*config).show_distance);
                        ui.checkbox("PlayerName", &mut (*config).show_player_name);
                        ui.separator();

                        // HeadShootLine
                        ui.checkbox("HeadShootLine", &mut (*config).show_head_shoot_line);
                        ui.same_line();
                        color_edit_u32_tuple(ui, "##HeadShootLineColor", &mut (*config).head_shoot_line_color);
                        ui.separator();

                        // FovLine
                        ui.checkbox("FovLine", &mut (*config).show_fov_line);
                        ui.same_line();
                        color_edit_u32_tuple(ui, "##FovLineColor", &mut (*config).fov_line_color);
                        ui.slider_config("FovLineSize", 20.0, 120.0).display_format("%.1f") .build(&mut (*config).fov_line_size);
                        ui.separator();

                        // LineToEnemy
                        ui.checkbox("LineToEnemy", &mut (*config).show_line_to_enemy);
                        ui.same_line();
                        color_edit_u32_tuple(ui, "##LineToEnemyColor", &mut (*config).line_to_enemy_color);
                        ui.separator();

                        // CrossHair
                        ui.checkbox("CrossHair", &mut (*config).show_crosshair);
                        ui.same_line();
                        color_edit_u32_tuple(ui, "##CrossHairColor", &mut (*config).crosshair_color);
                        ui.slider_config("CrossHairSize", 15.0, 200.0).display_format("%.1f").build(&mut (*config).crosshair_size);
                    });

                    // AimBot
                    TabItem::new("AimBot").build(&ui, || {
                        // AimBot & AimKey
                        ui.checkbox("AimBot", &mut (*config).aim_bot);
                        ui.combo_simple_string("AimKey", &mut (*config).aim_bot_hot_key, &["ALT", "LBUTTON", "MBUTTON", "RBUTTON", "SHIFT", "CONTROL"]);
                        ui.separator();

                        // AimFov & FovCircle
                        ui.slider_config("AimFov", 0.1, 89.0).display_format("%.1f").build(&mut (*config).aim_fov);
                        ui.checkbox("FovCircle", &mut (*config).show_aim_fov_range);
                        ui.same_line();
                        color_edit_u32_tuple(ui, "##FovCircleColor", &mut (*config).aim_fov_range_color);
                        ui.separator();

                        // Smooth
                        ui.slider_config("Smooth", 0.0, 0.9).display_format("%.1f").build(&mut (*config).smooth);
                        ui.separator();

                        // AimPos, & VisibleCheck
                        ui.combo_simple_string("AimPos", &mut (*config).aim_position, &["Head", "Neck", "Spine"]);
                        ui.checkbox("VisibleCheck", &mut (*config).visible_check);
                        ui.separator();

                        // Start Bullet, RCS Yaw, & RCS Pitch
                        ui.slider_config("Start Bullet", 1, 6).display_format("%d").build(&mut (*config).rcs_bullet);
                        ui.slider_config("RCS Yaw", 0.0, 2.0).display_format("%.1f").build(&mut (*config).rcs_scale.0);
                        ui.slider_config("RCS Pitch", 0.0, 2.0).display_format("%.1f").build(&mut (*config).rcs_scale.1);
                    });

                    // TriggerBot
                    TabItem::new("TriggerBot").build(&ui, || {
                        // TriggerBot, TriggerKey,
                        ui.checkbox("TriggerBot", &mut (*config).trigger_bot);
                        ui.combo_simple_string("TriggerKey", &mut (*config).trigger_hot_key, &["ALT", "LBUTTON", "MBUTTON", "RBUTTON", "SHIFT", "CONTROL"]);
                        ui.separator();

                        // TriggerMode
                        ui.combo_simple_string("TriggerMode", &mut (*config).trigger_mode, &["Hold", "Toggle"]);
                        ui.separator();

                        // Delay
                        ui.slider_config("Delay", 15, 170).display_format("%d").build(&mut (*config).trigger_delay);
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
                        ui.slider_config("PointSize", 0.8, 2.0).display_format("%.1f").build(&mut (*config).radar_point_size_proportion);
                        ui.slider_config("Proportion", 500.0, 3300.0).display_format("%.1f").build(&mut (*config).proportion);
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

                        // OBSBypass
                        ui.checkbox("OBSBypass", &mut (*config).obs_bypass);
                    });

                    TabItem::new("Config").build(&ui, || {
                        // New Config Input & Button
                        ui.input_text("New Config Name", &mut *new_config_name).build();
                        if ui.button("Create Config") {
                            let directory_pathbuf = PathBuf::from(&*config_dir);
                            
                            if let Some(config_path) = directory_pathbuf.join(format!("{}.conf.json", *new_config_name)).to_str() {
                                match (*config).save_config(config_path) {
                                    Ok(_) => { println!("{} Created new config: {}", "[ OKAY ]".bold().green(), format!("{}.conf.json", *new_config_name).bold()); },
                                    Err(str) => { println!("{} Failed to create new config: {} {}", "[ FAIL ]".bold().red(), format!("{}.conf.json", *new_config_name).bold(), format!("({})", str).bold()); }
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
                                        Ok(new_config) => {
                                            *config = new_config;
                                            println!("{} Loaded config: {}", "[ OKAY ]".bold().green(), format!("{}", config_name).bold());
                                        },
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
                                        Ok(_) => { println!("{} Saved config: {}", "[ OKAY ]".bold().green(), format!("{}", config_name).bold()); },
                                        Err(str) => { println!("{} Failed to save config: {} {}", "[ FAIL ]".bold().red(), format!("{}", config_name).bold(), format!("({})", str).bold()); }
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
                                        Ok(_) => { println!("{} Deleted config: {}", "[ OKAY ]".bold().green(), format!("{}", config_name).bold());},
                                        Err(str) => { println!("{} Failed to delete config: {} {}", "[ FAIL ]".bold().red(), format!("{}", config_name).bold(), format!("({})", str).bold()); }
                                    }
                                }
                            };
                        };

                        ui.separator();
                        
                        if ui.button("Reset to Default") {
                            let new_config = Config::default();
                            *config = new_config;
                            println!("{} Reset config to default", "[ OKAY ]".bold().green());
                        };
                    });
                });

                ui.separator();
                ui.text(format!("[{:?}] Toggle", toggle_key));
            });
    }
}