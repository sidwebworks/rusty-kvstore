use std::{collections::HashMap, fs, io, path::Path};

use anyhow::Result;
use colored::Colorize;

pub struct Database<'db> {
    filename: &'db str,
    flushed: bool,
    map: HashMap<String, String>,
}

impl<'db> Database<'db> {
    pub fn new(filename: &'db str) -> Result<Self, io::Error> {
        let contents = match Path::new(filename).exists() {
            true => {
                println!(
                    "{}",
                    "Database dump found, restoring from disk...".bright_blue()
                );
                fs::read_to_string(&filename)?
            }
            false => {
                println!("{}", "No existing database dump found".bright_yellow());
                fs::write(&filename, "")?;
                String::new()
            }
        };

        let mut map = HashMap::new();

        for line in contents.lines() {
            let (key, value) = line.split_once(" = ").expect("Database corrupt");
            map.insert(key.to_owned(), value.to_owned());
        }

        Ok(Database {
            map,
            filename,
            flushed: false,
        })
    }

    pub fn flush(mut self) -> Result<bool, io::Error> {
        self.flushed = true;
        flush_impl(&mut self)
    }
}

impl<'db> Database<'db> {
    pub fn set(&mut self, key: &str, value: &str) -> Option<String> {
        self.map.insert(key.to_owned(), value.to_owned())
    }

    pub fn get(&mut self, key: &str) -> Option<&String> {
        self.map.get(key)
    }

    pub fn del(&mut self, key: &str) -> Option<String> {
        self.map.remove(key)
    }
}

impl<'db> Drop for Database<'db> {
    fn drop(&mut self) {
        if !self.flushed {
            let _ = flush_impl(self);
        }
    }
}

fn flush_impl(database: &mut Database) -> Result<bool, std::io::Error> {
    let mut contents = String::new();

    for (key, value) in &database.map {
        contents.push_str(key);
        contents.push_str(" = ");
        contents.push_str(value);
        contents.push('\n');
    }

    fs::write(database.filename, contents)?;

    Ok(true)
}
