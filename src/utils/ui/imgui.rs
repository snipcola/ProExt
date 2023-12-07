// Copyright (c) 2023 Vytrol <vytrol@proton.me>
// SPDX-License-Identifier: MIT

use std::fs::read;

use imgui::{Context, FontSource, FontConfig, FontGlyphRanges};
use imgui_winit_support::{WinitPlatform, HiDpiMode};

use crate::config::ProgramConfig;
use crate::utils::ui::windows::Window;

pub fn add_imgui_default_font(fonts: &mut Vec<FontSource>, font_size: f32) {
    fonts.push(FontSource::DefaultFontData {
        config: Some(FontConfig { size_pixels: font_size, oversample_h: 3, oversample_v: 3, ..Default::default() })
    });
}

pub fn add_imgui_font(fonts: &mut Vec<FontSource>, font_path: &str, font_size: f32, range: FontGlyphRanges) {
    let buffer = match read(font_path) {
        Ok(file) => file,
        Err(_) => return
    };

    let slice: &'static [u8] = Box::leak(buffer.into_boxed_slice());

    fonts.push(FontSource::TtfData {
        data: slice,
        size_pixels: font_size,
        config: Some(FontConfig { glyph_ranges: range, oversample_h: 3, oversample_v: 3, ..Default::default() }),
    });
}

pub fn init_imgui(window: &Window) -> (WinitPlatform, Context) {
    let mut imgui_context = Context::create();
    imgui_context.set_ini_filename(None);
    imgui_context.set_log_filename(None);

    let mut winit_platform = WinitPlatform::init(&mut imgui_context);
    winit_platform.attach_window(imgui_context.io_mut(), window.window(), HiDpiMode::Locked(1.0));

    let font_size = ProgramConfig::Imgui::FontSize;
    let mut fonts = vec![];

    add_imgui_default_font(&mut fonts, font_size);
    add_imgui_font(&mut fonts, ProgramConfig::Imgui::FontPaths::Chinese, font_size, FontGlyphRanges::chinese_full());
    add_imgui_font(&mut fonts, ProgramConfig::Imgui::FontPaths::Cryillic, font_size, FontGlyphRanges::cyrillic());
    add_imgui_font(&mut fonts, ProgramConfig::Imgui::FontPaths::Arabic, font_size, FontGlyphRanges::from_slice(&[0x600, 0x6FF, 0]));

    imgui_context.fonts().add_font(&fonts);
    imgui_context.io_mut().font_global_scale = (1.0 / winit_platform.hidpi_factor()) as f32;

    return (winit_platform, imgui_context);
}