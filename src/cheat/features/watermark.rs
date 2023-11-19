use std::time::SystemTime;
use imgui::{Ui, StyleColor};
use mint::Vector4;

use crate::{utils::config::{PACKAGE_NAME, WindowPosition, CONFIG, Config, PACKAGE_VERSION}, ui::main::WINDOWS_ACTIVE};

pub fn get_current_time() -> String {
    let now = SystemTime::now();

    if let Ok(duration) = now.duration_since(SystemTime::UNIX_EPOCH) {
        let secs = duration.as_secs();
        return format!("{:02}:{:02}:{:02}", (secs / 3600) % 24, (secs / 60) % 60, secs % 60);
    } else {
        return "00:00:00".to_string();
    }
}

pub fn render_watermark(ui: &mut Ui, config: Config) {
    let window_position = config.window_positions.watermark;
    let titlebar_color = ui.push_style_color(StyleColor::TitleBgActive, [0.01, 0.01, 0.01, 1.0]);

    ui.window(format!("{} v{}", PACKAGE_NAME.to_string(), PACKAGE_VERSION.to_string()))
        .collapsible(false)
        .always_auto_resize(true)
        .position([window_position.x, window_position.y], imgui::Condition::Appearing)
        .build(|| {
            let window_active = ui.is_window_hovered();
            (*WINDOWS_ACTIVE.lock().unwrap()).insert("watermark".to_string(), window_active);

            let window_pos = ui.window_pos();
            let mut config_mut = CONFIG.lock().unwrap();
            (*config_mut).window_positions.watermark = WindowPosition { x: window_pos[0], y: window_pos[1] };
            drop(config_mut);

            ui.text_colored(Vector4 { x: 255.0, y: 255.0, z: 0.0, w: 255.0 }, get_current_time());
            ui.same_line();
            ui.text_colored(Vector4 { x: 0.0, y: 255.0, z: 0.0, w: 255.0 }, format!("{} FPS", ui.io().framerate.floor()));
        });

    titlebar_color.end();
}