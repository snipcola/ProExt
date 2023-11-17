mod utils;
mod cheat;
mod ui;

use std::thread::{self, sleep};
use colored::{Colorize, control::set_virtual_terminal};

use crate::utils::input::input;
use crate::utils::process_manager::attach_process_manager;
use crate::cheat::classes::offsets::update_offsets;
use crate::cheat::classes::game::init_game_address;
use crate::ui::main::init_gui;
use crate::utils::pause::pause;
use crate::utils::config::{setup_config, update_configs, PACKAGE_NAME, PACKAGE_VERSION, PACKAGE_AUTHORS, PROCESS_EXECUTABLE, THREAD_DELAYS};
use crate::utils::updater::{get_own_md5, get_latest_md5, update_exists, open_update_url};

fn main() {
    set_virtual_terminal(true).unwrap();
    println!("{} {} | {} | {}", "[ INFO ]".bold().cyan(), (*PACKAGE_NAME).bold(), (*PACKAGE_AUTHORS).bold(), format!("v{}", (*PACKAGE_VERSION)).bold());

    if !cfg!(debug_assertions) && update_exists() {
        let own_md5 = get_own_md5();
        let latest_md5 = get_latest_md5();

        if own_md5.is_some() && latest_md5.is_some() {
            if own_md5.unwrap() == latest_md5.unwrap() {
                println!("{} Software is up-to-date", "[ OKAY ]".bold().green());
            } else {
                let update_confirmation = input(format!("{} Software is not up-to-date. Would you like to update? (y/n):", "[ INFO ]".bold().yellow()));

                if update_confirmation.to_lowercase() == "y" {
                    open_update_url();
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
                    sleep(THREAD_DELAYS.update_configs);
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
        None =>  println!("{} Attached {} process", "[ OKAY ]".bold().green(), PROCESS_EXECUTABLE.bold()),
        Some(error) => {
            println!("{} Failed to attach {} process {}", "[ FAIL ]".bold().red(), PROCESS_EXECUTABLE.bold(), format!("({})", error).bold());
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