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

    let full_commands = json.as_object().unwrap().get(&command_to_execute);

    match full_commands {
        Some(full_commands) => {
            for command in full_commands.as_array().unwrap() {
                let command = command.as_str().unwrap();

                let full_command = "source venv/bin/activate; ".to_string() + command;

                let output = Command::new("sh")
                    .arg("-c")
                    .arg(full_command)
                    .output()
                    .expect("failed to execute process");

                println!("{}", String::from_utf8_lossy(&output.stdout));
            }
        }
        None => {
            let command = command_to_execute.as_str();

            let output = Command::new("sh")
                .arg("-c")
                .arg(command)
                .output()
                .expect("failed to execute process");

            println!("{}", String::from_utf8_lossy(&output.stdout));
        }
    }
}
