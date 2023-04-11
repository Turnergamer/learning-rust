use std::time::{Duration, Instant};

fn main() {
    let start = Instant::now();

    //your code
    let mut i = 0;
    loop {
        i += 1;
        if i == 1000000000{
            println!("Reached 1Billion");
            
            break;
        }


    }
    let elapsed = start.elapsed();
    println!("Time taken: {:?}", elapsed);

}