pub mod internal;
extern crate anyhow;
extern crate colored;

use crate::internal::cli::{parse_command, Command};
use colored::Colorize;
use internal::{cli, database::Database};

fn main() {
    println!("\n{}\n", "Welcome to KVDB".bold().bright_cyan());

    let mut database = Database::new("kv.db").expect("Failed to create database");

    cli::show_commands();

    loop {
        let raw = cli::prompt("", "Command is required!");

        let args: Vec<&str> = raw.split(" ").filter(|f| !f.trim().is_empty()).collect();

        match parse_command(&args[0].to_string()) {
            Command::EXIT => {
                println!("Exiting...");
                break;
            }
            Command::SHOW => {
                let entries = database.show();
                println!("{}", entries.cyan());
            }
            Command::SET => {
                let key = cli::safe_get(&args, 2, "Key");

                let value = cli::safe_get(&args, 3, "Value");

                database.set(key.as_str(), value.as_str());
            }
            Command::GET => {
                let key = cli::safe_get(&args, 2, "Key");

                if let Some(value) = database.get(key.as_str()) {
                    println!("\n{}", value)
                } else {
                    println!("No entry exists for the key: {}", key.bright_red())
                }
            }
            Command::DEL => {
                let key = cli::safe_get(&args, 2, "Key");

                match database.del(key.as_str()) {
                    Some(_) => (),
                    None => println!("No entry exists for the key: {}", key.bright_red()),
                }
            }
        };
    }
}
