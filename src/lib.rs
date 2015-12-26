extern crate sha1;
extern crate flate2;
extern crate rustc_serialize;

pub mod cache;
pub mod sha;

use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;
//use sha1;


// Where should we look for the object database?
pub static DEFAULT_DB_PATH: &'static str = ".grit/objects";
pub static INDEX_FILE: &'static str = ".grit/index";


pub fn init_db() -> () {
    //let path = Path::new(DEFAULT_DB_PATH);
    //
    fs::create_dir_all(DEFAULT_DB_PATH).unwrap_or_else(|e| match e.kind() {
        ErrorKind::PermissionDenied => panic!("Can't init! Permission denied"),
        _ => (),
    });

    let mut path = PathBuf::from(DEFAULT_DB_PATH);

    for ind in 0..256 as u32 {          // Iterate as u32s to avoid compiler
        path.push(to_hex(ind as u8));   // thinking we're gonna overflow
        fs::create_dir(&path).unwrap_or(());
        path.pop();
    }
    println!("Initialized grit db at {}", DEFAULT_DB_PATH);
}


// Inlining to_hex from libserialize
const CHARS: &'static [u8] = b"0123456789abcdef";

pub fn to_hex(byte: u8) -> String {
    let v = vec![CHARS[(byte >> 4)  as usize],
                 CHARS[(byte & 0xf) as usize]];
    unsafe {
        String::from_utf8_unchecked(v)
    }
}
