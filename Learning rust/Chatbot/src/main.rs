#![allow(snake_case)]
use std::io;
use std::time::{Duration, Instant};

fn main() {
    println!("What is your name?");
    
    let mut name = String::new();
    let bannednames = vec!["roni", "pelajahacks", "pel"];
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    let status;
    if name.trim().to_lowercase() == "roni"{
        status = "Gay";
    }else {
        status = "Straight";
    }
    println!("Hello, {}!", name.trim());
    println!("Found INFO \nSTATUS | {} \nNAME | {}", status, name);




    
    let mut i = 0;
    loop {
        i += 1;
        println!("{}", i);
        if i == 2000000{
            println!("Reached 2MIL");
            break;
        }

    }
    
    
}
