use std::thread;
use std::time::Instant;
use mint::{Vector3, Vector2};
use windows::Win32::Foundation::HWND;

use crate::ui::main::{WINDOW_INFO, UI_FUNCTIONS};
use crate::ui::windows::{is_window_focused, hide_window_from_capture};
use crate::cheat::functions::{is_enemy_at_crosshair, is_enemy_in_fov};
use crate::utils::config::{CONFIG, ProgramConfig};
use crate::cheat::classes::game::GAME;
use crate::utils::mouse::release_mouse;
use crate::utils::process_manager::{rpm, rpm_offset, rpm_auto};

use crate::cheat::classes::entity::{Entity, Flags};
use crate::cheat::classes::game::update_entity_list_entry;
use crate::cheat::features::aimbot::{get_aimbot_toggled, aimbot_check, render_fov_circle, run_aimbot};
use crate::cheat::features::bomb_timer::render_bomb_timer;
use crate::cheat::features::cheat_list::render_cheat_list;
use crate::cheat::features::esp::{render_bones, render_head, render_eye_ray, get_2d_bone_rect, get_2d_box, render_snap_line, render_box, render_health_bar, render_armor_bar, render_weapon_name, render_distance, render_name};
use crate::cheat::features::radar::render_radar;
use crate::cheat::features::spectator_list::{is_spectating, render_spectator_list};
use crate::cheat::features::triggerbot::{get_triggerbot_toggled, run_triggerbot};
use crate::cheat::features::visuals::{render_crosshair, render_headshot_line};
use crate::cheat::features::watermark::render_watermark;
use crate::cheat::features::esp::render_bomb;
use crate::cheat::functions::{get_bomb_planted, get_bomb, get_bomb_site, get_bomb_position};
use crate::cheat::functions::is_enemy_visible;
use crate::cheat::features::aimbot::{AB_LOCKED_ENTITY, AB_OFF_ENTITY};
use crate::cheat::features::triggerbot::{TB_LOCKED_ENTITY, TB_OFF_ENTITY};

pub fn run_cheats_thread(hwnd: HWND, self_hwnd: HWND) {
    thread::spawn(move || {
        let mut window_hidden_from_capture = false;
        let window_info = WINDOW_INFO.clone();
        let ui_functions = UI_FUNCTIONS.clone();
        
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
                (*ui_functions.lock().unwrap()).remove(&format!("skeleton_{}", entity));
                (*ui_functions.lock().unwrap()).remove(&format!("head_{}", entity));
                (*ui_functions.lock().unwrap()).remove(&format!("eye_ray_{}", entity));
                (*ui_functions.lock().unwrap()).remove(&format!("snap_line_{}", entity));
                (*ui_functions.lock().unwrap()).remove(&format!("box_{}", entity));
                (*ui_functions.lock().unwrap()).remove(&format!("weapon_name_{}", entity));
                (*ui_functions.lock().unwrap()).remove(&format!("distance_{}", entity));
                (*ui_functions.lock().unwrap()).remove(&format!("player_name_{}", entity));
                (*ui_functions.lock().unwrap()).remove(&format!("health_bar_{}", entity));
                (*ui_functions.lock().unwrap()).remove(&format!("armor_bar_{}", entity));
            };

            let remove_all_esp = || {
                for i in 0 .. 64 {
                    remove_esp(i);
                }
            };

            let remove_ui_elements = || {
                (*ui_functions.lock().unwrap()).remove("fov_circle");
                (*ui_functions.lock().unwrap()).remove("radar");
                (*ui_functions.lock().unwrap()).remove("headshot_line");
                (*ui_functions.lock().unwrap()).remove("bomb_timer");
                (*ui_functions.lock().unwrap()).remove("spectator_list");
                (*ui_functions.lock().unwrap()).remove("bomb");
                
                remove_all_esp();
            };

            // Watermark
            if config.misc.enabled && config.misc.watermark_enabled {
                (*ui_functions.lock().unwrap()).insert("watermark".to_string(), Box::new(move |ui| {
                    render_watermark(ui, config);
                }));
            } else {
                (*ui_functions.lock().unwrap()).remove("watermark");
            }

            let no_weapon =  local_entity.pawn.weapon_name == "Knife" || local_entity.pawn.weapon_name == "Fists";
            let is_aimbot_toggled = !no_pawn && get_aimbot_toggled(config) && config.aimbot.enabled && is_game_window_focused && (!config.aimbot.only_weapon || config.aimbot.only_weapon && !no_weapon);
            let is_triggerbot_toggled = !no_pawn && (config.triggerbot.always_activated || get_triggerbot_toggled(config)) && config.triggerbot.enabled && is_game_window_focused && (!config.triggerbot.only_weapon || config.triggerbot.only_weapon && !no_weapon);

            // Cheat List
            if config.misc.enabled && config.misc.cheat_list_enabled {
                (*ui_functions.lock().unwrap()).insert("cheat_list".to_string(), Box::new(move |ui| {
                    render_cheat_list(ui, config, !no_pawn, is_aimbot_toggled, is_triggerbot_toggled);
                }));
            } else {
                (*ui_functions.lock().unwrap()).remove("cheat_list");
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
                if !(config.misc.enabled && config.misc.show_on_spectate) {
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
                    (*ui_functions.lock().unwrap()).insert("bomb".to_string(), Box::new(move |ui| {
                        render_bomb(ui, bomb_pos, local_entity.pawn.pos, bomb_screen_pos, &bomb_site, config);
                    }));
                } else {
                    (*ui_functions.lock().unwrap()).remove("bomb");
                }
            } else {
                (*ui_functions.lock().unwrap()).remove("bomb");
            }

            // Bomb Timer
            if !no_pawn && config.misc.enabled && config.misc.bomb_timer_enabled {
                (*ui_functions.lock().unwrap()).insert("bomb_timer".to_string(), Box::new(move |ui| {
                    render_bomb_timer(ui, bomb_planted, bomb_site.clone(), config);
                }));
            } else {
                (*ui_functions.lock().unwrap()).remove("bomb_timer");
            }

            // Aimbot Data
            let mut max_aim_distance: f32 = 100000.0;
            let mut aim_pos: Option<Vector3<f32>> = None;
            let mut aim_entity_address: Option<u64> = None;

            // Radar Data
            let mut radar_points: Vec<(Vector3<f32>, f32)> = Vec::new();

            // Spectator Data
            let mut spectators: Vec<String> = Vec::new();

            // Local Data
            let mut local_player_controller_index = 1;

            // Entities
            for i in 0 .. 64 {
                let mut entity = Entity::default();
                let mut entity_address: u64 = 0;

                if let Some(sum) = ((i + 1) as u64).checked_mul(0x78) {
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

                // Team Check
                if (config.misc.enabled && config.misc.exclude_team) && entity.controller.team_id == local_entity.controller.team_id {
                    remove_esp(i);
                    continue;
                }

                // Alive Check
                if !entity.is_alive() {
                    remove_esp(i);
                    continue;
                }

                // Radar Point
                if config.radar.enabled {
                    radar_points.push((entity.pawn.pos, entity.pawn.view_angle.y));
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

                // Enemy Visible
                let enemy_visible = is_enemy_visible( entity.pawn.spotted_by_mask, local_entity.pawn.spotted_by_mask, local_player_controller_index, i);

                // Aimbot Check
                if !no_pawn && config.aimbot.enabled {
                    aimbot_check(bone.bone_pos_list, window_info.1.0, window_info.1.1, &mut aim_pos, &mut max_aim_distance, &mut aim_entity_address, entity.pawn.address, enemy_visible, !entity.pawn.has_flag(Flags::InAir), config);
                }

                // Skeleton
                if config.esp.enabled && config.esp.skeleton_enabled {
                    (*ui_functions.lock().unwrap()).insert(format!("skeleton_{}", i), Box::new(move |ui| {
                        render_bones(ui, bone.bone_pos_list, config);
                    }));
                } else {
                    (*ui_functions.lock().unwrap()).remove(&format!("skeleton_{}", i));
                }

                // Head
                if config.esp.enabled && config.esp.head_enabled {
                    (*ui_functions.lock().unwrap()).insert(format!("head_{}", i), Box::new(move |ui| {
                        render_head(ui, bone.bone_pos_list, config);
                    }));
                } else {
                    (*ui_functions.lock().unwrap()).remove(&format!("head_{}", i));
                }

                // Eye Ray
                if config.esp.enabled && config.esp.eye_ray_enabled {
                    (*ui_functions.lock().unwrap()).insert(format!("eye_ray_{}", i), Box::new(move |ui| {
                        render_eye_ray(ui, bone.bone_pos_list, entity.pawn.view_angle, config, game.view, window_info);
                    }));
                } else {
                    (*ui_functions.lock().unwrap()).remove(&format!("eye_ray_{}", i));
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
                if config.esp.enabled && config.esp.snap_line_enabled {
                    (*ui_functions.lock().unwrap()).insert(format!("snap_line_{}", i), Box::new(move |ui| {
                        render_snap_line(ui, rect, config, window_info.1.0, window_info.1.1);
                    }));
                } else {
                    (*ui_functions.lock().unwrap()).remove(&format!("snap_line_{}", i));
                }

                // Box
                if config.esp.enabled && config.esp.box_enabled {
                    (*ui_functions.lock().unwrap()).insert(format!("box_{}", i), Box::new(move |ui| {
                        render_box(ui, rect, enemy_visible, config);
                    }));
                } else {
                    (*ui_functions.lock().unwrap()).remove(&format!("box_{}", i));
                }

                // Health Bar
                if config.esp.enabled && config.esp.health_bar_enabled {
                    (*ui_functions.lock().unwrap()).insert(format!("health_bar_{}", i), Box::new(move |ui| {
                        render_health_bar(ui, entity.pawn.health as f32, rect, config);
                    }));
                } else {
                    (*ui_functions.lock().unwrap()).remove(&format!("health_bar_{}", i));
                }

                // Armor Bar
                if config.esp.enabled && config.esp.armor_bar_enabled {
                    (*ui_functions.lock().unwrap()).insert(format!("armor_bar_{}", i), Box::new(move |ui| {
                        render_armor_bar(ui, entity.pawn.armor as f32, rect, config);
                    }));
                } else {
                    (*ui_functions.lock().unwrap()).remove(&format!("armor_bar_{}", i));
                }

                // Weapon Name
                if config.esp.enabled && config.esp.weapon_name_enabled {
                    (*ui_functions.lock().unwrap()).insert(format!("weapon_name_{}", i), Box::new(move |ui| {
                        render_weapon_name(ui, &entity.pawn.weapon_name, rect, config);
                    }));
                } else {
                    (*ui_functions.lock().unwrap()).remove(&format!("weapon_name_{}", i));
                }

                // Distance
                if !no_pawn && config.esp.enabled && config.esp.distance_enabled {
                    (*ui_functions.lock().unwrap()).insert(format!("distance_{}", i), Box::new(move |ui| {
                        render_distance(ui, entity.pawn.pos, local_entity.pawn.pos, rect, config);
                    }));
                } else {
                    (*ui_functions.lock().unwrap()).remove(&format!("distance_{}", i));
                }

                // Name
                if config.esp.enabled && config.esp.name_enabled {
                    (*ui_functions.lock().unwrap()).insert(format!("player_name_{}", i), Box::new(move |ui| {
                        render_name(ui, &entity.controller.player_name, rect, config);
                    }));
                } else {
                    (*ui_functions.lock().unwrap()).remove(&format!("player_name_{}", i));
                }
            }

            // Spectator List
            if !no_pawn && config.misc.enabled && config.misc.spectator_list_enabled {
                (*ui_functions.lock().unwrap()).insert("spectator_list".to_string(), Box::new(move |ui| {
                    render_spectator_list(ui, spectators.clone(), config);
                }));
            } else {
                (*ui_functions.lock().unwrap()).remove("spectator_list");
            }
            
            // Aim Info
            let (aiming_at_enemy, allow_shoot, aiming_at_address) = {
                if no_pawn {
                    (false, false, 0)
                } else {
                    is_enemy_at_crosshair(local_entity.pawn.address, local_entity.controller.team_id, game.address.entity_list, config)
                }
            };

            let aimbot_info = {
                if let Some(aim_pos) = aim_pos {
                    match is_enemy_in_fov(config, aim_pos, local_entity.pawn.camera_pos, local_entity.pawn.view_angle) {
                        Some(v) => Some(v),
                        None => None
                    }
                } else {
                    None
                }
            };

            // Crosshair
            if !no_pawn && config.crosshair.enabled && (!config.crosshair.only_weapon || config.crosshair.only_weapon && !no_weapon) {
                (*ui_functions.lock().unwrap()).insert("cross_hair".to_string(), Box::new(move |ui| {
                    render_crosshair(ui, Vector2 { x: window_info.1.0 as f32 / 2.0, y: window_info.1.1 as f32 / 2.0 }, aiming_at_enemy && allow_shoot, config);
                }));
            } else {
                (*ui_functions.lock().unwrap()).remove("cross_hair");
            }

            // Headshot Line
            if !no_pawn && config.misc.enabled && config.misc.headshot_line_enabled {
                (*ui_functions.lock().unwrap()).insert("headshot_line".to_string(), Box::new(move |ui| {
                    render_headshot_line(ui, window_info.1.0, window_info.1.1, local_entity.pawn.fov, local_entity.pawn.view_angle.x, config);
                }));
            } else {
                (*ui_functions.lock().unwrap()).remove("headshot_line");
            }

            // FOV Circle
            if !no_pawn && config.aimbot.enabled && config.aimbot.fov_circle_enabled && (!config.aimbot.only_weapon || config.aimbot.only_weapon && !no_weapon) {
                (*ui_functions.lock().unwrap()).insert("fov_circle".to_string(), Box::new(move |ui| {
                    render_fov_circle(ui, window_info.1.0, window_info.1.1, local_entity.pawn.fov, aimbot_info, config);
                }));
            } else {
                (*ui_functions.lock().unwrap()).remove("fov_circle");
            }

            // Radar
            if !no_pawn && config.radar.enabled {
                (*ui_functions.lock().unwrap()).insert("radar".to_string(), Box::new(move |ui| {
                    render_radar(ui, config, local_entity.pawn.pos, local_entity.pawn.view_angle.y, radar_points.clone());
                }));
            } else {
                (*ui_functions.lock().unwrap()).remove("radar");
            }

            // Aimbot
            if is_aimbot_toggled {
                if let Some(aimbot_info) = aimbot_info {
                    if let Some(aim_pos) = aim_pos {
                        if let Some(entity_index) = aim_entity_address {
                            run_aimbot(config, aimbot_info, window_info, game.view, aim_pos, entity_index);
                        }
                    }
                }
            }

            // Aimbot Lock
            if !is_aimbot_toggled || aim_entity_address.is_none() || aim_pos.is_none() || aimbot_info.is_none() {
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
            if is_triggerbot_toggled && aiming_at_enemy && allow_shoot && aiming_at_address != 0 {
                run_triggerbot(aiming_at_address, config);
            } else {
                release_mouse();
            }

            // Triggerbot Lock
            if !is_triggerbot_toggled || aiming_at_address == 0 || !aiming_at_enemy || !allow_shoot {
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