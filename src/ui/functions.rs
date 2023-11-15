use std::{time::Instant, thread::{self, sleep}, sync::{Arc, Mutex}};

use glium::{glutin::{event_loop::{EventLoop, ControlFlow}, dpi::{PhysicalSize, PhysicalPosition}, event::{Event, DeviceEvent, ElementState, WindowEvent}}, Display, Surface};
use imgui::{ImColor32, Ui, ColorEditFlags, Context};
use imgui_glium_renderer::Renderer;
use imgui_winit_support::WinitPlatform;
use mint::{Vector2, Vector3, Vector4};
use mki::{Mouse, Keyboard};
use windows::Win32::Foundation::HWND;

use crate::ui::{main::{WINDOW_FOCUSED, WINDOW_INFO, WINDOW_LAST_MOVED, EXIT, TOGGLED, UI_FUNCTIONS}, windows::{get_window_info, is_window_focused, focus_window}, menu::render_menu};
use crate::utils::config::THREAD_DELAYS;
use crate::utils::config::{TOGGLE_KEY_MKI, TOGGLE_KEY};

pub fn hotkey_index_to_io(hotkey_index: usize) -> Result<Mouse, Keyboard> {
    if hotkey_index == 1 {
        return Ok(Mouse::Left);
    }
    else if hotkey_index == 2 {
        return Ok(Mouse::Middle);
    }
    else if hotkey_index == 3 {
        return Ok(Mouse::Right);
    }
    else if hotkey_index == 4 {
        return Err(Keyboard::LeftShift);
    }
    else if hotkey_index == 5 {
        return Err(Keyboard::LeftControl);
    }
    else {
        return Err(Keyboard::LeftAlt);
    }
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

pub fn color_with_alpha((red, green, blue, _): (u32, u32, u32, u32), alpha: f32) -> (f32, f32, f32, f32) {
    return (red as f32 / 255.0, green as f32 / 255.0, blue as f32 / 255.0, alpha);
}

pub fn color_with_masked_alpha((red, green, blue, _): (u32, u32, u32, u32), alpha: u32) -> (f32, f32, f32) {
    return ((red & alpha) as f32 / 255.0, (green & alpha) as f32 / 255.0, (blue & alpha) as f32 / 255.0);
}

pub fn mix_colors(color_1: ImColor32, color_2: ImColor32, t: f32) -> ImColor32 {
    return ImColor32::from_rgba_f32s(t * color_1.r as f32 / 255.0 + (1.0 - t) * color_2.r as f32 / 255.0, t * color_1.g as f32 / 255.0 + (1.0 - t) * color_2.g as f32 / 255.0, t * color_1.b as f32 / 255.0 + (1.0 - t) * color_2.b as f32 / 255.0, color_1.a as f32 / 255.0);
}

pub fn distance_between_vec2(pos1: Vector2<f32>, pos2: Vector2<f32>) -> f32 {
    let x_diff = pos2.x - pos1.x;
    let y_diff = pos2.y - pos1.y;
    let distance = (x_diff.powi(2) + y_diff.powi(2)).sqrt();

    return distance;
}

pub fn distance_between_vec3(pos1: Vector3<f32>, pos2: Vector3<f32>) -> f32 {
    let x_diff = pos2.x - pos1.x;
    let y_diff = pos2.y - pos1.y;
    let z_diff = pos2.z - pos1.z;
    let distance = (x_diff.powi(2) + y_diff.powi(2) + z_diff.powi(2)).sqrt();

    return distance;
}

pub fn rectangle(ui: &mut Ui, pos: Vector2<f32>, size: Vector2<f32>, color: ImColor32, thickness: f32, rounding: u32, filled: bool) {
    ui.get_background_draw_list().add_rect(pos, Vector2 { x: pos.x + size.x, y: pos.y + size.y }, color).thickness(thickness).rounding(rounding as f32).filled(filled).build();
}

pub fn text(ui: &mut Ui, text: String, pos: Vector2<f32>, color: ImColor32, keep_center: bool) {
    if !keep_center {
        ui.get_background_draw_list().add_text(pos, color, text);
    } else {
        let text_width = ui.calc_text_size_with_opts(text.clone(), false, 0.0)[0];
        ui.get_background_draw_list().add_text(Vector2 { x: pos.x - text_width / 2.0, y: pos.y }, color, text);
    }
}

pub fn stroke_text(ui: &mut Ui, _text: String, pos: Vector2<f32>, color: ImColor32, keep_center: bool) {
    text(ui, _text.clone(), Vector2 { x: pos.x - 1.0, y: pos.y + 1.0 }, ImColor32::from_rgb(0, 0, 0), keep_center);
    text(ui, _text.clone(), Vector2 { x: pos.x - 1.0, y: pos.y - 1.0 }, ImColor32::from_rgb(0, 0, 0), keep_center);
    text(ui, _text.clone(), Vector2 { x: pos.x - 1.0, y: pos.y - 1.0 }, ImColor32::from_rgb(0, 0, 0), keep_center);
    text(ui, _text.clone(), Vector2 { x: pos.x + 1.0, y: pos.y + 1.0 }, ImColor32::from_rgb(0, 0, 0), keep_center);
    text(ui, _text.clone(), Vector2 { x: pos.x + 1.0, y: pos.y - 1.0 }, ImColor32::from_rgb(0, 0, 0), keep_center);
    text(ui, _text, pos, color, keep_center);
}

pub fn run_windows_thread(window_hwnd: HWND, self_hwnd: HWND) {
    let window_focused = WINDOW_FOCUSED.clone();
    let window_info = WINDOW_INFO.clone();
    let window_last_moved = WINDOW_LAST_MOVED.clone();
    let exit = EXIT.clone();

    let mut stored_window_info: ((i32, i32), (i32, i32)) = ((0, 0), (0, 0));

    thread::spawn(move || {
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
}

pub fn bind_ui_keys(window_hwnd: HWND, self_hwnd: HWND) {
    let toggled = TOGGLED.clone();
    let window_focused = WINDOW_FOCUSED.clone();

    (*TOGGLE_KEY_MKI).bind(move | _ | {
        if !*window_focused.lock().unwrap() {
            return;
        }

        let toggled_value = *toggled.lock().unwrap();
        *toggled.lock().unwrap() = !toggled_value;
        
        if toggled_value {
            focus_window(window_hwnd);
        } else {
            focus_window(self_hwnd);
        }
    });
}

pub fn run_event_loop(event_loop_display: Arc<Mutex<(EventLoop<()>, Display)>>, winit_platform_imgui_context: Arc<Mutex<(WinitPlatform, Context)>>, window_hwnd: HWND, self_hwnd: HWND) {
    let window_info = WINDOW_INFO.clone();
    let window_last_moved = WINDOW_LAST_MOVED.clone();

    let ui_functions = UI_FUNCTIONS.clone();
    let toggled = TOGGLED.clone();
    let exit = EXIT.clone();

    let (event_loop, display) = Arc::try_unwrap(event_loop_display).unwrap().into_inner().unwrap();
    let (mut winit_platform, mut imgui_context) = Arc::try_unwrap(winit_platform_imgui_context).unwrap().into_inner().unwrap();
    let mut renderer = Renderer::init(&mut imgui_context, &display).unwrap();

    let toggle_key = &*TOGGLE_KEY;
    let mut last_frame = Instant::now();

    run_windows_thread(window_hwnd, self_hwnd);

    event_loop.run(move | event, _, control_flow | {
        let toggled_value = *toggled.lock().unwrap();
        let window_last_moved = *window_last_moved.lock().unwrap();
        let gl_window = display.gl_window();

        if window_last_moved.elapsed().as_millis() < 250 {
            gl_window.window().set_cursor_hittest(false).unwrap();
        } else {
            gl_window.window().set_cursor_hittest(toggled_value).unwrap();
        }

        if let Some(((x, y), (width, height))) = *window_info.lock().unwrap() {
            if window_last_moved.elapsed().as_millis() < 10 {
                gl_window.window().set_inner_size(PhysicalSize::new(width, height));
                gl_window.window().set_outer_position(PhysicalPosition::new(x, y));
            }
        }

        if *exit.lock().unwrap() {
            *control_flow = ControlFlow::Exit;
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

                for (_, function) in (*ui_functions.lock().unwrap()).iter() {
                    function(ui);
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
                    if &keycode == toggle_key && key.state == ElementState::Pressed {
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