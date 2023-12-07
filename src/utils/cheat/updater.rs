// Copyright (c) 2023 Vytrol <vytrol@proton.me>
// SPDX-License-Identifier: MIT

use std::io::Read;
use std::fs::File;
use std::path::PathBuf;

use ureq::get;
use md5::compute;

use crate::config::ProgramConfig;

pub fn get_own_md5(exe_pathbuf: PathBuf) -> Option<String> {
    let mut file = match File::open(exe_pathbuf) {
        Ok(file) => file,
        Err(_) => { return None; }
    };

    let mut buffer = Vec::new();

    if let Err(_) = file.read_to_end(&mut buffer) {
        return None;
    }

    return Some(format!("{:x}", compute(&buffer)));
}

pub fn get_latest_md5() -> Option<String> {
    let response = match get(ProgramConfig::Update::HashURL).call() {
        Ok(response) => response,
        Err(_) => { return None; }
    };

    return match response.into_string() {
        Ok(text) => Some(text.trim().to_string()),
        Err(_) => None
    };
}

pub fn update_exists() -> bool {
    match get(ProgramConfig::Update::URL).call() {
        Ok(response) => { return response.status() == 200; },
        Err(_) => { return false; }
    }
}