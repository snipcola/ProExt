// Copyright (c) 2023 Vytrol <vytrol@proton.me>
// SPDX-License-Identifier: MIT

use std::process::Command;

pub fn open_url(url: &str) {
    Command::new("cmd.exe")
        .args(["/C", "start", url])
        .spawn()
        .ok();
}