use std::{sync::{Arc, Mutex}, time::SystemTime};
use imgui::Ui;
use mint::Vector4;
use lazy_static::lazy_static;
use crate::{utils::config::{CONFIG, Config, ProgramConfig}, ui::{main::WINDOWS_ACTIVE, functions::color_u32_to_f32}};

lazy_static!{ 
    pub static ref WATERMARK_RESET_POSITION: Arc<Mutex<Option<[f32; 2]>>> = Arc::new(Mutex::new(None));
}

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
    let mut reset_position = WATERMARK_RESET_POSITION.lock().unwrap();
    let (window_position, condition) = if let Some(position) = *reset_position {
        *reset_position = None;
        (position, imgui::Condition::Always)
    } else {
        (config.window_positions.watermark, imgui::Condition::Once)
    };

    drop(reset_position);

    ui.window(format!("{} v{}", ProgramConfig::Package::Name, ProgramConfig::Package::Version))
        .collapsible(false)
        .always_auto_resize(true)
        .position(window_position, condition)
        .build(|| {
            (*WINDOWS_ACTIVE.lock().unwrap()).insert("watermark".to_string(), ui.is_window_hovered());
            (*CONFIG.lock().unwrap()).window_positions.watermark = ui.window_pos();

            let watermark_one_f32 = color_u32_to_f32(config.misc.watermark_color_one);
            let watermark_one_color = Vector4 { x: watermark_one_f32.0, y: watermark_one_f32.1, z: watermark_one_f32.2, w: watermark_one_f32.3 };

            let watermark_two_f32 = color_u32_to_f32(config.misc.watermark_color_two);
            let watermark_two_color = Vector4 { x: watermark_two_f32.0, y: watermark_two_f32.1, z: watermark_two_f32.2, w: watermark_two_f32.3 };

            ui.text_colored(watermark_one_color, get_current_time());
            ui.same_line();
            ui.text_colored(watermark_two_color, format!("{} FPS", ui.io().framerate.floor()));
        });
}