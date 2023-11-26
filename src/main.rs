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

    if !cfg!(debug_assertions) && update_exists() {
        let own_md5 = get_own_md5();
        let latest_md5 = get_latest_md5();

        if own_md5.is_some() && latest_md5.is_some() {
            if own_md5.unwrap() == latest_md5.unwrap() {
                println!("{} Software is up-to-date", "[ OKAY ]".bold().green());
            } else {
                let update_confirmation = input(format!("{} Software is not up-to-date. Would you like to update? (y/n):", "[ INFO ]".bold().yellow()));

                if update_confirmation.to_lowercase() == "y" {
                    open_url(ProgramConfig::Update::URL);
                    return;
                }
            }
        } else {
            println!("{} Couldn't check for updates, continuing anyway", "[ FAIL ]".bold().red());
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
            
            println!("{} Set-up config", "[ OKAY ]".bold().green());
        },
        Some(string) => {
            println!("{} Failed to set-up config {}", "[ FAIL ]".bold().red(), format!("({})", string).bold());
            return pause();
        }
    };

    match attach_process_manager() {
        None =>  println!("{} Attached {} process", "[ OKAY ]".bold().green(), ProgramConfig::TargetProcess::Executable.bold()),
        Some(error) => {
            println!("{} Failed to attach {} process {}", "[ FAIL ]".bold().red(), ProgramConfig::TargetProcess::Executable.bold(), format!("({})", error).bold());
            return pause();
        }
    }

    match update_offsets() {
        None => {
            println!("{} Updated offsets", "[ OKAY ]".bold().green());
        },
        Some(string) => {
            println!("{} Failed to update offsets {}", "[ FAIL ]".bold().red(), format!("({})", string).bold());
            return pause();
        }
    }
    
    if init_game_address() {
        println!("{} Initialized addresses", "[ OKAY ]".bold().green());
    } else {
        println!("{} Failed to initialize addresses", "[ FAIL ]".bold().red());
    }

    init_gui();
    pause();
}