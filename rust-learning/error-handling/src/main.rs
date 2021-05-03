use std::{fs, io};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let file_name = "hello.txt";

    let username =
        read_username_from_file(file_name)?;

    println!("Hello, {}", username);

    Ok(())
}

fn read_username_from_file(name: &str) -> Result<String, io::Error> {
    fs::read_to_string(name)
}
