#![allow(dead_code)]
use std::io;
use std::collections::HashMap;

mod command_parser;
#[allow(unused_imports)]
use crate::command_parser::{Command, parse_command, help_all, help};
mod adders;
use crate::adders::{AdderMethod, adder_wrapper};
mod listers;
use crate::listers::{list_all_departments, list_department};

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

        match cmd {
            Command::Exit() => {
                println!("Goodbye!");
                break;
            }
            other => {
                command_handeler(&mut departments, other);
            }
        }

    }
}

fn command_handeler(
    departments: &mut HashMap<String, Vec<String>>,
    cmd: Command
){
    match &cmd {
        Command::Unknown(cmd) => {
            println!("Unknown Command '{}'", cmd);
            println!("Enter 'help' for all commands TODO");
        }
        Command::Add{user, dep} => {
            println!("add | user: {}, dep: {}", user, dep);
            adder_wrapper(
                departments,
                user.to_string(),
                dep.to_string(),
                AdderMethod::GetMut,
            );
        }
        Command::Search(dep) => {
            println!("search | dep: {}", dep);
            list_department(departments, dep.to_string());
        }
        Command::Dump() => {
            println!("List all deps");
            list_all_departments(&departments)
        }
        Command::Warn{command, reason} => {
            println!("warning on {} because of {}", command, reason);
        }
        Command::Exit() => {
            println!("Unreachable line?")
        }
    }
}