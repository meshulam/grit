
// Where should we look for the object database?
pub static GRIT_DIR_NAME: &'static str = ".grit";


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
