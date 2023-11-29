#![cfg_attr(all(target_os = "windows", not(debug_assertions)), windows_subsystem = "windows")]

mod utils;
mod cheat;
mod ui;

use std::thread::{self, sleep};
use utils::messagebox::{MessageBoxStyle, MessageBoxButtons, MessageBoxResult};

use crate::utils::messagebox::{create_messagebox, create_dialog};
use crate::utils::open::open_url;
use crate::utils::process_manager::{attach_process_manager, get_process_amount};
use crate::cheat::classes::offsets::update_offsets;
use crate::cheat::classes::game::init_game_address;
use crate::ui::main::init_gui;
use crate::utils::config::{setup_config, update_configs, ProgramConfig};
use crate::utils::updater::{get_own_md5, get_latest_md5, update_exists};

fn main() {
    if get_process_amount(ProgramConfig::Package::Executable) > 1 {
        return create_messagebox(MessageBoxStyle::Error, "Already Running", &format!("{} is already running.", ProgramConfig::Package::Name));
    }

    if !cfg!(debug_assertions) && ProgramConfig::Package::AskStart {
        let caption = format!("{} - {}", ProgramConfig::Package::Name, ProgramConfig::Package::Authors.replace(":", ", "));
        let text = format!("Would you like to start {} v{}?", ProgramConfig::Package::Name, ProgramConfig::Package::Version);

        match create_dialog(MessageBoxStyle::Info, MessageBoxButtons::YesNo, &caption, &text) {
            MessageBoxResult::No => return,
            _ => {}
        }
    }
    
    if !cfg!(debug_assertions) && ProgramConfig::Update::Enabled && update_exists() {
        let own_md5 = get_own_md5();
        let latest_md5 = get_latest_md5();

        if own_md5.is_some() && latest_md5.is_some() {
            if own_md5.unwrap() != latest_md5.unwrap() {
                match create_dialog(MessageBoxStyle::Info, MessageBoxButtons::YesNo, "Update Available", &format!("This version of {} is outdated, would you like to update?", ProgramConfig::Package::Name)) {
                    MessageBoxResult::Yes => return open_url(ProgramConfig::Update::URL),
                    _ => {}
                };
            }
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

    match attach_process_manager() {
        Some(_) => {
            match create_dialog(MessageBoxStyle::Warning, MessageBoxButtons::OkCancel, "Not Found", &format!("Couldn't find {}, wait for it to open?", ProgramConfig::TargetProcess::Executable)) {
                MessageBoxResult::Cancel => return,
                _ => {}
            };
        
            let mut failed_attempts: u32 = 0;
            
            loop {
                match attach_process_manager() {
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
                
                sleep(ProgramConfig::ThreadDelays::AttachTargetProcess);
            }
        },
        None => {}
    }

    match update_offsets() {
        Some(_) => {
            let mut failed_attempts: u32 = 0;

            loop {
                match update_offsets() {
                    None => break,
                    Some(error) => {
                        failed_attempts += 1;

                        if failed_attempts >= ProgramConfig::TargetProcess::UpdateOffsetsMaxAttempts {
                            return create_messagebox(MessageBoxStyle::Error, "Error", &format!("Failed to update offsets ({}).", error));
                        }
                    }
                }

                sleep(ProgramConfig::ThreadDelays::UpdateOffsets);
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

                sleep(ProgramConfig::ThreadDelays::InitAddresses);
            }
        },
        true => {}
    }

    init_gui();
}