use std::env;

use forgery::docs;

fn main() {
    let args: Vec<String> = env::args().collect();

    forgery::parse_forge_commands("forgefile.json");

    if args.len() == 1 {
        // print all the commands available
        println!("{}", docs::USAGE);
        // println!("{}", docs::COMMANDS);
        return;
    }
}
