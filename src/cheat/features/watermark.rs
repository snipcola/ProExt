use chrono::Local;
use imgui::Ui;
use mint::Vector4;

use crate::utils::config::{PACKAGE_NAME, WindowPosition, CONFIG, Config};

pub fn render_watermark(ui: &mut Ui, config: Config) {
    let window_position = config.window_positions.watermark;

    ui.window("Watermark")
        .resizable(false)
        .collapsible(false)
        .scroll_bar(false)
        .title_bar(false)
        .always_auto_resize(true)
        .position([window_position.x, window_position.y], imgui::Condition::Appearing)
        .build(|| {
            let window_pos = ui.window_pos();
            let mut config_mut = CONFIG.lock().unwrap();
            (*config_mut).window_positions.watermark = WindowPosition { x: window_pos[0], y: window_pos[1] };
            drop(config_mut);

            ui.text_colored(Vector4 { x: 0.0, y: 255.0, z: 255.0, w: 255.0 }, PACKAGE_NAME.to_string());
            ui.same_line();
            ui.text_colored(Vector4 { x: 255.0, y: 255.0, z: 0.0, w: 255.0 }, format!("{}", Local::now().format("%H:%M:%S")));
            ui.same_line();
            ui.text_colored(Vector4 { x: 0.0, y: 255.0, z: 0.0, w: 255.0 }, format!("{} FPS", f32::floor(ui.io().framerate)));
        });
}