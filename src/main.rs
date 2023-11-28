mod utils;
mod cheat;
mod ui;

use std::thread::{self, sleep};
use colored::{Colorize, control::set_virtual_terminal};

use crate::utils::input::input;
use crate::utils::open::open_url;
use crate::utils::process_manager::attach_process_manager;
use crate::cheat::classes::offsets::update_offsets;
use crate::cheat::classes::game::init_game_address;
use crate::ui::main::init_gui;
use crate::utils::pause::pause;
use crate::utils::config::{setup_config, update_configs, ProgramConfig};
use crate::utils::updater::{get_own_md5, get_latest_md5, update_exists};

fn main() {
    set_virtual_terminal(true).unwrap();
    println!("{} {} {} | {}", "[ INFO ]".bold().cyan(), ProgramConfig::Package::Name.bold(), format!("v{}", ProgramConfig::Package::Version).bold(), ProgramConfig::Package::Authors.replace(":", " & ").bold());

    if !cfg!(debug_assertions) && ProgramConfig::Update::Enabled && update_exists() {
        let own_md5 = get_own_md5();
        let latest_md5 = get_latest_md5();

        if own_md5.is_some() && latest_md5.is_some() {
            if own_md5.unwrap() != latest_md5.unwrap() {
                let update_confirmation = input(format!("{} Software is outdated, would you like to update? (y/n):", "[ INFO ]".bold().yellow()));

                if update_confirmation.to_lowercase() == "y" {
                    open_url(ProgramConfig::Update::URL);
                    return;
                }
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
        Some(string) => {
            println!("{} Failed to set-up config {}", "[ FAIL ]".bold().red(), format!("({})", string).bold());
            return pause();
        }
    };

    match attach_process_manager() {
        Some(_) => {
            let mut failed_attempts: u32 = 0;
            println!("{} Waiting for {}...", "[ INFO ]".bold().yellow(), ProgramConfig::TargetProcess::Executable.bold());
            
            loop {
                // Attach
                match attach_process_manager() {
                    None => break,
                    Some(error) => {
                        if error != "ProcessId" {
                            failed_attempts += 1;
                        }

                        // Check
                        if failed_attempts >= ProgramConfig::TargetProcess::MaxAttempts {
                            println!("{} Failed to attach {} {}", "[ FAIL ]".bold().red(), ProgramConfig::TargetProcess::Executable.bold(), format!("({})", error).bold());
                            return pause();
                        }
                    }
                }
                
                // Delay
                sleep(ProgramConfig::ThreadDelays::AttachTargetProcess);
            }
        },
        None => {}
    }

    match update_offsets() {
        Some(_) => {
            let mut failed_attempts: u32 = 0;

            loop {
                // Update
                match update_offsets() {
                    None => break,
                    Some(error) => {
                        failed_attempts += 1;

                        // Check
                        if failed_attempts >= ProgramConfig::TargetProcess::UpdateOffsetsMaxAttempts {
                            println!("{} Failed to update offsets {}", "[ FAIL ]".bold().red(), format!("({})", error).bold());
                            return pause();
                        }
                    }
                }

                // Delay
                sleep(ProgramConfig::ThreadDelays::UpdateOffsets);
            }
        },
        None => {}
    }

    match init_game_address() {
        false => {
            let mut failed_attempts: u32 = 0;

            loop {
                // Init
                match init_game_address() {
                    true => break,
                    false => {
                        failed_attempts += 1;

                        // Check
                        if failed_attempts >= ProgramConfig::TargetProcess::InitAddressesMaxAttempts {
                            println!("{} Failed to initialize addresses", "[ FAIL ]".bold().red());
                            return pause();
                        }
                    }
                }

                // Delay
                sleep(ProgramConfig::ThreadDelays::InitAddresses);
            }
        },
        true => {}
    }

    init_gui();
    pause();
}