use dont_disappear::any_key_to_continue;
use colored::Colorize;

pub fn pause() {
    any_key_to_continue::custom_msg(format!("{} Press any key to quit...", "[ EXIT ]".bold().yellow()).as_str());
}