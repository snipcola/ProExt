#![cfg_attr(all(target_os = "windows", not(debug_assertions)), windows_subsystem = "windows")]

mod config;
mod utils;
mod cheat;
mod ui;

use std::env;
use std::thread::{self, sleep};

use crate::config::ProgramConfig;

use crate::utils::open::open_url;
use crate::utils::cheat::process::{attach_process, get_process_amount};
use crate::utils::cheat::config::{setup_config, update_configs};
use crate::utils::cheat::updater::update_available;
use crate::utils::messagebox::{create_messagebox, create_dialog, MessageBoxStyle, MessageBoxButtons, MessageBoxResult};

use crate::cheat::classes::game::init_game_address;
use crate::ui::main::init_gui;

fn main() {
    let exe_pathbuf = match env::current_exe() {
        Ok(exe) => exe,
        Err(_) => return
    };

    if get_process_amount(ProgramConfig::Package::Executable) > 1 || get_process_amount(&exe_pathbuf.file_name().unwrap().to_string_lossy()) > 1 {
        return create_messagebox(MessageBoxStyle::Error, "Already Running", &format!("{} is already running.", ProgramConfig::Package::Name));
    }
    
    if !cfg!(debug_assertions) && ProgramConfig::Update::Enabled {
        match update_available() {
            Some(new_version) => match create_dialog(MessageBoxStyle::Info, MessageBoxButtons::YesNo, &format!("{} Available", new_version), &format!("{} {} is outdated, would you like to update?", ProgramConfig::Package::Name, ProgramConfig::Package::Version)) {
                MessageBoxResult::Yes => return open_url(ProgramConfig::Update::URL),
                _ => {}
            },
            None => {}
        }
    }

    match setup_config() {
        None => {
            thread::spawn(|| {
                loop {
                    update_configs();
                    sleep(ProgramConfig::ThreadDelays::UpdateConfigs);
                }
            });
        },
        Some(error) => {
            return create_messagebox(MessageBoxStyle::Error, "Error", &format!("Failed to set-up config ({}).", error));
        }
    }

    match attach_process() {
        Some(_) => {
            match create_dialog(MessageBoxStyle::Warning, MessageBoxButtons::OkCancel, "Not Found", &format!("Couldn't find {}, wait for it to open?", ProgramConfig::TargetProcess::Executable)) {
                MessageBoxResult::Cancel => return,
                _ => {}
            };
        
            let mut failed_attempts: u32 = 0;
            
            loop {
                match attach_process() {
                    None => break,
                    Some(error) => {
                        if error != "ProcessId" {
                            failed_attempts += 1;
                        }

                        if failed_attempts >= ProgramConfig::TargetProcess::MaxAttempts {
                            return create_messagebox(MessageBoxStyle::Error, "Error", &format!("Failed to attach {} ({}).", ProgramConfig::TargetProcess::Executable, error));
                        }
                    }
                }
                
                sleep(ProgramConfig::CheckDelays::AttachProcess);
            }
        },
        None => {}
    }

    match init_game_address() {
        false => {
            let mut failed_attempts: u32 = 0;

            loop {
                match init_game_address() {
                    true => break,
                    false => {
                        failed_attempts += 1;

                        if failed_attempts >= ProgramConfig::TargetProcess::InitAddressesMaxAttempts {
                            return create_messagebox(MessageBoxStyle::Error, "Error", "Failed to initialize addresses.");
                        }
                    }
                }

                sleep(ProgramConfig::CheckDelays::InitAddresses);
            }
        },
        true => {}
    }

    init_gui();
}