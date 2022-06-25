use std::env;

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

    let full_command = json.as_object().unwrap().get(&command_to_execute);

    match full_command {
        Some(full_command) => {
            println!("full command: {:?}", full_command);
        }
        None => {
            println!("{}", "command not found");
        }
    }
}
