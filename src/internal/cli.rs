use std::io::{stdin, stdout, Write};

use colored::Colorize;

pub enum Command {
    SET,
    GET,
    DEL,
    EXIT,
    SHOW
}

pub fn parse_command(command: &String) -> Command {
    match command.to_uppercase().as_str() {
        "SET" => Command::SET,
        "GET" => Command::GET,
        "DEL" => Command::DEL,
        "EXIT" => Command::EXIT,
        "SHOW" => Command::SHOW,
        unknown => {
            println!(
                "Invalid command: {}, exiting...",
                unknown.bright_red().bold()
            );
            std::process::exit(1);
        }
    }
}

pub fn show_commands() {
    println!(
        "\n{}{}",
        "Available commands: ",
        "SET | GET | DEL | SHOW | EXIT".bold().bright_green()
    );
}

pub fn prompt(name: &str, msg: &str) -> String {
    let mut line = String::new();
    print!("{}{}", name.bright_green().bold(), " > ".bright_cyan());

    stdout().flush().unwrap();

    stdin().read_line(&mut line).expect("[ERROR] reading line!");

    if line.trim().is_empty() {
        println!("Invalid input");
        return prompt(name, msg);
    }

    return line.trim().to_string();
}

pub fn safe_get(args: &Vec<&str>, order: usize, name: &str) -> String {
    match args.len() < order {
        true => prompt(
            &name.to_uppercase().as_str(),
            &format!("{} is required", name).as_str(),
        ),
        false => args[order - 1].to_string(),
    }
}
