// Copyright (c) 2024 Snipcola
// SPDX-License-Identifier: MIT

use std::time::Instant;
use std::thread::{self, sleep};
use std::sync::{Arc, Mutex};
use std::process;

use lazy_static::lazy_static;

use glow::{HasContext, COLOR_BUFFER_BIT};
use mki::{Action, State, InhibitEvent};

use glutin::event_loop::{EventLoop, ControlFlow};
use glutin::dpi::{PhysicalSize, PhysicalPosition};
use glutin:: event::{Event, DeviceEvent, ElementState, WindowEvent};

use imgui::Context;
use imgui_winit_support::WinitPlatform;
use imgui_glow_renderer::AutoRenderer;

use windows::Win32::Foundation::HWND;

use crate::config::ProgramConfig;
use crate::cheat::thread::run_cheats_thread;

use crate::ui::main::{WINDOW_INFO, EXIT, TOGGLED, RENDER_LIST, BG_ALPHA};
use crate::ui::functions::apply_style;
use crate::ui::menu::render_menu;

use crate::utils::ui::windows::{Window, get_glow_context, focus_window, get_window_info, is_window_focused};
use crate::utils::cheat::rpc::initialize_rpc;
use crate::utils::mouse::get_mouse_position;

lazy_static! {
    pub static ref MOUSE_POS: Arc<Mutex<Option<(i32, i32)>>> = Arc::new(Mutex::new(None));
    pub static ref FOCUS_SELF: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
}

pub fn run_windows_thread(hwnd: HWND) {
    thread::spawn(move || {
        let window_offset = 2;
        let window_info = WINDOW_INFO.clone();
        let exit = EXIT.clone();

        loop {
            if let Some(((x, y), (width, height))) = get_window_info(hwnd) {
                *window_info.lock().unwrap() = Some(((x + window_offset, y + window_offset), (width - (window_offset * 2), height - (window_offset * 2))));
            } else {
                *exit.lock().unwrap() = true;
                break;
            }
            
            // Delay
            sleep(ProgramConfig::ThreadDelays::WindowTasks);
        }
    });
}

pub fn run_io_thread() {
    thread::spawn(move || {
        let mouse_pos = MOUSE_POS.clone();
        
        loop {
            if let Some(pos) = get_mouse_position() {
                *mouse_pos.lock().unwrap() = Some(pos);
            }

            // Delay
            sleep(ProgramConfig::ThreadDelays::IOTasks);
        }
    });
}

pub fn bind_ui_keys(hwnd: HWND) {
    let toggled = TOGGLED.clone();
    let focused = FOCUS_SELF.clone();

    ProgramConfig::Keys::ToggleKeyMKI.act_on(Action {
        callback: Box::new(move | _, state | {
            if state == State::Released {
                if !is_window_focused(hwnd) {
                    return;
                }
        
                let toggled_value = *toggled.lock().unwrap();
                *toggled.lock().unwrap() = !toggled_value;
        
                if !toggled_value {
                    *focused.lock().unwrap() = true;
                } else {
                    focus_window(hwnd);
                }
            }
        }),
        inhibit: InhibitEvent::No,
        defer: true,
        sequencer: false
    });

    ProgramConfig::Keys::ExitKeyMKI.bind(move | _ | {
        if !is_window_focused(hwnd) {
            return;
        }

        process::exit(0);
    });
}

pub fn run_event_loop(event_loop_window: Arc<Mutex<(EventLoop<()>, Window)>>, winit_platform_imgui_context: Arc<Mutex<(WinitPlatform, Context)>>, hwnd: HWND, self_hwnd: HWND) {
    let window_info = WINDOW_INFO.clone();
    let render_list = RENDER_LIST.clone();

    let toggled = TOGGLED.clone();
    let exit = EXIT.clone();
    let focus = FOCUS_SELF.clone();

    let (event_loop, window) = Arc::try_unwrap(event_loop_window).unwrap().into_inner().unwrap();
    let (mut winit_platform, mut imgui_context) = Arc::try_unwrap(winit_platform_imgui_context).unwrap().into_inner().unwrap();

    let glow_context = get_glow_context(&window);
    let mut renderer = AutoRenderer::initialize(glow_context, &mut imgui_context).unwrap();

    let mut last_frame = Instant::now();
    let default_style = *imgui_context.style();

    run_windows_thread(hwnd);
    run_io_thread();
    run_cheats_thread(hwnd, self_hwnd);

    bind_ui_keys(hwnd);
    initialize_rpc();

    window.window().set_visible(true);
    
    event_loop.run(move | event, _, control_flow | {
        let toggled_value = *toggled.lock().unwrap();

        let io = imgui_context.io_mut();
        let mut mouse_pos = MOUSE_POS.lock().unwrap();

        window.window().set_cursor_hittest(toggled_value).ok();

        if let Some(((x, y), (width, height))) = *window_info.lock().unwrap() {
            if let Some(pos) = *mouse_pos {    
                *mouse_pos = None;
                io.add_mouse_pos_event([(pos.0 - x) as f32, (pos.1 - y) as f32]);
            }

            window.window().set_inner_size(PhysicalSize::new(width, height));
            window.window().set_outer_position(PhysicalPosition::new(x, y));
        }

        if *exit.lock().unwrap() {
            *control_flow = ControlFlow::Exit;
        }

        if *focus.lock().unwrap() {
            *focus.lock().unwrap() = false;
            window.window().focus_window();
        }

        match event {
            Event::NewEvents(_) => {
                let now = Instant::now();
                imgui_context.io_mut().update_delta_time(now.duration_since(last_frame));
                last_frame = now;
            },
            Event::MainEventsCleared => {
                winit_platform.prepare_frame(imgui_context.io_mut(), window.window()).unwrap();
                window.window().request_redraw();
            },
            Event::RedrawRequested(_) => {
                unsafe {
                    if toggled_value {
                        renderer.gl_context().clear_color(0.0, 0.0, 0.0, *BG_ALPHA.lock().unwrap());
                    } else {
                        renderer.gl_context().clear_color(0.0, 0.0, 0.0, 0.0);
                    }

                    renderer.gl_context().clear(COLOR_BUFFER_BIT);
                }

                apply_style(imgui_context.style_mut(), default_style);

                let ui = imgui_context.new_frame();

                if toggled_value {
                    render_menu(ui);
                }

                for (_, function) in (*render_list.lock().unwrap()).iter() {
                    function(ui);
                }

                winit_platform.prepare_render(ui, window.window());                
                renderer.render(imgui_context.render()).unwrap();
                window.swap_buffers().unwrap();
            },
            Event::DeviceEvent {
                event: DeviceEvent::Key(key),
                ..
            } => {
                if let Some(keycode) = key.virtual_keycode {
                    if keycode == ProgramConfig::Keys::ToggleKey && key.state == ElementState::Released {
                        *toggled.lock().unwrap() = !toggled_value;
                        
                        if !toggled_value {
                            window.window().focus_window();
                        } else {
                            focus_window(hwnd);
                        }
                    } else if keycode == ProgramConfig::Keys::ExitKey && key.state == ElementState::Pressed {
                        process::exit(0);
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
                winit_platform.handle_event(imgui_context.io_mut(), window.window(), &event);
            }
        };
    });
}