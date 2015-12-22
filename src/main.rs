use std::env;

mod lib;

fn print_help() {
    println!("grit - a git-like file database in rust");
    println!("  'grit init' -- initialize the object db");
    println!("  'grit help' -- print this message");
}

fn main() {
    let cmd: String = env::args().nth(1).unwrap_or("help".to_string());

    if cmd == "init".to_string() {
        lib::init_db();
    } else {
        print_help();
    }
}

