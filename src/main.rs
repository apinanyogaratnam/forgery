use std::env;
use std::process::Command;

use forgery::docs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        // print all the commands available
        println!("{}", docs::USAGE);
        // println!("{}", docs::COMMANDS);
        return;
    }

    let json = forgery::parse_forge_commands("forgefile.json");

    let command_to_execute = args[1..].join(" ");

    println!("command to execute: {}", command_to_execute);

    let full_commands = json.as_object().unwrap().get(&command_to_execute);

    match full_commands {
        Some(full_commands) => {
            for command in full_commands.as_array().unwrap() {
                let command = command.as_str().unwrap();

                Command::new(command)
                    .spawn()
                    .unwrap();
            }
        }
        None => {
            println!("{}", "command not found");
        }
    }
}
