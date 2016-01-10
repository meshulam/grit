extern crate sha1;
extern crate flate2;

pub mod cache;
pub mod sha;
pub mod hex;

use hex::ToHex;
use std::fs;
use std::io;
use std::io::{Error, ErrorKind};
use std::path::{PathBuf, Path};


// Where should we look for the object database?
pub static DEFAULT_DB_PATH: &'static str = ".grit/objects";


pub fn init_db() -> () {
    fs::create_dir_all(DEFAULT_DB_PATH).unwrap_or_else(|e| match e.kind() {
        ErrorKind::PermissionDenied => panic!("Can't init! Permission denied"),
        _ => (),
    });

    let mut path = PathBuf::from(DEFAULT_DB_PATH);

    for ind in 0..256 as u32 {                  // Iterate as u32s to avoid compiler
        let byte_str = [ind as u8].to_hex();    // thinking we're gonna overflow
        path.push(byte_str);
        fs::create_dir(&path).unwrap_or(());
        path.pop();
    }
    println!("Initialized grit db at {}", DEFAULT_DB_PATH);
}



pub fn find_db_path() -> io::Result<PathBuf> {
    let path = try!(fs::canonicalize("."));
    find_db_in(path)
}

fn find_db_in(mut path: PathBuf) -> io::Result<PathBuf> {
    path.push(".grit");
    if is_valid_db(&path) {
        return Ok(path);
    }
    path.pop();
    match path.pop() {
        true => find_db_in(path),
        false => Err(Error::new(ErrorKind::Other, "No grit DB found!"))
    }
}

fn is_valid_db(path: &Path) -> bool {
    match fs::metadata(path) {
        Err(_) => false,
        Ok(md) => md.is_dir()
    }
}

