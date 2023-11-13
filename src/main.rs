mod utils;
mod cheat;
mod ui;

use std::thread::{self, sleep};
use colored::{Colorize, control::set_virtual_terminal};

use crate::utils::process_manager::{AttachStatus, attach_process_manager};
use crate::cheat::classes::offsets::update_offsets;
use crate::cheat::classes::game::init_game_address;
use crate::ui::main::init_gui;
use crate::utils::pause::pause;
use crate::utils::config::{setup_config, update_configs, PACKAGE_NAME, PACKAGE_VERSION, PACKAGE_AUTHORS, DEBUG, PROCESS_EXECUTABLE, THREAD_DELAYS};

fn main() {
    set_virtual_terminal(true).unwrap();
    println!("{} {} | {} | {}", "[ INFO ]".bold().cyan(), (*PACKAGE_NAME).bold(), (*PACKAGE_AUTHORS).bold(), format!("v{}", (*PACKAGE_VERSION)).bold());

    match setup_config() {
        None => {
            let update_configs_thread = thread::spawn(|| {
                loop {
                    update_configs();
                    sleep(THREAD_DELAYS.update_configs);
                }
            });
            
            if *DEBUG { println!("{} UpdateConfigs Thread ID: {} (delay: {})", "[ INFO ]".blue().bold(), format!("{:?}", update_configs_thread.thread().id()).bold(), format!("{:?}", THREAD_DELAYS.update_configs).bold()); }
            println!("{} Set-up config", "[ OKAY ]".bold().green());
        },
        Some(string) => {
            println!("{} Failed to set-up config {}", "[ FAIL ]".bold().red(), format!("({})", string).bold());
            return pause();
        }
    };

    match attach_process_manager() {
        AttachStatus::Success =>  println!("{} Attached {} process", "[ OKAY ]".bold().green(), PROCESS_EXECUTABLE.bold()),
        status => {
            println!("{} Failed to attach {} process {}", "[ FAIL ]".bold().red(), PROCESS_EXECUTABLE.bold(), format!("({:?})", status).bold());
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