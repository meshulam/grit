/*
 * Accessors for the object cache.
 */

use {DEFAULT_DB_PATH, INDEX_FILE};

use sha::ShaHash;
use std::fs::File;
use std::io;
use std::io::{Read, Write, BufReader};
use std::fs;
use std::path::PathBuf;
use sha1::Sha1;
use flate2::Compression;
use flate2::write::DeflateEncoder;
use flate2::read::DeflateDecoder;


enum ObjectType {
    Commit,
    Tree,
    Blob,
    Tag,
    Unknown
}

impl ObjectType {
    fn key(&self) -> &str {
        match *self {
            ObjectType::Commit => "commit",
            ObjectType::Tree => "tree",
            ObjectType::Blob => "blob",
            ObjectType::Tag => "tag",
            ObjectType::Unknown => "unknown"
        }
    }

    fn parse(key: &str) -> ObjectType {
        match key {
            "commit" => ObjectType::Commit,
            "tree" => ObjectType::Tree,
            "blob" => ObjectType::Blob,
            "tag" => ObjectType::Tag,
            _ => ObjectType::Unknown
        }
    }
}


struct ObjectWriter {
    hasher: Sha1,
    deflator: DeflateEncoder<File>,
    temp_path: PathBuf
}

impl ObjectWriter {
    fn new() -> ObjectWriter {
        let mut path = PathBuf::from(DEFAULT_DB_PATH);
        path.push("tempobj");   // TODO: un-hardcode this
        let tempfile = File::create(&path).unwrap();

        ObjectWriter {
            hasher: Sha1::new(),
            deflator: DeflateEncoder::new(tempfile, Compression::Default),
            temp_path: path
        }
    }

    fn finish(self) -> io::Result<ShaHash> {
        try!(self.deflator.finish());
        let mut sha = ShaHash::new();
        self.hasher.output(sha.bytes_mut());     // Dump the sha hash into our struct

        try!(fs::rename(&self.temp_path, &sha.path(DEFAULT_DB_PATH)));
        let ret: io::Result<ShaHash> = Ok(sha);
        ret
    }
}

impl Write for ObjectWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.deflator.write(buf).map(|wrote| {
            self.hasher.update(buf);
            wrote
        })
    }

    fn flush(&mut self) -> io::Result<()> {
        self.deflator.flush()
    }
}

const BUF_SIZE: usize = 1024;

pub fn hash_object(filename: &String) -> io::Result<String> {
    let mut file = try!(File::open(filename));
    let md = try!(file.metadata());

    let mut writer = ObjectWriter::new();
    let mut buf = [0u8; BUF_SIZE];

    try!(write!(&mut writer, "{} {}\0", ObjectType::Blob.key(), md.len()));

    while let Ok(size) = file.read(&mut buf) {
        if size == 0 { break; }
        let read_bytes = &buf[..size];
        try!(writer.write(read_bytes));
    }

    writer.finish().map(|sha| sha.to_hex())
}

pub fn cat_file(hash: String) -> io::Result<BufReader<DeflateDecoder<File>>> {
    assert!(hash.len() >= 6, 
            "Must provide at least first six characters of obj hash");
    let mut path = PathBuf::from(DEFAULT_DB_PATH);

    let (dir, filename) = hash.split_at(2);
    path.push(dir);
    path.push(filename);

    let decoder = DeflateDecoder::new(try!(File::open(path)));
    Ok(BufReader::new(decoder))
}

pub fn read_cache() {
    match File::open(INDEX_FILE).and_then(|f| f.metadata()) {
        Ok(md) =>
            println!("Opened index file {} with length {}", INDEX_FILE, md.len()),
        Err(e) => println!("Error! {}", e)
    }
}
