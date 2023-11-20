use std::ops::BitAnd;
use imgui::{Style, ImColor32, Ui, ColorEditFlags, StyleColor};
use mint::{Vector2, Vector3, Vector4};
use mki::{Mouse, Keyboard};

use crate::utils::config::CONFIG;

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
        return Ok(Mouse::Side);
    }
    else if hotkey_index == 5 {
        return Ok(Mouse::Extra);
    }
    else if hotkey_index == 6 {
        return Err(Keyboard::LeftShift);
    }
    else if hotkey_index == 7 {
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
    return (red.bitand(alpha) as f32 / 255.0, green.bitand(alpha) as f32 / 255.0, blue.bitand(alpha) as f32 / 255.0);
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

fn color_to_style_color(color: (u32, u32, u32, u32)) -> [f32; 4] {
    return [color.0 as f32 / 255.0, color.1 as f32 / 255.0, color.2 as f32 / 255.0, color.3 as f32 / 255.0];
}

pub fn apply_style(style: &mut Style) {
    let config = CONFIG.clone().lock().unwrap().clone();

    // Alpha
    style.alpha = config.style.alpha;

    // Window
    style.window_padding = config.style.window_padding;
    style.window_rounding = config.style.window_rounding;
    style.window_border_size = config.style.window_border_size;
    style.window_title_align = config.style.window_title_align;

    // Frame
    style.frame_padding = config.style.frame_padding;
    style.frame_rounding = config.style.frame_rounding;
    style.frame_border_size = config.style.frame_border_size;

    // Tab
    style.tab_rounding = config.style.tab_rounding;
    style.tab_border_size = config.style.tab_border_size;

    // Scrollbar
    style.scrollbar_rounding = config.style.scrollbar_rounding;
    style.scrollbar_size = config.style.scrollbar_size;

    // Popup
    style.popup_rounding = config.style.popup_rounding;
    style.popup_border_size = config.style.popup_border_size;

    // Item
    style.item_spacing = config.style.item_spacing;
    style.item_inner_spacing = config.style.item_inner_spacing;
    style.indent_spacing = config.style.indent_spacing;

    // Grab
    style.grab_rounding = config.style.grab_rounding;

    // Colors
    style.colors[StyleColor::Text as usize] = color_to_style_color(config.style.colors.text);
    style.colors[StyleColor::TextDisabled as usize] = color_to_style_color(config.style.colors.text_disabled);
    
    style.colors[StyleColor::WindowBg as usize] = color_to_style_color(config.style.colors.window_bg);
    style.colors[StyleColor::ChildBg as usize] = color_to_style_color(config.style.colors.child_bg);
    style.colors[StyleColor::PopupBg as usize] = color_to_style_color(config.style.colors.popup_bg);

    style.colors[StyleColor::Border as usize] = color_to_style_color(config.style.colors.border);
    style.colors[StyleColor::BorderShadow as usize] = color_to_style_color(config.style.colors.border_shadow);

    style.colors[StyleColor::FrameBg as usize] = color_to_style_color(config.style.colors.frame_bg);
    style.colors[StyleColor::FrameBgHovered as usize] = color_to_style_color(config.style.colors.frame_bg_hovered);
    style.colors[StyleColor::FrameBgActive as usize] = color_to_style_color(config.style.colors.frame_bg_active);

    style.colors[StyleColor::TitleBg as usize] = color_to_style_color(config.style.colors.title_bg);
    style.colors[StyleColor::TitleBgCollapsed as usize] = color_to_style_color(config.style.colors.title_bg_collapsed);
    style.colors[StyleColor::TitleBgActive as usize] = color_to_style_color(config.style.colors.title_bg_active);

    style.colors[StyleColor::TextSelectedBg as usize] = color_to_style_color(config.style.colors.text_selected_bg);
    style.colors[StyleColor::CheckMark as usize] = color_to_style_color(config.style.colors.checkmark);

    style.colors[StyleColor::ScrollbarBg as usize] = color_to_style_color(config.style.colors.scrollbar_bg);
    style.colors[StyleColor::ScrollbarGrab as usize] = color_to_style_color(config.style.colors.scrollbar_grab);
    style.colors[StyleColor::ScrollbarGrabHovered as usize] = color_to_style_color(config.style.colors.scrollbar_grab_hovered);
    style.colors[StyleColor::ScrollbarGrabActive as usize] = color_to_style_color(config.style.colors.scrollbar_grab_active);

    style.colors[StyleColor::SliderGrab as usize] = color_to_style_color(config.style.colors.slider_grab);
    style.colors[StyleColor::SliderGrabActive as usize] = color_to_style_color(config.style.colors.slider_grab_active);

    style.colors[StyleColor::Button as usize] = color_to_style_color(config.style.colors.button);
    style.colors[StyleColor::ButtonHovered as usize] = color_to_style_color(config.style.colors.button_hovered);
    style.colors[StyleColor::ButtonActive as usize] = color_to_style_color(config.style.colors.button_active);

    style.colors[StyleColor::Tab as usize] = color_to_style_color(config.style.colors.tab);
    style.colors[StyleColor::TabHovered as usize] = color_to_style_color(config.style.colors.tab_hovered);
    style.colors[StyleColor::TabActive as usize] = color_to_style_color(config.style.colors.tab_active);

    style.colors[StyleColor::Separator as usize] = color_to_style_color(config.style.colors.seperator);
}