use std::fs::{self, File, OpenOptions};
use std::io;
use std::path::Path;

fn create_file<P: AsRef<Path>>(path: P) -> io::Result<File> {
    let path = path.as_ref();

    if let Some(parent_dir) = path.parent() {
        fs::create_dir_all(parent_dir)?;
    }

    OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)
}

fn main() {
    let mut args = std::env::args();
    args.next();

    for arg in args {
        match create_file(&arg) {
            Ok(_) => {
                println!("Created {}", &arg);
            },
            Err(err) => {
                eprintln!("Error creating file {}: {}", &arg, err);
            }
        }
    }
}