use std::path::Path;
use std::fs;

// Where should we look for the object database?
static DEFAULT_DB_PATH: &'static str = ".grit/objects";

pub fn init_db() -> () {
    //let path = Path::new(DEFAULT_DB_PATH);
    //
    fs::create_dir_all(DEFAULT_DB_PATH);
}

