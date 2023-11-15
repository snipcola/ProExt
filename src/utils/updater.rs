use std::{io::Read, env, fs::File, process::Command};
use chttp::{get, http::StatusCode};
use md5::compute;

use crate::utils::config::UPDATE_URL;
use crate::utils::config::UPDATE_HASH_URL;

pub fn get_own_md5() -> Option<String> {
    let exe_path = match env::current_exe() {
        Ok(exe) => exe,
        _ => { return None; }
    };

    let mut file = match File::open(exe_path) {
        Ok(file) => file,
        _ => { return None; }
    };

    let mut buffer = Vec::new();

    if let Err(_) = file.read_to_end(&mut buffer) {
        return None;
    }

    return Some(format!("{:x}", compute(&buffer)));
}

pub fn get_latest_md5() -> Option<String> {
    let mut response = match get(UPDATE_HASH_URL.clone()) {
        Ok(response) => response,
        _ => { return None; }
    };

    let mut text = "".to_string();
    
    if let Err(_) = response.body_mut().read_to_string(&mut text) {
        return None;
    }

    return Some(text.trim().to_string());
}

pub fn update_exists() -> bool {
    match chttp::get(UPDATE_URL.clone()) {
        Ok(response) => { return response.status() == StatusCode::OK || response.status() == StatusCode::FOUND; },
        Err(_) => { return false; }
    }
}

pub fn open_update_url() {
    Command::new("cmd.exe")
        .args(["/C", "start", &UPDATE_URL.clone()])
        .spawn()
        .ok();
}