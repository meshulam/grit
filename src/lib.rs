extern crate sha1;
extern crate flate2;

pub mod cache;
pub mod sha;
pub mod hex;
pub mod util;

use GRIT_DIR_NAME;

use hex::ToHex;
use std::fs;
use std::io;
use std::io::{Error, ErrorKind};
use std::path::{PathBuf, Path};



pub fn init_db() -> () {
    let mut path = PathBuf::from(GRIT_DIR_NAME);
    path.push("objects");

    fs::create_dir_all(path).unwrap_or_else(|e| match e.kind() {
        ErrorKind::PermissionDenied => panic!("Can't init! Permission denied"),
        _ => (),
    });

    for ind in 0..256 as u32 {                  // Iterate as u32s to avoid compiler
        let byte_str = [ind as u8].to_hex();    // thinking we're gonna overflow
        path.push(byte_str);
        fs::create_dir(&path).unwrap_or(());
        path.pop();
    }
    println!("Initialized grit db at {}", GRIT_DIR_NAME);
}

