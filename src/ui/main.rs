use std::{time::Instant, thread::{self, sleep}, sync::{Arc, Mutex}, f32::consts::PI};
use colored::Colorize;
use imgui::{Ui, ColorEditFlags};
use lazy_static::lazy_static;

use glium::{glutin::{event_loop::ControlFlow, event::{Event, WindowEvent, DeviceEvent, ElementState}, dpi::{PhysicalSize, PhysicalPosition}}, Surface};
use imgui_glium_renderer::Renderer;
use mint::{Vector4, Vector2, Vector3};

use crate::{ui::{menu::render_menu, radar::render_radar}, utils::{config::{DEBUG, PACKAGE_NAME, PACKAGE_VERSION, PACKAGE_AUTHORS, PROCESS_TITLE, PROCESS_CLASS, TOGGLE_KEY, THREAD_DELAYS, CONFIG}, process_manager::{read_memory, read_memory_auto}}, cheat::{game::{GAME, update_entity_list_entry, set_view_angle}, entity::Entity, bone::{BoneIndex, aim_position_to_bone_index}}};
use crate::ui::windows::{create_window, find_window, focus_window, init_imgui, get_window_info, is_window_focused};

lazy_static! {
    pub static ref WINDOW_INFO: Arc<Mutex<Option<((i32, i32), (i32, i32))>>> = Arc::new(Mutex::new(None));
    pub static ref WINDOW_LAST_MOVED: Arc<Mutex<Instant>> = Arc::new(Mutex::new(Instant::now()));
    pub static ref WINDOW_FOCUSED: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));

    pub static ref AIMBOT_TOGGLED: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    pub static ref TOGGLED: Arc<Mutex<bool>> = Arc::new(Mutex::new(true));
    pub static ref EXIT: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
}

pub fn color_edit_u32_tuple(ui: &Ui, label: &str, color_tuple: &mut (u32, u32, u32, u32)) {
    let mut color_vector = Vector4 {
        x: color_tuple.0 as f32 / 255.0,
        y: color_tuple.1 as f32 / 255.0,
        z: color_tuple.2 as f32 / 255.0,
        w: color_tuple.3 as f32 / 255.0,
    };

    ui.color_edit4_config(label, &mut color_vector)
        .flags(ColorEditFlags::NO_INPUTS)
        .build();

    color_tuple.0 = (color_vector.x * 255.0) as u32;
    color_tuple.1 = (color_vector.y * 255.0) as u32;
    color_tuple.2 = (color_vector.z * 255.0) as u32;
    color_tuple.3 = (color_vector.w * 255.0) as u32;
}

pub fn color_u32_to_f32(color: (u32, u32, u32, u32)) -> (f32, f32, f32, f32) {
    return (color.0 as f32 / 255.0, color.1 as f32 / 255.0, color.2 as f32 / 255.0, color.3 as f32 / 255.0);
}

pub fn distance_to_vec(pos1: Vector2<f32>, pos2: Vector2<f32>) -> f32 {
    let x_diff = pos2.x - pos1.x;
    let y_diff = pos2.y - pos1.y;
    let distance = (x_diff.powi(2) + y_diff.powi(2)).sqrt();

    return distance;
}

pub fn hotkey_index_to_io(hotkey_index: usize) -> Result<rdev::Button, rdev::Key> {
    if hotkey_index == 1 {
        return Ok(rdev::Button::Left);
    }
    else if hotkey_index == 2 {
        return Ok(rdev::Button::Middle);
    }
    else if hotkey_index == 3 {
        return Ok(rdev::Button::Right);
    }
    else if hotkey_index == 4 {
        return Err(rdev::Key::ShiftLeft);
    }
    else if hotkey_index == 5 {
        return Err(rdev::Key::ControlLeft);
    }
    else {
        return Err(rdev::Key::Alt);
    }
}

pub fn init_gui() {
    let title = &format!("{} v{} - {}", (*PACKAGE_NAME).to_uppercase(), (*PACKAGE_VERSION), (*PACKAGE_AUTHORS));
    let window_title = &*PROCESS_TITLE;
    let window_class = &*PROCESS_CLASS;
    let toggle_key = &*TOGGLE_KEY;

    let window_hwnd = match find_window(window_title, Some(window_class)) {
        Some(hwnd) => hwnd,
        None => {
            println!("{} Failed to find {} window", "[ FAIL ]".bold().red(), window_title.bold());
            return;
        }
    };

    if *DEBUG { println!("{} {} Window HWND: {}", "[ INFO ]".blue().bold(), window_title.bold(), format!("{:?}", window_hwnd).bold()); }

    let (event_loop, display) = create_window(title, window_hwnd);
    let (mut winit_platform, mut imgui_context) = init_imgui(&display);

    let self_hwnd = match find_window(title, None) {
        Some(hwnd) => hwnd,
        None => {
            println!("{} Failed to find {} window", "[ FAIL ]".bold().red(), title.bold());
            return;
        }
    };

    if *DEBUG { println!("{} Self Window HWND: {}", "[ INFO ]".blue().bold(), format!("{:?}", self_hwnd).bold()); }

    focus_window(self_hwnd);

    let mut renderer = Renderer::init(&mut imgui_context, &display).unwrap();
    let mut last_frame = Instant::now();

    println!("{} Rendering GUI (toggle: {})", "[ OKAY ]".bold().green(), format!("{:?}", toggle_key).bold());

    let toggled = TOGGLED.clone();
    let exit = EXIT.clone();
    let window_info = WINDOW_INFO.clone();
    let window_last_moved = WINDOW_LAST_MOVED.clone();
    let window_focused = WINDOW_FOCUSED.clone();
    let aimbot_toggled = AIMBOT_TOGGLED.clone();
    
    let key_event_thread = thread::spawn(move || {
        let _ = rdev::listen(move | event | {
            match event.event_type {
                rdev::EventType::KeyRelease(key) => {
                    let window_focused = *window_focused.lock().unwrap();

                    if format!("{:?}", key) == format!("{:?}", toggle_key) && window_focused {
                        let toggled_value = *toggled.lock().unwrap();
                        *toggled.lock().unwrap() = !toggled_value;
                        
                        if toggled_value {
                            focus_window(window_hwnd);
                        } else {
                            focus_window(self_hwnd);
                        }
                    } else {
                        match hotkey_index_to_io((*CONFIG.lock().unwrap()).aim_bot_hot_key) {
                            Ok(_) => {},
                            Err(aimbot_key) => {
                                if key == aimbot_key && is_window_focused(window_hwnd) {
                                    (*aimbot_toggled.lock().unwrap()) = false;
                                }
                            }
                        }
                    }
                },
                rdev::EventType::KeyPress(key) => {
                    match hotkey_index_to_io((*CONFIG.lock().unwrap()).aim_bot_hot_key) {
                        Ok(_) => {},
                        Err(aimbot_key) => {
                            if key == aimbot_key && is_window_focused(window_hwnd) {
                                (*aimbot_toggled.lock().unwrap()) = true;
                            }
                        }
                    }
                },
                rdev::EventType::ButtonPress(button) => {
                    match hotkey_index_to_io((*CONFIG.lock().unwrap()).aim_bot_hot_key) {
                        Err(_) => {},
                        Ok(aimbot_button) => {
                            if button == aimbot_button && is_window_focused(window_hwnd) {
                                (*aimbot_toggled.lock().unwrap()) = true;
                            }
                        }
                    }
                },
                rdev::EventType::ButtonRelease(button) => {
                    match hotkey_index_to_io((*CONFIG.lock().unwrap()).aim_bot_hot_key) {
                        Err(_) => {},
                        Ok(aimbot_button) => {
                            if button == aimbot_button && is_window_focused(window_hwnd) {
                                (*aimbot_toggled.lock().unwrap()) = false;
                            }
                        }
                    }
                },
                _ => {}
            }
        });
    });

    if *DEBUG { println!("{} KeyEvents Thread ID: {}", "[ INFO ]".blue().bold(), format!("{:?}", key_event_thread.thread().id()).bold()); }

    let window_focused = WINDOW_FOCUSED.clone();
    let window_tasks_thread = thread::spawn(move || {
        let mut stored_window_info: ((i32, i32), (i32, i32)) = ((0, 0), (0, 0));

        loop {
            if let Some(((x, y), (width, height))) = get_window_info(window_hwnd) {
                let window_info_var = ((x + 1, y + 1), (width - 2, height - 2));

                if window_info_var != stored_window_info {
                    stored_window_info = window_info_var;
                    *window_info.lock().unwrap() = Some(window_info_var);
                    *window_last_moved.lock().unwrap() = Instant::now();
                }
            } else {
                *exit.lock().unwrap() = true;
            }

            *window_focused.lock().unwrap() = is_window_focused(window_hwnd) || is_window_focused(self_hwnd);
            sleep(THREAD_DELAYS.window_tasks);
        }
    });

    if *DEBUG { println!("{} WindowTasks Thread ID: {} (delay: {})", "[ INFO ]".blue().bold(), format!("{:?}", window_tasks_thread.thread().id()).bold(), format!("{:?}", THREAD_DELAYS.window_tasks).bold()); }

    let toggled = TOGGLED.clone();
    let exit = EXIT.clone();
    let window_info = WINDOW_INFO.clone();
    let window_last_moved = WINDOW_LAST_MOVED.clone();
    let aimbot_toggled = AIMBOT_TOGGLED.clone();

    event_loop.run(move | event, _, control_flow | {
        let toggled_value = *toggled.lock().unwrap();
        let window_info_value = *window_info.lock().unwrap();

        let exit = *exit.lock().unwrap();
        let window_last_moved = *window_last_moved.lock().unwrap();
        let gl_window = display.gl_window();

        if window_last_moved.elapsed().as_millis() < 250 {
            gl_window.window().set_cursor_hittest(false).unwrap();
        } else {
            gl_window.window().set_cursor_hittest(toggled_value).unwrap();
        }

        if let Some(((x, y), (width, height))) = window_info_value {
            if window_last_moved.elapsed().as_millis() < 10 {
                gl_window.window().set_inner_size(PhysicalSize::new(width, height));
                gl_window.window().set_outer_position(PhysicalPosition::new(x, y));
            }
        }

        if exit {
            *control_flow = ControlFlow::Exit;
        }

        if !is_window_focused(window_hwnd) {
            (*aimbot_toggled.lock().unwrap()) = false;
        }

        match event {
            Event::NewEvents(_) => {
                let now = Instant::now();
                imgui_context.io_mut().update_delta_time(now.duration_since(last_frame));
                last_frame = now;
            },
            Event::MainEventsCleared => {
                winit_platform.prepare_frame(imgui_context.io_mut(), gl_window.window()).unwrap();
                gl_window.window().request_redraw();
            },
            Event::RedrawRequested(_) => {
                let ui = imgui_context.frame();
                let matrix_address = GAME.lock().unwrap().address.matrix;
                let controller_address = GAME.lock().unwrap().address.local_controller;
                let pawn_address = GAME.lock().unwrap().address.local_pawn;
                let mut skip = false;
                let mut no_pawn = false;

                if !skip {
                    if !read_memory(matrix_address, &mut (*GAME.lock().unwrap()).view.matrix, 64) {
                        skip = true;
                    }
                }

                if !skip {
                    update_entity_list_entry();
                }

                let mut local_controller_address = 0;
                let mut local_pawn_address = 0;

                if !skip {
                    if !read_memory_auto(controller_address, &mut local_controller_address) {
                        skip = true;
                    }
                }

                if !skip {
                    if !read_memory_auto(pawn_address, &mut local_pawn_address) {
                        skip = true;
                    }
                }

                let mut local_entity = Entity::default();
                let mut local_player_controller_index = 1;

                if !skip {
                    if !local_entity.update_controller(local_controller_address) {
                        skip = true;
                    }
                }

                if !skip {
                    if !local_entity.update_pawn(local_pawn_address) {
                        if !(*CONFIG.lock().unwrap()).show_when_spec {
                            skip = true;
                        };

                        no_pawn = true;
                    }
                }

                // Aimbot Data
                let mut max_aim_distance: f32 = 100000.0;
                let mut aim_pos = Vector3 { x: 0.0, y: 0.0, z: 0.0 };

                if !skip {
                    // Entities
                    for i in 0 .. 64 {
                        let mut entity = Entity::default();
                        let mut entity_address: u64 = 0;

                        if !read_memory_auto((*GAME.lock().unwrap()).address.entity_list_entry + (i + 1) * 0x78, &mut entity_address) {
                            continue;
                        }

                        if entity_address == local_entity.controller.address {
                            local_player_controller_index = i;
                            continue;
                        }

                        if !entity.update_controller(entity_address) {
                            continue;
                        }

                        if !entity.update_pawn(entity.pawn.address) {
                            continue;
                        }

                        if (*CONFIG.lock().unwrap()).team_check && entity.controller.team_id == local_entity.controller.team_id {
                            continue;
                        }

                        if !entity.is_alive() {
                            continue;
                        }

                        // [TODO] Does nothing until world_to_screen() is fixed.
                        if !entity.is_in_screen() {
                            continue;
                        }

                        if let Some(((_, _), (width, height))) = window_info_value {
                            if let Some(bone) = entity.get_bone() {
                                let pos = Vector2 { x: width as f32 / 2.0, y: height as f32 / 2.0 };
                                let distance_to_sight = distance_to_vec(bone.bone_pos_list[BoneIndex::Head as usize].screen_pos, pos);

                                if distance_to_sight < max_aim_distance {
                                    max_aim_distance = distance_to_sight;

                                    if !(*CONFIG.lock().unwrap()).visible_check || entity.pawn.b_spotted_by_mask & (1 << local_player_controller_index) != 0 || local_entity.pawn.b_spotted_by_mask & (1 << i) != 0 {
                                        if let Some(bone) = entity.get_bone() {
                                            let bone_index = aim_position_to_bone_index((*CONFIG.lock().unwrap()).aim_position);
                                            aim_pos = bone.bone_pos_list[bone_index].pos;

                                            if bone_index as usize == BoneIndex::Head as usize {
                                                aim_pos.z -= -1.0;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // FOV Circle
                    if !no_pawn && (*CONFIG.lock().unwrap()).show_aim_fov_range {
                        if let Some(((_, _), (width, height))) = window_info_value {
                            let center_point: Vector2<f32> = Vector2 { x: width as f32 / 2.0, y: height as f32 / 2.0 };
                            let radius = ((*CONFIG.lock().unwrap()).aim_fov / 180.0 * PI / 2.0).tan() / (local_entity.pawn.fov as f32 / 180.0 * PI / 2.0).tan() * width as f32;
                            ui.get_background_draw_list().add_circle(center_point, radius, color_u32_to_f32((*CONFIG.lock().unwrap()).aim_fov_range_color)).build();
                        }
                    }

                    // FOV
                    if !no_pawn && (*CONFIG.lock().unwrap()).show_fov_line {
                        if let Some(((_, _), (width, height))) = window_info_value {
                            let mut line_end_point: [Vector2<f32>; 2] = [Vector2 { x: 0.0, y: 0.0 }, Vector2 { x: 0.0, y: 0.0 }];
                            let pos: Vector2<f32> = Vector2 { x: width as f32 / 2.0, y: height as f32 / 2.0 };
                            let radian = (local_entity.pawn.fov as f32 / 2.0).to_radians();
                            let color = color_u32_to_f32((*CONFIG.lock().unwrap()).fov_line_color);

                            line_end_point[0].y = pos.y - (*CONFIG.lock().unwrap()).fov_line_size;
                            line_end_point[1].y = line_end_point[0].y;
                            
                            let length = (*CONFIG.lock().unwrap()).fov_line_size * radian.tan();

                            line_end_point[0].x = pos.x - length;
                            line_end_point[1].x = pos.x + length;

                            ui.get_background_draw_list().add_line(pos, line_end_point[0], color).build();
                            ui.get_background_draw_list().add_line(pos, line_end_point[1], color).build();
                        }
                    }

                    // Crosshair
                    if (*CONFIG.lock().unwrap()).show_crosshair {
                        if let Some(((_, _), (width, height))) = window_info_value {
                            let sight_pos: Vector2<f32> = Vector2 { x: width as f32 / 2.0, y: height as f32 / 2.0 };
                            let color = color_u32_to_f32((*CONFIG.lock().unwrap()).crosshair_color);

                            let line1_first = Vector2 { x: sight_pos.x - (*CONFIG.lock().unwrap()).crosshair_size, y: sight_pos.y };
                            let line1_second = Vector2 { x: sight_pos.x + (*CONFIG.lock().unwrap()).crosshair_size, y: sight_pos.y };

                            let line2_first = Vector2 { x: sight_pos.x, y: sight_pos.y - (*CONFIG.lock().unwrap()).crosshair_size };
                            let line2_second = Vector2 { x: sight_pos.x, y: sight_pos.y + (*CONFIG.lock().unwrap()).crosshair_size };

                            ui.get_background_draw_list().add_line(line1_first, line1_second, color).build();
                            ui.get_background_draw_list().add_line(line2_first, line2_second, color).build();
                        }
                    }

                    // Aimbot
                    if *aimbot_toggled.lock().unwrap() {
                        let local_pos = local_entity.pawn.camera_pos;
                        let opp_pos = Vector3 { x: aim_pos.x - local_pos.x, y: aim_pos.y - local_pos.y, z: aim_pos.z - local_pos.z };
                        let distance = f32::sqrt(f32::powf(opp_pos.x, 2.0) + f32::powf(opp_pos.y, 2.0));
                        let mut yaw = f32::atan2(opp_pos.y, opp_pos.x) * 57.295779513 - local_entity.pawn.view_angle.y;
                        let mut pitch = -f32::atan(opp_pos.z / distance) * 57.295779513 - local_entity.pawn.view_angle.x;
                        let norm = f32::sqrt(f32::powf(yaw, 2.0) + f32::powf(pitch, 2.0));
                        let mut ret = false;

                        if norm < (*CONFIG.lock().unwrap()).aim_fov {
                            yaw = yaw * (1.0 - (*CONFIG.lock().unwrap()).smooth) + local_entity.pawn.view_angle.y;
                            pitch = pitch * (1.0 - (*CONFIG.lock().unwrap()).smooth) + local_entity.pawn.view_angle.x;

                            if local_entity.pawn.shots_fired > (*CONFIG.lock().unwrap()).rcs_bullet as u64 {
                                let mut punch_angle = Vector2 { x: 0.0, y: 0.0 };

                                if local_entity.pawn.aim_punch_cache.count <= 0 && local_entity.pawn.aim_punch_cache.count > 0xFFFF {
                                    ret = true;
                                }

                                if !ret {
                                    if !read_memory_auto(local_entity.pawn.aim_punch_cache.data + (local_entity.pawn.aim_punch_cache.count - 1) * std::mem::size_of::<Vector3<f32>>() as u64, &mut punch_angle) {
                                        ret = true;
                                    }
                                }

                                if !ret {
                                    yaw = yaw - punch_angle.y * (*CONFIG.lock().unwrap()).rcs_scale.0;
                                    pitch = pitch - punch_angle.x * (*CONFIG.lock().unwrap()).rcs_scale.1;
                                }
                            }

                            if !ret {
                                set_view_angle(yaw, pitch);
                            }
                        }
                    }
                }

                if !skip {
                    render_radar(ui);
                }
                
                render_menu(ui);

                let mut target = display.draw();
                target.clear_color_srgb(0.0, 0.0, 0.0, 0.0);
                winit_platform.prepare_render(ui, gl_window.window());

                let draw_data = imgui_context.render();
                renderer.render(&mut target, draw_data).unwrap();
                target.finish().unwrap();
            },
            Event::DeviceEvent {
                event: DeviceEvent::Key(key),
                ..
            } => {
                if let Some(keycode) = key.virtual_keycode {
                    if &keycode == toggle_key && key.state == ElementState::Released {
                        *toggled.lock().unwrap() = !toggled_value;

                        if toggled_value {
                            focus_window(window_hwnd);
                        } else {
                            focus_window(self_hwnd);
                        }
                    }
                }
            },
            Event::WindowEvent {
                event: WindowEvent::CloseRequested | WindowEvent::Destroyed,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            },
            event => {
                winit_platform.handle_event(imgui_context.io_mut(), gl_window.window(), &event);
            }
        };
    });
}