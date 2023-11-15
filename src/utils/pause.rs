use colored::Colorize;
use std::io::{self, Read};

pub fn pause() {
    println!("{} Press enter to exit...", "[ EXIT ]".bold().yellow());
    io::stdin().read(&mut [0]).ok();
}