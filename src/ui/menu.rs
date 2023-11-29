use std::{sync::{Arc, Mutex}, path::PathBuf};
use imgui::{Ui, TabBar, TabItem, WindowHoveredFlags};
use lazy_static::lazy_static;

use crate::utils::{config::{CONFIG, CONFIG_DIR, CONFIGS, Config, delete_config, ProgramConfig, DEFAULT_CONFIG, CONFIG_EXTENSION}, open::open_url, messagebox::{MessageBoxStyle, create_messagebox}};
use crate::ui::functions::color_edit_u32_tuple;
use crate::ui::main::WINDOWS_ACTIVE;
use crate::ui::functions::reset_window_positions;

lazy_static! {
    static ref NEW_CONFIG_NAME: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
    static ref LOADED_CONFIG: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(DEFAULT_CONFIG.clone())));
    pub static ref MENU_RESET_POSITION: Arc<Mutex<Option<[f32; 2]>>> = Arc::new(Mutex::new(None));
}

pub fn render_menu(ui: &mut Ui) {
    let mut config = CONFIG.lock().unwrap();
    let mut configs = CONFIGS.lock().unwrap().clone();
    let config_dir = CONFIG_DIR.lock().unwrap().clone();

    let mut new_config_name = NEW_CONFIG_NAME.lock().unwrap();
    let loaded_config = LOADED_CONFIG.clone();

    let mut reset_position = MENU_RESET_POSITION.lock().unwrap();
    let (window_position, condition) = if let Some(position) = *reset_position {
        *reset_position = None;
        (position, imgui::Condition::Always)
    } else {
        (config.window_positions.menu, imgui::Condition::Once)
    };

    drop(reset_position);

    ui.window("Menu")
        .collapsible(false)
        .always_auto_resize(true)
        .focus_on_appearing(true)
        .position(window_position, condition)
        .build(|| {
            let window_active = ui.is_window_hovered_with_flags(WindowHoveredFlags::ALLOW_WHEN_BLOCKED_BY_ACTIVE_ITEM | WindowHoveredFlags::ALLOW_WHEN_BLOCKED_BY_POPUP) || ui.is_any_item_hovered();
            (*WINDOWS_ACTIVE.lock().unwrap()).insert("menu".to_string(), window_active);

            let window_pos = ui.window_pos();
            (*config).window_positions.menu = window_pos;

            TabBar::new("Cheat").build(&ui, || {
                // ESP
                TabItem::new("ESP").build(&ui, || {
                    // Enabled
                    ui.checkbox("ESP", &mut (*config).esp.enabled);
                    
                    if (*config).esp.enabled {
                        // Outline
                        ui.checkbox("Outline##ESP", &mut (*config).esp.outline);

                        // Thickness
                        ui.slider_config("Thickness##ESP", 0.5, 5.0).display_format("%.1f").build(&mut (*config).esp.thickness);

                        // Rounding
                        ui.slider_config("Rounding##ESP", 0, 15).display_format("%d").build(&mut (*config).esp.rounding);
                        ui.separator();

                        // Box
                        ui.checkbox("Box##ESP", &mut (*config).esp.box_enabled);
                        
                        if (*config).esp.box_enabled {
                            ui.same_line();
                            color_edit_u32_tuple(ui, "##ColorESPBox", &mut (*config).esp.box_color);
                            ui.same_line();
                            ui.combo_simple_string("##ModeESPBox", &mut (*config).esp.box_mode, &["Normal", "Dynamic"]);

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
                                color_edit_u32_tuple(ui, "##FilledColorOneESPBox", &mut (*config).esp.filled_box_color_one);
                                
                                if (*config).esp.rounding <= 0 {
                                    ui.same_line();
                                    color_edit_u32_tuple(ui, "##FilledColorTwoESPBox", &mut (*config).esp.filled_box_color_two);
                                }

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
                        }

                        // Armor
                        ui.checkbox("Armor##ESP", &mut (*config).esp.armor_bar_enabled);

                        if (*config).esp.armor_bar_enabled {
                            ui.same_line();
                            color_edit_u32_tuple(ui, "##ColorESPArmor", &mut (*config).esp.armor_bar_color);
                        }

                        // Bar Mode
                        ui.combo_simple_string("Bar Mode##ESPBar", &mut (*config).esp.bar_mode, &["Vertical", "Horizontal"]);
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

                // Aimbot
                TabItem::new("Aimbot").build(&ui, || {
                    // Enabled
                    ui.checkbox("Aimbot", &mut (*config).aimbot.enabled);

                    if (*config).aimbot.enabled {
                        // Aim Key
                        if !(*config).aimbot.always {
                            ui.same_line();
                            ui.combo_simple_string("##KeyAimbot", &mut (*config).aimbot.key, &["Alt", "Left Mouse", "Middle Mouse", "Right Mouse", "Side Mouse", "Extra Mouse", "Shift", "Control"]);
                        }

                        // Mode
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
                            ui.checkbox("Outline##AimbotCircle", &mut (*config).aimbot.fov_circle_outline_enabled);

                            // Thickness
                            ui.slider_config("Thickness##AimbotCircle", 0.5, 5.0).display_format("%.1f").build(&mut (*config).aimbot.fov_circle_thickness);
                            ui.separator();
                        }

                        // Only Visible, Only Grounded, & Only Weapon
                        ui.checkbox("Always##Aimbot", &mut (*config).aimbot.always);
                        ui.checkbox("Only Visible##Aimbot", &mut (*config).aimbot.only_visible);
                        ui.checkbox("Only Grounded##Aimbot", &mut (*config).aimbot.only_grounded);
                        ui.checkbox("Only Weapon##Aimbot", &mut (*config).aimbot.only_weapon);
                        ui.separator();

                        // Bone, FOV
                        ui.combo_simple_string("Bone##Aimbot", &mut (*config).aimbot.bone, &["Head", "Neck", "Spine"]);
                        ui.slider_config("Fov##Aimbot", 0.5, 89.0).display_format("%.1f").build(&mut (*config).aimbot.fov);
                        ui.separator();

                        // Smooth
                        ui.slider_config("Smooth##Aimbot", 0.0, 5.0).display_format("%.1f").build(&mut (*config).aimbot.smooth);
                        ui.slider_config("Smooth Offset##Aimbot", 0.0, 1.0).display_format("%.1f").build(&mut (*config).aimbot.smooth_offset);
                        ui.separator();

                        // Delay
                        ui.slider_config("Delay##Aimbot", 0, 500).display_format("%d").build(&mut (*config).aimbot.delay);
                        ui.slider_config("Delay Offset##Aimbot", 0, 100).display_format("%d").build(&mut (*config).aimbot.delay_offset);
                    }
                });

                // Triggerbot
                TabItem::new("Triggerbot").build(&ui, || {
                    // Enabled
                    ui.checkbox("Triggerbot", &mut (*config).triggerbot.enabled);
                    
                    if (*config).triggerbot.enabled {
                        // Trigger Key
                        if !(*config).triggerbot.always {
                            ui.same_line();
                            ui.combo_simple_string("##KeyTriggerbot", &mut (*config).triggerbot.key, &["Alt", "Left Mouse", "Middle Mouse", "Right Mouse", "Side Mouse", "Extra Mouse", "Shift", "Control"]);
                        }

                        // Mode
                        ui.combo_simple_string("Mode##Triggerbot", &mut (*config).triggerbot.mode, &["Hold", "Toggle"]);
                        
                        // Action
                        ui.combo_simple_string("Action##Triggerbot", &mut (*config).triggerbot.action, &["Click", "Press"]);

                        if (*config).triggerbot.action == 0 {
                            // Interval
                            ui.separator();
                            ui.slider_config("Interval##Triggerbot", 50, 500).display_format("%d").build(&mut (*config).triggerbot.tap_interval);
                            ui.slider_config("Interval Offset##Triggerbot", 0, 100).display_format("%d").build(&mut (*config).triggerbot.tap_interval_offset);
                        }

                        ui.separator();

                        // Delay
                        ui.slider_config("Delay##Triggerbot", 0, 500).display_format("%d").build(&mut (*config).triggerbot.delay);
                        ui.slider_config("Delay Offset##Triggerbot", 0, 100).display_format("%d").build(&mut (*config).triggerbot.delay_offset);
                        ui.separator();

                        // Always Activated
                        ui.checkbox("Always##Triggerbot", &mut (*config).triggerbot.always);

                        // Only Weapon
                        ui.checkbox("Only Weapon##Triggerbot", &mut (*config).triggerbot.only_weapon);
                    }
                });

                // Crosshair
                TabItem::new("Crosshair").build(&ui, || {
                    // Enabled
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

                        ui.separator();

                        // Only Weapon
                        ui.checkbox("Only Weapon##Crosshair", &mut (*config).crosshair.only_weapon);
                    }
                });

                // Radar
                TabItem::new("Radar").build(&ui, || {
                    // Enabled
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

                        // Radar Outline
                        ui.checkbox("Outline##Radar", &mut (*config).radar.outline);

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

                // Misc
                TabItem::new("Misc").build(&ui, || {
                    // Enabled
                    ui.checkbox("Misc", &mut (*config).misc.enabled);

                    if (*config).misc.enabled {
                        ui.separator();

                        // Watermark
                        ui.checkbox("Watermark##Misc", &mut (*config).misc.watermark_enabled);

                        if (*config).misc.watermark_enabled {
                            ui.same_line();
                            color_edit_u32_tuple(ui, "##ColorMiscWatermarkOne", &mut (*config).misc.watermark_color_one);
                            ui.same_line();
                            color_edit_u32_tuple(ui, "##ColorMiscWatermarkTwo", &mut (*config).misc.watermark_color_two);
                        }

                        // Cheat List
                        ui.same_line();
                        ui.checkbox("Cheat List##Misc", &mut (*config).misc.cheat_list_enabled);

                        if (*config).misc.cheat_list_enabled {
                            ui.same_line();
                            color_edit_u32_tuple(ui, "##ColorMiscCheatListOne", &mut (*config).misc.cheat_list_color_one);
                            ui.same_line();
                            color_edit_u32_tuple(ui, "##ColorMiscCheatListTwo", &mut (*config).misc.cheat_list_color_two);
                        }

                        ui.separator();

                        // Bomb Timer & Spectator List
                        ui.checkbox("Bomb Timer##Misc", &mut (*config).misc.bomb_timer_enabled);

                        if (*config).misc.bomb_timer_enabled {
                            ui.same_line();
                            color_edit_u32_tuple(ui, "##ColorMiscBombTimerDisabled", &mut (*config).misc.bomb_timer_color_disabled);
                            ui.same_line();
                            color_edit_u32_tuple(ui, "##ColorMiscBombTimerEnabled", &mut (*config).misc.bomb_timer_color_enabled);
                        }

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

                        // Headshot Line
                        ui.checkbox("Headshot Line##Misc", &mut (*config).misc.headshot_line_enabled);
                        
                        if (*config).misc.headshot_line_enabled {
                            ui.same_line();
                            color_edit_u32_tuple(ui, "##ColorMiscHeadshotLine", &mut (*config).misc.headshot_line_color);
                        }
                    }
                });

                // Style
                TabItem::new("Style").build(&ui, || {
                    // Enabled
                    ui.checkbox("Style", &mut (*config).style.enabled);
                    
                    if (*config).style.enabled {
                        // Alpha
                        ui.same_line();
                        ui.slider_config("Alpha##Style", 0.2, 1.0).display_format("%.1f").build(&mut (*config).style.alpha);
                        ui.separator();

                        // Window
                        ui.slider_config("Window Padding X##Style", 0.0, 50.0).display_format("%.1f").build(&mut (*config).style.window_padding[0]);
                        ui.same_line();
                        ui.slider_config("Window Padding Y##Style", 0.0, 50.0).display_format("%.1f").build(&mut (*config).style.window_padding[1]);
                        ui.slider_config("Window Rounding##Style", 0.0, 25.0).display_format("%.1f").build(&mut (*config).style.window_rounding);
                        ui.same_line();
                        ui.slider_config("Window Border Size##Style", 0.0, 10.0).display_format("%.1f").build(&mut (*config).style.window_border_size);
                        ui.slider_config("Window Title Align X##Style", 0.0, 1.0).display_format("%.1f").build(&mut (*config).style.window_title_align[0]);
                        ui.same_line();
                        ui.slider_config("Window Title Align Y##Style", 0.0, 1.0).display_format("%.1f").build(&mut (*config).style.window_title_align[1]);
                        ui.separator();

                        // Frame
                        ui.slider_config("Frame Padding X##Style", 0.0, 50.0).display_format("%.1f").build(&mut (*config).style.frame_padding[0]);
                        ui.same_line();
                        ui.slider_config("Frame Padding Y##Style", 0.0, 50.0).display_format("%.1f").build(&mut (*config).style.frame_padding[1]);
                        ui.slider_config("Frame Rounding##Style", 0.0, 25.0).display_format("%.1f").build(&mut (*config).style.frame_rounding);
                        ui.same_line();
                        ui.slider_config("Frame Border Size##Style", 0.0, 10.0).display_format("%.1f").build(&mut (*config).style.frame_border_size);
                        ui.separator();

                        // Tab
                        ui.slider_config("Tab Rounding##Style", 0.0, 25.0).display_format("%.1f").build(&mut (*config).style.tab_rounding);
                        ui.same_line();
                        ui.slider_config("Tab Border Size##Style", 0.0, 10.0).display_format("%.1f").build(&mut (*config).style.tab_border_size);
                        ui.separator();

                        // Scrollbar
                        ui.slider_config("Scrollbar Rounding##Style", 0.0, 25.0).display_format("%.1f").build(&mut (*config).style.scrollbar_rounding);
                        ui.same_line();
                        ui.slider_config("Scrollbar Size##Style", 0.0, 15.0).display_format("%.1f").build(&mut (*config).style.scrollbar_size);
                        ui.separator();

                        // Popup
                        ui.slider_config("Popup Rounding##Style", 0.0, 25.0).display_format("%.1f").build(&mut (*config).style.popup_rounding);
                        ui.same_line();
                        ui.slider_config("Popup Border Size##Style", 0.0, 10.0).display_format("%.1f").build(&mut (*config).style.popup_border_size);
                        ui.separator();

                        // Item
                        ui.slider_config("Item Spacing X##Style", 0.0, 50.0).display_format("%.1f").build(&mut (*config).style.item_spacing[0]);
                        ui.same_line();
                        ui.slider_config("Item Spacing Y##Style", 0.0, 50.0).display_format("%.1f").build(&mut (*config).style.item_spacing[1]);
                        ui.slider_config("Item Inner Spacing X##Style", 0.0, 50.0).display_format("%.1f").build(&mut (*config).style.item_inner_spacing[0]);
                        ui.same_line();
                        ui.slider_config("Item Inner Spacing Y##Style", 0.0, 50.0).display_format("%.1f").build(&mut (*config).style.item_inner_spacing[1]);
                        ui.separator();

                        // Indent
                        ui.slider_config("Indent Spacing##Style", 0.0, 25.0).display_format("%.1f").build(&mut (*config).style.indent_spacing);
                        ui.same_line();

                        // Grab
                        ui.slider_config("Grab Rounding##Style", 0.0, 25.0).display_format("%.1f").build(&mut (*config).style.grab_rounding);
                        ui.separator();

                        // Colors
                        color_edit_u32_tuple(ui, "Text##Style", &mut (*config).style.colors.text);
                        ui.same_line();
                        color_edit_u32_tuple(ui, "Text Disabled##Style", &mut (*config).style.colors.text_disabled);
                        ui.same_line();
                        color_edit_u32_tuple(ui, "Window BG##Style", &mut (*config).style.colors.window_bg);
                        ui.same_line();
                        color_edit_u32_tuple(ui, "Child Window BG##Style", &mut (*config).style.colors.child_bg);
                        ui.same_line();
                        color_edit_u32_tuple(ui, "Popup BG##Style", &mut (*config).style.colors.popup_bg);
                        ui.same_line();
                        color_edit_u32_tuple(ui, "Border##Style", &mut (*config).style.colors.border);
                        ui.same_line();
                        color_edit_u32_tuple(ui, "Border Shadow##Style", &mut (*config).style.colors.border_shadow);
                        ui.same_line();
                        color_edit_u32_tuple(ui, "Frame BG##Style", &mut (*config).style.colors.frame_bg);

                        color_edit_u32_tuple(ui, "Frame BG Hovered##Style", &mut (*config).style.colors.frame_bg_hovered);
                        ui.same_line();
                        color_edit_u32_tuple(ui, "Frame BG Active##Style", &mut (*config).style.colors.frame_bg_active);
                        ui.same_line();
                        color_edit_u32_tuple(ui, "Title BG##Style", &mut (*config).style.colors.title_bg);
                        ui.same_line();
                        color_edit_u32_tuple(ui, "Title BG Collapsed##Style", &mut (*config).style.colors.title_bg_collapsed);
                        ui.same_line();
                        color_edit_u32_tuple(ui, "Title BG Active##Style", &mut (*config).style.colors.title_bg_active);
                        ui.same_line();
                        color_edit_u32_tuple(ui, "Text Selected BG##Style", &mut (*config).style.colors.text_selected_bg);

                        color_edit_u32_tuple(ui, "Checkmark##Style", &mut (*config).style.colors.checkmark);
                        ui.same_line();
                        color_edit_u32_tuple(ui, "Scrollbar BG##Style", &mut (*config).style.colors.scrollbar_bg);
                        ui.same_line();
                        color_edit_u32_tuple(ui, "Scrollbar Grab##Style", &mut (*config).style.colors.scrollbar_grab);
                        ui.same_line();
                        color_edit_u32_tuple(ui, "Scrollbar Grab Hovered##Style", &mut (*config).style.colors.scrollbar_grab_hovered);
                        ui.same_line();
                        color_edit_u32_tuple(ui, "Scrollbar Grab Active##Style", &mut (*config).style.colors.scrollbar_grab_active);
                        ui.same_line();
                        color_edit_u32_tuple(ui, "Slider Grab##Style", &mut (*config).style.colors.slider_grab);

                        color_edit_u32_tuple(ui, "Slider Grab Active##Style", &mut (*config).style.colors.slider_grab_active);
                        ui.same_line();
                        color_edit_u32_tuple(ui, "Button##Style", &mut (*config).style.colors.button);
                        ui.same_line();
                        color_edit_u32_tuple(ui, "Button Hovered##Style", &mut (*config).style.colors.button_hovered);
                        ui.same_line();
                        color_edit_u32_tuple(ui, "Button Active##Style", &mut (*config).style.colors.button_active);
                        ui.same_line();
                        color_edit_u32_tuple(ui, "Tab##Style", &mut (*config).style.colors.tab);
                        ui.same_line();
                        color_edit_u32_tuple(ui, "Tab Hovered##Style", &mut (*config).style.colors.tab_hovered);
                        ui.same_line();
                        color_edit_u32_tuple(ui, "Tab Active##Style", &mut (*config).style.colors.tab_active);
                        ui.same_line();
                        color_edit_u32_tuple(ui, "Separator##Style", &mut (*config).style.colors.separator);
                    }
                });

                // Settings
                TabItem::new("Settings").build(&ui, || {
                    // Enabled
                    ui.checkbox("Settings", &mut (*config).settings.enabled);

                    if (*config).settings.enabled {
                        ui.separator();

                        // Bypass Capture & Discord RPC
                        ui.checkbox("Bypass Capture##Settings", &mut (*config).settings.bypass_capture);
                        ui.checkbox("Discord RPC##Settings", &mut (*config).settings.discord_rpc_enabled);
                    }

                    // Config
                    ui.separator();
                    
                    // Config Input & Create Button
                    ui.input_text("##NameConfig", &mut *new_config_name).build();
                    ui.same_line();

                    if ui.button("Create##Config") {
                        if *new_config_name != "" {
                            let new_config_path = format!("{}.{}", *new_config_name, *CONFIG_EXTENSION);
                            let directory_pathbuf = PathBuf::from(&*config_dir);
                            let new_config = config.clone();
                            
                            if let Some(config_path) = directory_pathbuf.join(new_config_path.clone()).to_str() {
                                match new_config.save_config(config_path, true) {
                                    Err(error) => create_messagebox(MessageBoxStyle::Error, "Error", &format!("Failed to create new config: {} ({}).", new_config_path, error)),
                                    Ok(_) => {
                                        *new_config_name = "".to_string();
                                        *config = new_config;
                                        *loaded_config.lock().unwrap() = Some(new_config_path);
                                        configs = CONFIGS.lock().unwrap().clone();
                                    }
                                }
                            }
                        }
                    }

                    ui.separator();

                    // Loaded Config
                    let loaded_conf = loaded_config.lock().unwrap().clone();

                    // Current Configs
                    for (config_name, config_item) in configs.clone() {
                        if let Some(config_item) = config_item {
                            let mut loaded = false;

                            if let Some(loaded_config) = &loaded_conf {
                                if &config_name == loaded_config {
                                    loaded = true;
                                }
                            }

                            if ui.selectable_config(config_name.replace(&format!(".{}", *CONFIG_EXTENSION), "")).selected(loaded).build() {                            
                                *config = config_item;
                                *loaded_config.lock().unwrap() = Some(config_name.to_string());
                                reset_window_positions(config_item.window_positions);
                            }
                        } else {
                            ui.selectable_config(format!("{} (failed)", config_name.replace(&format!(".{}", *CONFIG_EXTENSION), ""))).disabled(true).build();
                        }
                    }

                    if let Some(config_name) = &loaded_conf {
                        if !configs.contains_key(config_name) {
                            let default_config_name = &*DEFAULT_CONFIG;

                            if let Some(default_config) = configs.get(default_config_name) {
                                if let Some(default_config) = default_config {
                                    *config = *default_config;
                                    *loaded_config.lock().unwrap() = Some(default_config_name.to_string());
                                    reset_window_positions(default_config.window_positions);
                                }
                            };
                        }

                        if let Some(config_path) = PathBuf::from(&*config_dir).join(config_name).to_str() {
                            ui.separator();

                            if ui.button("Save##Config") {
                                match (*config).save_config(config_path, false) {
                                    Err(error) => create_messagebox(MessageBoxStyle::Error, "Error", &format!("Failed to save config: {} ({}).", config_name, error)),
                                    Ok(_) => {}
                                }
                            }

                            if config_name != &*DEFAULT_CONFIG {
                                ui.same_line();

                                if ui.button("Delete##Config") {
                                    match delete_config(config_path) {
                                        Err(error) => create_messagebox(MessageBoxStyle::Error, "Error", &format!("Failed to delete config: {} ({}).", config_name, error)),
                                        Ok(_) => {}
                                    }
                                }
                            }
                        }
                    }

                    ui.same_line();
                    
                    if ui.button("Reset##Config") {
                        let default_config = Config::default();

                        *config = default_config;
                        *loaded_config.lock().unwrap() = Some((*DEFAULT_CONFIG).to_string());
                        reset_window_positions(default_config.window_positions);
                    }
                });

                // Info
                TabItem::new("Info").build(&ui, || {
                    // Title
                    ui.text(ProgramConfig::Package::Name);
                    ui.text(ProgramConfig::Package::Description);
                    ui.separator();

                    // Info
                    ui.text(format!("Version: {}", ProgramConfig::Package::Version));
                    ui.text(format!("Author(s): {}", ProgramConfig::Package::Authors.replace(":", ", ")));
                    ui.separator();

                    // Links
                    if ui.button("Source") {
                        open_url(ProgramConfig::Links::Source);
                    }

                    ui.same_line();

                    if ui.button("License") {
                        open_url(ProgramConfig::Links::License);
                    }
                });
            });

            ui.separator();
            ui.text(format!("Toggle: {:?}", ProgramConfig::Keys::ToggleKey));

            ui.same_line();
            ui.text(format!("| Exit: {:?}", ProgramConfig::Keys::ExitKey));

            if let Some(loaded_config) = &loaded_config.lock().unwrap().clone() {
                ui.same_line();
                ui.text(format!("| Config: \"{}\"", loaded_config.replace(&format!(".{}", *CONFIG_EXTENSION), "")));

                if let Some(current_config) = configs.get(loaded_config) {
                    if let Some(current_config) = current_config {
                        if current_config != &*config {
                            ui.same_line();
                            ui.text("(modified)");
                        }
                    }
                }
            }
        });
}