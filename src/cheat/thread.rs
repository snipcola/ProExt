// Copyright (c) 2024 Snipcola
// SPDX-License-Identifier: MIT

use std::thread;
use std::time::Instant;

use mint::{Vector3, Vector2};
use windows::Win32::Foundation::HWND;

use crate::config::ProgramConfig;
use crate::utils::mouse::release_mouse;

use crate::ui::main::{WINDOW_INFO, RENDER_LIST};
use crate::utils::ui::windows::{is_window_focused, hide_window_from_capture};

use crate::utils::cheat::config::CONFIG;
use crate::utils::cheat::process::{rpm, rpm_offset, rpm_auto};

use crate::cheat::classes::game::{GAME, update_entity_list_entry};
use crate::cheat::classes::entity::{Entity, Flags};

use crate::cheat::features::esp::{render_bones, render_head, render_eye_ray, get_2d_bone_rect, get_2d_box, render_snap_line, render_box, render_health_bar, render_armor_bar, render_ammo_bar, render_weapon_name, render_distance, render_name, render_bomb, get_esp_toggled, render_headshot_line};
use crate::cheat::features::rcs::{get_rcs_toggled, run_rcs, get_rcs_config, get_rcs_mouse};
use crate::cheat::features::aimbot::{AB_LOCKED_ENTITY, AB_OFF_ENTITY, get_aimbot_yaw_pitch, get_aimbot_config, get_aimbot_toggled, aimbot_check, render_fov_circle, run_aimbot};
use crate::cheat::features::triggerbot::{TB_LOCKED_ENTITY, TB_OFF_ENTITY, get_triggerbot_config, get_triggerbot_toggled, run_triggerbot};
use crate::cheat::features::crosshair::{render_crosshair, get_crosshair_toggled, get_crosshair_config};
use crate::cheat::features::radar::{render_radar, get_radar_toggled};

use crate::cheat::features::watermark::render_watermark;
use crate::cheat::features::cheat_list::render_cheat_list;
use crate::cheat::features::bomb_timer::render_bomb_timer;
use crate::cheat::features::spectator_list::{is_spectating, render_spectator_list};

use crate::cheat::functions::{get_bomb_planted, get_bomb, get_bomb_site, get_bomb_position, is_enemy_visible, has_weapon, is_enemy_at_crosshair, calculate_distance};

pub fn run_cheats_thread(hwnd: HWND, self_hwnd: HWND) {
    thread::spawn(move || {
        let mut window_hidden_from_capture = false;
        let window_info = WINDOW_INFO.clone();
        let render_list = RENDER_LIST.clone();
        
        let mut no_pawn = false;
        let mut local_entity = Entity::default();

        loop {
            let game = GAME.lock().unwrap().clone();
            let config = CONFIG.lock().unwrap().clone();
            let window_info = match window_info.lock().unwrap().clone() {
                Some(window_info) => window_info,
                None => { continue; }
            };

            let is_game_window_focused = is_window_focused(hwnd);

            if !window_hidden_from_capture && (config.settings.enabled && config.settings.bypass_capture) {
                hide_window_from_capture(self_hwnd, true);
                window_hidden_from_capture = true;
            } else if window_hidden_from_capture && !(config.settings.enabled && config.settings.bypass_capture) {
                hide_window_from_capture(self_hwnd, false);
                window_hidden_from_capture = false;
            }

            let matrix_address = game.address.matrix;
            let controller_address = game.address.local_controller;
            let pawn_address = game.address.local_pawn;
            
            let remove_esp = | entity: u64 | {
                (*render_list.lock().unwrap()).remove(&format!("skeleton_{}", entity));
                (*render_list.lock().unwrap()).remove(&format!("head_{}", entity));
                (*render_list.lock().unwrap()).remove(&format!("eye_ray_{}", entity));
                (*render_list.lock().unwrap()).remove(&format!("snap_line_{}", entity));
                (*render_list.lock().unwrap()).remove(&format!("box_{}", entity));
                (*render_list.lock().unwrap()).remove(&format!("weapon_name_{}", entity));
                (*render_list.lock().unwrap()).remove(&format!("distance_{}", entity));
                (*render_list.lock().unwrap()).remove(&format!("player_name_{}", entity));
                (*render_list.lock().unwrap()).remove(&format!("health_bar_{}", entity));
                (*render_list.lock().unwrap()).remove(&format!("armor_bar_{}", entity));
                (*render_list.lock().unwrap()).remove(&format!("ammo_bar_{}", entity));
            };

            let remove_all_esp = || {
                for i in 0 .. 64 {
                    remove_esp(i);
                }
            };

            let remove_ui_elements = || {
                (*render_list.lock().unwrap()).remove("cross_hair");
                (*render_list.lock().unwrap()).remove("fov_circle");
                (*render_list.lock().unwrap()).remove("radar");
                (*render_list.lock().unwrap()).remove("headshot_line");
                (*render_list.lock().unwrap()).remove("bomb_timer");
                (*render_list.lock().unwrap()).remove("spectator_list");
                (*render_list.lock().unwrap()).remove("bomb");
                
                remove_all_esp();
            };

            // Watermark
            if config.misc.enabled && config.misc.watermark_enabled {
                (*render_list.lock().unwrap()).insert("watermark".to_string(), Box::new(move |ui| {
                    render_watermark(ui, config);
                }));
            } else {
                (*render_list.lock().unwrap()).remove("watermark");
            }

            let no_weapon = !has_weapon(local_entity.pawn.weapon_type);

            let rcs_config = if config.rcs.shared { config.rcs.configs.shared } else { get_rcs_config(config.rcs.configs, local_entity.pawn.weapon_type) };
            let aimbot_config = if config.aimbot.shared { config.aimbot.configs.shared } else { get_aimbot_config(config.aimbot.configs, local_entity.pawn.weapon_type) };
            let triggerbot_config = if config.triggerbot.shared { config.triggerbot.configs.shared } else { get_triggerbot_config(config.triggerbot.configs, local_entity.pawn.weapon_type) };
            let crosshair_config = if config.crosshair.shared { config.crosshair.configs.shared } else { get_crosshair_config(config.crosshair.configs, local_entity.pawn.weapon_type) };

            let is_aimbot_toggled = !no_pawn && config.aimbot.enabled && is_game_window_focused && (!config.aimbot.only_weapon || config.aimbot.only_weapon && !no_weapon) && (config.aimbot.always || get_aimbot_toggled(config));
            let is_triggerbot_toggled = !no_pawn && config.triggerbot.enabled && is_game_window_focused && (!config.triggerbot.only_weapon || config.triggerbot.only_weapon && !no_weapon) && (config.triggerbot.always || get_triggerbot_toggled(config));
            let is_rcs_toggled = !no_pawn && config.rcs.enabled && is_game_window_focused && (config.rcs.always || get_rcs_toggled(config));
            let is_esp_toggled = config.esp.enabled && (config.esp.always || get_esp_toggled(config));
            let is_crosshair_toggled = config.crosshair.enabled && (!config.crosshair.only_weapon || no_pawn || config.crosshair.only_weapon && !no_weapon) && (config.crosshair.always || get_crosshair_toggled(config));
            let is_radar_toggled = config.radar.enabled && (config.radar.always || get_radar_toggled(config));

            // Cheat List
            if config.misc.enabled && config.misc.cheat_list_enabled {
                (*render_list.lock().unwrap()).insert("cheat_list".to_string(), Box::new(move |ui| {
                    render_cheat_list(ui, config, !no_pawn, is_aimbot_toggled, is_triggerbot_toggled, is_rcs_toggled, is_esp_toggled, is_crosshair_toggled, is_radar_toggled);
                }));
            } else {
                (*render_list.lock().unwrap()).remove("cheat_list");
            }

            if !rpm(matrix_address, &mut (*GAME.lock().unwrap()).view.matrix, 64) {
                remove_ui_elements();
                continue;
            }

            update_entity_list_entry();

            let mut local_controller_address = 0;
            let mut local_pawn_address = 0;

            if !rpm_auto(controller_address, &mut local_controller_address) {
                remove_ui_elements();
                continue;
            }

            if !rpm_auto(pawn_address, &mut local_pawn_address) {
                remove_ui_elements();
                continue;
            }

            // Update Controller & Pawn
            if !local_entity.update_controller(local_controller_address) {
                remove_ui_elements();
                continue;
            }

            if !local_entity.update_pawn(local_pawn_address, window_info, game.view) {
                if !(config.settings.enabled && config.settings.show_on_spectate) {
                    remove_ui_elements();
                    continue;
                };

                no_pawn = true;
            } else {
                no_pawn = false;
            }

            // Bomb Data
            let (bomb_planted, bomb_site, bomb_pos): (bool, Option<String>, Option<Vector3<f32>>) = if !no_pawn && (config.esp.bomb_enabled || (config.misc.enabled && config.misc.bomb_timer_enabled)) {                
                let bomb_address = game.address.bomb;
                let bomb_planted = get_bomb_planted(bomb_address);

                if bomb_planted {
                    let planted_bomb = get_bomb(bomb_address);

                    let bomb_site = match planted_bomb {
                        Some(bomb) => get_bomb_site(bomb),
                        None => None
                    };
                    
                    let bomb_pos = match planted_bomb {
                        Some(bomb) => get_bomb_position(bomb),
                        None => None
                    };

                    (bomb_planted, bomb_site, bomb_pos)
                } else {
                    (bomb_planted, None, None)
                }
            } else {
                (false, None, None)
            };

            // Bomb
            if !no_pawn && config.esp.bomb_enabled && bomb_site.is_some() && bomb_pos.is_some() {
                let (bomb_site, bomb_pos) = (bomb_site.clone().unwrap(), bomb_pos.unwrap());
                let mut bomb_screen_pos = Vector2 { x: 0.0, y: 0.0 };
                
                if game.view.world_to_screen(bomb_pos, &mut bomb_screen_pos, window_info) {
                    (*render_list.lock().unwrap()).insert("bomb".to_string(), Box::new(move |ui| {
                        render_bomb(ui, bomb_pos, local_entity.pawn.pos, bomb_screen_pos, &bomb_site, config);
                    }));
                } else {
                    (*render_list.lock().unwrap()).remove("bomb");
                }
            } else {
                (*render_list.lock().unwrap()).remove("bomb");
            }

            // Bomb Timer
            if config.misc.enabled && config.misc.bomb_timer_enabled {
                (*render_list.lock().unwrap()).insert("bomb_timer".to_string(), Box::new(move |ui| {
                    render_bomb_timer(ui, bomb_planted, bomb_site.clone(), config, no_pawn);
                }));
            } else {
                (*render_list.lock().unwrap()).remove("bomb_timer");
            }

            // Aimbot Data
            let mut max_aim_distance: f32 = f32::MAX;
            let mut aim_pos: Option<Vector3<f32>> = None;
            let mut aim_entity_address: Option<u64> = None;

            // Radar Data
            let mut radar_points: Vec<(Vector3<f32>, f32, bool, bool)> = Vec::new();

            // Spectator Data
            let mut spectators: Vec<String> = Vec::new();

            // Local Data
            let mut local_player_controller_index = 1;

            // Entities
            for i in 0 .. 64 {
                let mut entity = Entity::default();
                let mut entity_address: u64 = 0;

                if let Some(sum) = (i + 1).checked_mul(0x78) {
                    if !rpm_offset(game.address.entity_list_entry, sum, &mut entity_address) {
                        remove_esp(i);
                        continue;
                    }
                } else {
                    remove_esp(i);
                    continue;
                }

                // Self Check
                if entity_address == local_entity.controller.address {
                    local_player_controller_index = i;
                    remove_esp(i);
                    continue;
                }

                // Update Controller & Pawn
                if !entity.update_controller(entity_address) {
                    remove_esp(i);
                    continue;
                }

                if !entity.update_pawn(entity.pawn.address, window_info, game.view) {
                    remove_esp(i);
                    continue;
                }

                // Spectator Check
                if !no_pawn && (config.misc.enabled && config.misc.spectator_list_enabled) && is_spectating(entity.controller.address, game.address.entity_list_entry, local_entity.pawn.address) {
                    spectators.push(entity.controller.player_name.clone());
                }

                // Is Friendly
                let exclude_team = config.settings.enabled && config.settings.exclude_team;
                let is_friendly = entity.controller.team_id == local_entity.controller.team_id;

                // Team Check
                if exclude_team && is_friendly {
                    remove_esp(i);
                    continue;
                }

                // Alive Check
                if !entity.is_alive() {
                    remove_esp(i);
                    continue;
                }

                // Enemy Visible
                let is_visible = if is_friendly { true } else { is_enemy_visible(entity.pawn.spotted_by_mask, local_entity.pawn.spotted_by_mask, local_player_controller_index, i) };

                // Distance
                let distance = if no_pawn { 0 } else { calculate_distance(entity.pawn.pos, local_entity.pawn.pos) };

                // Radar Point
                if is_radar_toggled {
                    radar_points.push((entity.pawn.pos, entity.pawn.view_angle.y, is_visible, is_friendly));
                }

                // Screen Check
                if !entity.is_in_screen(window_info, game.view) {
                    remove_esp(i);
                    continue;
                }

                // Bone Data
                let bone = match entity.get_bone() {
                    Some(bone) => bone,
                    None => {
                        remove_esp(i);
                        continue;
                    }
                };

                // Aimbot Check
                if !no_pawn && config.aimbot.enabled {
                    aimbot_check(bone.bone_pos_list, window_info.1.0, window_info.1.1, &mut aim_pos, &mut max_aim_distance, &mut aim_entity_address, entity.pawn.address, is_visible, !entity.pawn.has_flag(Flags::InAir), distance, aimbot_config);
                }

                // Skeleton
                if is_esp_toggled && config.esp.skeleton_enabled {
                    (*render_list.lock().unwrap()).insert(format!("skeleton_{}", i), Box::new(move |ui| {
                        render_bones(ui, bone.bone_pos_list, config);
                    }));
                } else {
                    (*render_list.lock().unwrap()).remove(&format!("skeleton_{}", i));
                }

                // Head
                if is_esp_toggled && config.esp.head_enabled {
                    (*render_list.lock().unwrap()).insert(format!("head_{}", i), Box::new(move |ui| {
                        render_head(ui, bone.bone_pos_list, config);
                    }));
                } else {
                    (*render_list.lock().unwrap()).remove(&format!("head_{}", i));
                }

                // Eye Ray
                if is_esp_toggled && config.esp.eye_ray_enabled {
                    (*render_list.lock().unwrap()).insert(format!("eye_ray_{}", i), Box::new(move |ui| {
                        render_eye_ray(ui, bone.bone_pos_list, entity.pawn.view_angle, config, game.view, window_info);
                    }));
                } else {
                    (*render_list.lock().unwrap()).remove(&format!("eye_ray_{}", i));
                }

                // Rect Data
                let rect = {
                    if config.esp.box_mode == 0 {
                        get_2d_box(bone.bone_pos_list, entity.pawn.screen_pos)
                    } else {
                        get_2d_bone_rect(bone.bone_pos_list)
                    }
                };

                // Rect Check
                let (max_width, max_height) = ((window_info.1.0 as f32 * 1.5), (window_info.1.1 as f32 * 1.5));

                if rect.x.abs() >= max_width || rect.y.abs() >= max_height || rect.z.abs() >= max_width || rect.w.abs() >= max_height {
                    remove_esp(i);
                    continue;
                }

                // Snapline
                if is_esp_toggled && config.esp.snap_line_enabled {
                    (*render_list.lock().unwrap()).insert(format!("snap_line_{}", i), Box::new(move |ui| {
                        render_snap_line(ui, rect, config, window_info.1.0, window_info.1.1);
                    }));
                } else {
                    (*render_list.lock().unwrap()).remove(&format!("snap_line_{}", i));
                }

                // Box
                if is_esp_toggled && config.esp.box_enabled {
                    (*render_list.lock().unwrap()).insert(format!("box_{}", i), Box::new(move |ui| {
                        render_box(ui, rect, is_visible, is_friendly, config);
                    }));
                } else {
                    (*render_list.lock().unwrap()).remove(&format!("box_{}", i));
                }

                // Health Bar
                if is_esp_toggled && config.esp.health_bar_enabled {
                    (*render_list.lock().unwrap()).insert(format!("health_bar_{}", i), Box::new(move |ui| {
                        render_health_bar(ui, entity.pawn.health as f32, rect, config);
                    }));
                } else {
                    (*render_list.lock().unwrap()).remove(&format!("health_bar_{}", i));
                }

                // Armor Bar
                if is_esp_toggled && config.esp.armor_bar_enabled {
                    (*render_list.lock().unwrap()).insert(format!("armor_bar_{}", i), Box::new(move |ui| {
                        render_armor_bar(ui, entity.pawn.armor as f32, rect, config);
                    }));
                } else {
                    (*render_list.lock().unwrap()).remove(&format!("armor_bar_{}", i));
                }

                // Ammo Bar
                if is_esp_toggled && config.esp.ammo_bar_enabled {
                    (*render_list.lock().unwrap()).insert(format!("ammo_bar_{}", i), Box::new(move |ui| {
                        render_ammo_bar(ui, entity.pawn.weapon_ammo as f32, entity.pawn.weapon_max_ammo as f32, rect, config);
                    }));
                } else {
                    (*render_list.lock().unwrap()).remove(&format!("ammo_bar_{}", i));
                }

                // Weapon Name
                if is_esp_toggled && config.esp.weapon_name_enabled {
                    (*render_list.lock().unwrap()).insert(format!("weapon_name_{}", i), Box::new(move |ui| {
                        render_weapon_name(ui, &entity.pawn.weapon_name, rect, config);
                    }));
                } else {
                    (*render_list.lock().unwrap()).remove(&format!("weapon_name_{}", i));
                }

                // Distance
                if !no_pawn && is_esp_toggled && config.esp.distance_enabled {
                    (*render_list.lock().unwrap()).insert(format!("distance_{}", i), Box::new(move |ui| {
                        render_distance(ui, distance, rect, config);
                    }));
                } else {
                    (*render_list.lock().unwrap()).remove(&format!("distance_{}", i));
                }

                // Name
                if is_esp_toggled && config.esp.name_enabled {
                    (*render_list.lock().unwrap()).insert(format!("player_name_{}", i), Box::new(move |ui| {
                        render_name(ui, &entity.controller.player_name, rect, config);
                    }));
                } else {
                    (*render_list.lock().unwrap()).remove(&format!("player_name_{}", i));
                }
            }

            // Spectator List
            if config.misc.enabled && config.misc.spectator_list_enabled {
                (*render_list.lock().unwrap()).insert("spectator_list".to_string(), Box::new(move |ui| {
                    render_spectator_list(ui, spectators.clone(), config, no_pawn);
                }));
            } else {
                (*render_list.lock().unwrap()).remove("spectator_list");
            }
            
            // Aim Info
            let (aiming_at_enemy, allow_shoot, aiming_at_address, aiming_at_pos) = {
                if no_pawn {
                    (false, false, 0, None)
                } else {
                    is_enemy_at_crosshair(local_entity.pawn.address, local_entity.controller.team_id, game.address.entity_list, config.settings.enabled && config.settings.exclude_team)
                }
            };

            let aimbot_info = {
                if let Some(aim_pos) = aim_pos {
                    match get_aimbot_yaw_pitch(aimbot_config, aim_pos, local_entity.pawn.camera_pos, local_entity.pawn.view_angle) {
                        Some(v) => Some(v),
                        None => None
                    }
                } else {
                    None
                }
            };

            // RCS Info
            let rcs_info = get_rcs_mouse(config, rcs_config, local_entity.pawn.shots_fired, local_entity.pawn.aim_punch_cache);

            // Crosshair
            if is_crosshair_toggled {
                (*render_list.lock().unwrap()).insert("cross_hair".to_string(), Box::new(move |ui| {
                    render_crosshair(ui, Vector2 { x: window_info.1.0 as f32 / 2.0, y: window_info.1.1 as f32 / 2.0 }, aiming_at_enemy && allow_shoot, crosshair_config);
                }));
            } else {
                (*render_list.lock().unwrap()).remove("cross_hair");
            }

            // Headshot Line
            if is_esp_toggled && config.esp.headshot_line_enabled {
                (*render_list.lock().unwrap()).insert("headshot_line".to_string(), Box::new(move |ui| {
                    render_headshot_line(ui, window_info.1.0, window_info.1.1, local_entity.pawn.fov, local_entity.pawn.view_angle.x, config);
                }));
            } else {
                (*render_list.lock().unwrap()).remove("headshot_line");
            }

            // FOV Circle
            if !no_pawn && config.aimbot.enabled && aimbot_config.fov_circle_enabled && (!config.aimbot.only_weapon || config.aimbot.only_weapon && !no_weapon) && (!aimbot_config.fov_circle_only_toggled || aimbot_config.fov_circle_only_toggled && is_aimbot_toggled) {
                (*render_list.lock().unwrap()).insert("fov_circle".to_string(), Box::new(move |ui| {
                    render_fov_circle(ui, window_info.1.0, window_info.1.1, local_entity.pawn.fov, aimbot_info, aimbot_config);
                }));
            } else {
                (*render_list.lock().unwrap()).remove("fov_circle");
            }

            // Radar
            if is_radar_toggled {
                (*render_list.lock().unwrap()).insert("radar".to_string(), Box::new(move |ui| {
                    let radar_points = if no_pawn { Vec::new() } else { radar_points.clone() };
                    render_radar(ui, config, local_entity.pawn.pos, local_entity.pawn.view_angle.y, radar_points.clone());
                }));
            } else {
                (*render_list.lock().unwrap()).remove("radar");
            }

            // Toggled
            let aimbot_toggled = is_aimbot_toggled && aim_entity_address.is_some() && aim_pos.is_some() && aimbot_info.is_some();
            let triggerbot_toggled = is_triggerbot_toggled && aiming_at_enemy && allow_shoot && aiming_at_address != 0 && aiming_at_pos.is_some();
            let rcs_toggled = is_rcs_toggled && rcs_info.is_some();

            // RCS
            if rcs_toggled && !aimbot_toggled {
                run_rcs(rcs_info.unwrap());
            }

            // Aimbot
            if aimbot_toggled {
                if let Some(aimbot_info) = aimbot_info {
                    if let Some(aim_pos) = aim_pos {
                        if let Some(entity_index) = aim_entity_address {
                            run_aimbot(aimbot_config, aimbot_info, window_info, game.view, aim_pos, entity_index, is_rcs_toggled, rcs_info);
                        }
                    }
                }
            }

            // Aimbot Lock
            if !aimbot_toggled {
                let mut locked_entity = AB_LOCKED_ENTITY.lock().unwrap();
                let mut off_entity = AB_OFF_ENTITY.lock().unwrap();
                
                if off_entity.is_none() {
                    *off_entity = Some(Instant::now());
                }

                if off_entity.unwrap().elapsed() > ProgramConfig::CheatDelays::AimbotOffEntity {
                    *locked_entity = None;
                }
            } else {
                *AB_OFF_ENTITY.lock().unwrap() = None;
            }

            // Triggerbot
            if triggerbot_toggled {
                run_triggerbot(aiming_at_address, triggerbot_config, aiming_at_pos.unwrap(), local_entity.pawn.pos);
            } else {
                release_mouse();
            }

            // Triggerbot Lock
            if !triggerbot_toggled {
                let mut locked_entity = TB_LOCKED_ENTITY.lock().unwrap();
                let mut off_entity = TB_OFF_ENTITY.lock().unwrap();

                if off_entity.is_none() {
                    *off_entity = Some(Instant::now());
                }

                if off_entity.unwrap().elapsed() > ProgramConfig::CheatDelays::TriggerbotOffEntity {
                    *locked_entity = None;
                    release_mouse();
                }
            } else {
                *TB_OFF_ENTITY.lock().unwrap() = None;
            }
        }
    });
}