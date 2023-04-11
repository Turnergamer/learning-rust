fn main() {
    let num: u32 = 3; 
    let result = factorial(num);
    println!("Factorial of {} is {}", num, result);
}

fn factorial(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}