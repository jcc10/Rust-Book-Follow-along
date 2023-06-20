#![allow(dead_code)]
use std::io;
use std::collections::HashMap;

// TODO: Split this into different modules/files.
mod command_parser;
#[allow(unused_imports)]
use crate::command_parser::{Command, parse_command, help_all, help};
mod adders;
use crate::adders::{AdderMethod, adder_wrapper};


fn main() {
    let mut departments = HashMap::new();
    loop {
        println!("Enter a command below:");

        let mut raw_array = String::new();

        let cmd = match io::stdin().read_line(&mut raw_array) {
            Ok(_) => {
                parse_command(&raw_array)
            }
            Err(_) => {
                println!("Failed to read line.");
                continue;
            }
        };

        match &cmd {
            Command::Unknown(cmd) => {
                println!("Unknown Command '{}'", cmd);
                println!("Enter 'help' for all commands TODO");
            }
            Command::Add{user, dep} => {
                println!("add | user: {}, dep: {}", user, dep);
                adder_wrapper(
                    &mut departments,
                    user.to_string(),
                    dep.to_string(),
                    AdderMethod::GetMut,
                );
            }
            Command::Search(dep) => {
                println!("search | dep: {}", dep);
            }
            Command::Dump() => {
                println!("List all deps");
                list_all_departments(&departments)
            }
            Command::Warn{command, reason} => {
                println!("warning on {} because of {}", command, reason);
            }
            Command::Exit() => {
                println!("Goodbye!");
                break;
            }
        }
    }
}


fn list_all_departments(hm: &HashMap<String, Vec<String>>){
    //TODO: Sort the items.
    let items: Vec<&String> = hm.keys().collect();
    for key in items {
        match hm.get(key) {
            Some(vec) => {
                println!("| {}", key);
                for person in vec {
                    println!("|-- {}", person);
                }
            }
            None => continue,
        };
    }
}