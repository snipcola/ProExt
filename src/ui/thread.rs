use std::{time::Instant, thread::{self, sleep}, sync::{Arc, Mutex}, process};
use glow::{HasContext, COLOR_BUFFER_BIT};
use glutin::{event_loop::{EventLoop, ControlFlow}, dpi::{PhysicalSize, PhysicalPosition}, event::{Event, DeviceEvent, ElementState, WindowEvent}};
use imgui::Context;
use imgui_winit_support::WinitPlatform;
use windows::Win32::Foundation::HWND;
use lazy_static::lazy_static;
use imgui_glow_renderer::AutoRenderer;

use crate::{ui::{main::{WINDOW_INFO, EXIT, TOGGLED, UI_FUNCTIONS}, windows::{get_window_info, is_window_focused}, menu::render_menu, functions::apply_style}, utils::{mouse::get_mouse_position, rpc::initialize_rpc}, cheat::thread::run_cheats_thread};
use crate::utils::config::ProgramConfig;
use crate::ui::main::WINDOWS_ACTIVE;
use crate::ui::windows::Window;
use crate::ui::windows::get_glow_context;

lazy_static! {
    pub static ref MOUSE_POS: Arc<Mutex<Option<(i32, i32)>>> = Arc::new(Mutex::new(None));
}

pub fn run_windows_thread(hwnd: HWND) {
    thread::spawn(move || {
        let window_info = WINDOW_INFO.clone();
        let exit = EXIT.clone();

        loop {
            if let Some(((x, y), (width, height))) = get_window_info(hwnd) {
                let window_info_var = ((x + 1, y + 1), (width - 2, height - 2));
                *window_info.lock().unwrap() = Some(window_info_var);
            } else {
                *exit.lock().unwrap() = true;
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

    ProgramConfig::Keys::ToggleKeyMKI.bind(move | _ | {
        if !is_window_focused(hwnd) {
            return;
        }

        let toggled_value = *toggled.lock().unwrap();
        *toggled.lock().unwrap() = !toggled_value;
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
    let ui_functions = UI_FUNCTIONS.clone();

    let toggled = TOGGLED.clone();
    let exit = EXIT.clone();

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

        window.window().set_cursor_hittest(toggled_value && (*WINDOWS_ACTIVE.lock().unwrap()).values().any(| val | val == &true)).ok();

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
                    renderer.gl_context().clear_color(0.0, 0.0, 0.0, 0.0);
                    renderer.gl_context().clear(COLOR_BUFFER_BIT);
                }

                apply_style(imgui_context.style_mut(), default_style);

                let ui = imgui_context.new_frame();

                if toggled_value {
                    render_menu(ui);
                }

                for (_, function) in (*ui_functions.lock().unwrap()).iter() {
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
                    if keycode == ProgramConfig::Keys::ToggleKey && key.state == ElementState::Pressed {
                        *toggled.lock().unwrap() = !toggled_value;
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