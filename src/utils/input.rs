use std::io::{stdin, stdout, Write};

pub fn input(question: String) -> String {
    print!("{} ", question);
    stdout().flush().ok();

    let mut input = String::new();

    match stdin().read_line(&mut input) {
        Ok(_) => {
            return input.trim().to_string();
        },
        Err(_) => {
            return "".to_string();
        }
    }
}