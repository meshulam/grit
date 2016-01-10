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
    let all_args: env::Args = env::args();

    let cmd: &str = cmd_args.next().map_or("help", |arg| arg.as_slice());
    let cmd_args: Vec<String> = all_args.collect();

    if cmd == "init" {
        libgrit::init_db();
    } else if cmd == "status" {
        status(cmd_args);
    } else if cmd == "commit" {
        commit(cmd_args);
    } else if cmd == "hash-object" {
        hash_object(cmd_args);
    } else if cmd == "cat-file" {
        cat_file(cmd_args);
    } else {
        print_help();
    }
}

fn commit(mut args: Vec<String>) {
    println!("stub commit");
}

fn status(mut args: Vec<String>) {
    match libgrit::find_db_path() {
        Ok(path) => println!("Found grit DB at {}", path.display()),
        Err(e) => println!("Can't find grit DB: {}", e)
    }
}

fn cat_file(mut args: Vec<String>) {
    let file = args.last().unwrap();
    match libgrit::cache::cat_file(file) {
        Ok(stream) => for line in stream.lines() {
            println!("{}", line.unwrap());
        },
        Err(e) => println!("Error cat-ing: {}", e)
    }
}

fn hash_object(mut args: Vec<String>) {
    let file = args.last().unwrap();

    match libgrit::cache::hash_object(file) {
        Ok(hash) => println!("Stored: {}", hash),
        Err(e) => println!("Error hashing: {}", e)
    }
}
