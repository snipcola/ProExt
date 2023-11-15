use reqwest::{StatusCode, blocking::get};
use std::{io::Read, env, fs::File};
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
    
    if let Err(_) = response.read_to_string(&mut text) {
        return None;
    }

    return Some(text.trim().to_string());
}

pub fn update_exists() -> bool {
    match get(UPDATE_URL.clone()) {
        Ok(response) => { return response.status() == StatusCode::OK; },
        Err(_) => { return false; }
    }
}