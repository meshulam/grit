extern crate libgrit;

use std::env;
use std::io::BufRead;

fn print_help() {
    println!("grit - a git-like file database in rust");
    println!("  grit init -- initialize the object db");
    println!("  grit hash-object");
    println!("  grit cat-file");
    println!("  grit help -- print this message");
}

fn main() {
    let cmd: String = env::args().nth(1).unwrap_or("help".to_string());
    let cmd_args: Vec<String> = env::args().skip(1).collect();

    if cmd == "init" {
        libgrit::init_db();
    } else if cmd == "hash-object" {
        hash_object(cmd_args);
    } else if cmd == "cat-file" {
        cat_file(cmd_args);
    } else {
        print_help();
    }
}

fn cat_file(mut args: Vec<String>) {
    let file = args.pop().unwrap();
    match libgrit::cache::cat_file(file) {
        Ok(stream) => for line in stream.lines() {
            println!("{}", line.unwrap());
        },
        Err(e) => println!("Error cat-ing: {}", e)
    }
}

fn hash_object(mut args: Vec<String>) {
    let file = args.pop().unwrap();

    match libgrit::cache::hash_object(&file) {
        Ok(hash) => println!("Stored: {}", hash),
        Err(e) => println!("Error hashing: {}", e)
    }
}
