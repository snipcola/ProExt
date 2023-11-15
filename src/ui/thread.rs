use std::{time::Instant, thread::{self, sleep}, sync::{Arc, Mutex}};

use imgui::Context;
use imgui_dx11_renderer::Renderer;
use imgui_winit_support::WinitPlatform;
use windows::Win32::{Foundation::HWND, Graphics::{Direct3D11::{ID3D11DeviceContext, ID3D11Device, ID3D11RenderTargetView}, Dxgi::IDXGISwapChain}};
use winit::{event_loop::{EventLoop, ControlFlow}, window::Window, dpi::{PhysicalSize, PhysicalPosition}, event::{Event, DeviceEvent, ElementState, WindowEvent}};

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

pub fn run_event_loop(window_info: Arc<Mutex<(EventLoop<()>, Window, Option<ID3D11DeviceContext>, ID3D11Device, IDXGISwapChain, Option<ID3D11RenderTargetView>)>>, imgui_info: Arc<Mutex<(WinitPlatform, Context)>>, window_hwnd: HWND, self_hwnd: HWND) {
    let window_size_info = WINDOW_INFO.clone();
    let window_last_moved = WINDOW_LAST_MOVED.clone();

    let ui_functions = UI_FUNCTIONS.clone();
    let toggled = TOGGLED.clone();
    let exit = EXIT.clone();

    let (event_loop, window, device_ctx, device, swapchain, target) = Arc::try_unwrap(window_info).unwrap().into_inner().unwrap();
    let (mut platform, mut imgui) = Arc::try_unwrap(imgui_info).unwrap().into_inner().unwrap();
    
    let mut renderer = unsafe { Renderer::new(&mut imgui, &device).unwrap() };
    let mut last_frame = Instant::now();
    let toggle_key = &*TOGGLE_KEY;

    run_windows_thread(window_hwnd, self_hwnd);

    event_loop.run(move | event, _, control_flow | {
        let toggled_value = *toggled.lock().unwrap();
        let window_last_moved = *window_last_moved.lock().unwrap();

        if window_last_moved.elapsed().as_millis() < 250 {
            // gl_window.window().set_cursor_hittest(false).unwrap();
        } else {
            // gl_window.window().set_cursor_hittest(toggled_value).unwrap();
        }

        if let Some(((x, y), (width, height))) = *window_size_info.lock().unwrap() {
            if window_last_moved.elapsed().as_millis() < 10 {
                window.set_inner_size(PhysicalSize::new(width, height));
                window.set_outer_position(PhysicalPosition::new(x, y));
            }
        }

        if *exit.lock().unwrap() {
            *control_flow = ControlFlow::Exit;
        }

        match event {
            Event::NewEvents(_) => {
                let now = Instant::now();
                imgui.io_mut().update_delta_time(now.duration_since(last_frame));
                last_frame = now;
            },
            Event::MainEventsCleared => {
                platform.prepare_frame(imgui.io_mut(), &window).unwrap();
                window.request_redraw();
            },
            Event::RedrawRequested(_) => {
                unsafe {
                    if let Some(ref context) = device_ctx {
                        context.OMSetRenderTargets(&[target.clone()], None);
                        context.ClearRenderTargetView(target.as_ref().unwrap(), &0.6);
                    }
                }

                let mut ui = imgui.frame();

                for (_, function) in (*ui_functions.lock().unwrap()).iter() {
                    function(&mut ui);
                }

                render_menu(&mut ui);

                platform.prepare_render(&mut ui, &window);
                renderer.render(ui.render()).unwrap();
                unsafe { swapchain.Present(1, 0).unwrap() };
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
                platform.handle_event(imgui.io_mut(), &window, &event);
            }
        };
    });
}