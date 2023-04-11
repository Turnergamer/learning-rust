rust
Copy code

use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

fn main() {
    // Get the user's home directory
    let mut path = match dirs::home_dir() {
        Some(path) => path,
        None => panic!("Failed to get home directory."),
    };

    // Add the file name to the path
    path.push("data.txt");

    // Open a file for writing
    let mut file = match File::create(&path) {
        Ok(file) => file,
        Err(e) => panic!("Failed to create file: {}", e),
    };

    // Write some data to the file
    let data = "Hello, world!";
    match file.write_all(data.as_bytes()) {
        Ok(_) => println!("Data saved to file: {:?}", path),
        Err(e) => panic!("Failed to write data to file: {}", e),
    }
}