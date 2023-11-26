use std::process::Command;

pub fn open_url(url: &str) {
    Command::new("cmd.exe")
        .args(["/C", "start", url])
        .spawn()
        .ok();
}