use std::{time::Instant, thread::{self, sleep}, sync::{Arc, Mutex}};

use glium::{glutin::{event_loop::{EventLoop, ControlFlow}, dpi::{PhysicalSize, PhysicalPosition}, event::{Event, DeviceEvent, ElementState, WindowEvent}}, Display, Surface};
use imgui::Context;
use imgui_glium_renderer::Renderer;
use imgui_winit_support::WinitPlatform;
use windows::Win32::Foundation::HWND;

use crate::ui::{main::{WINDOW_FOCUSED, WINDOW_INFO, WINDOW_LAST_MOVED, EXIT, TOGGLED, UI_FUNCTIONS}, windows::{get_window_info, is_window_focused, focus_window}, menu::render_menu};
use crate::utils::config::THREAD_DELAYS;
use crate::utils::config::{TOGGLE_KEY_MKI, TOGGLE_KEY};

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