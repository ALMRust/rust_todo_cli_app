extern crate clap;

use crate::types::Commands;
use clap::{App, Arg, SubCommand};
use std::process::exit;

pub fn cli_config() -> App<'static, 'static> {
    App::new("Rust Todo App")
        .version("1.0")
        .author("Alessandro Maclaine")
        .about("A cli todo app")
        .subcommand(
            SubCommand::with_name("add")
                .about("Adds a todo item")
                .arg(Arg::with_name("text").required(true)),
        )
        .subcommand(
            SubCommand::with_name("edit")
                .about("Edit a todo item")
                .arg(Arg::with_name("id").required(true))
                .arg(Arg::with_name("text").required(true)),
        )
        .subcommand(
            SubCommand::with_name("complete")
                .about("Marks a todo item complete")
                .arg(Arg::with_name("id").required(true)),
        )
        .subcommand(
            SubCommand::with_name("incomplete")
                .about("Marks a todo item incomplete")
                .arg(Arg::with_name("id").required(true)),
        )
        .subcommand(SubCommand::with_name("list").about("List todos"))
        .subcommand(
            SubCommand::with_name("remove")
                .about("Remove a todo item")
                .arg(Arg::with_name("id").required(true)),
        )
}

pub fn get_command() -> Commands {
    let matches = cli_config().get_matches();
    match matches.subcommand() {
        ("add", Some(sub_m)) => {
            let text = sub_m.value_of("text").unwrap();
            Commands::Add(text.to_string())
        }
        ("edit", Some(sub_m)) => {
            let id = sub_m.value_of("id").unwrap();
            if let Ok(id) = id.parse() {
                let text = sub_m.value_of("text").unwrap();
                Commands::Edit(id, text.to_string())
            } else {
                println!("Must pass a number as id for edit");
                exit(1);
            }
        }
        ("complete", Some(sub_m)) => {
            let id = sub_m.value_of("id").unwrap();
            if let Ok(id) = id.parse() {
                Commands::Complete(id)
            } else {
                println!("Must pass a number as id for complete");
                exit(1);
            }
        }
        ("incomplete", Some(sub_m)) => {
            let id = sub_m.value_of("id").unwrap();
            if let Ok(id) = id.parse() {
                Commands::Incomplete(id)
            } else {
                println!("Must pass a number as id for incomplete");
                exit(1);
            }
        }
        ("remove", Some(sub_m)) => {
            let id = sub_m.value_of("id").unwrap();
            if let Ok(id) = id.parse() {
                Commands::Remove(id)
            } else {
                println!("Must pass a number as id for remove");
                exit(1);
            }
        }
        ("list", Some(_)) => Commands::List,
        _ => {
            println!("Please enter a valid command or -h for help");
            exit(1);
        }
    }
}
