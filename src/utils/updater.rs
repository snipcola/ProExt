use std::{io::Read, env, fs::File, process::Command};
use ureq::get;
use md5::compute;

use crate::utils::config::ProgramConfig;

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
    let response = match get(ProgramConfig::Update::HashURL).call() {
        Ok(response) => response,
        _ => { return None; }
    };

    return match response.into_string() {
        Ok(text) => Some(text.trim().to_string()),
        _ => None
    };
}

pub fn update_exists() -> bool {
    match get(ProgramConfig::Update::URL).call() {
        Ok(response) => { return response.status() == 200; },
        Err(_) => { return false; }
    }
}

pub fn open_update_url() {
    Command::new("cmd.exe")
        .args(["/C", "start", ProgramConfig::Update::URL])
        .spawn()
        .ok();
}