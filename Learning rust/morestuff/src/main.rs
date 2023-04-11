#[macro_export]
macro_rules! throw {
    ($error:expr, $line:expr) => {
        println!("Error: {} at line {}", $error, $line);
        std::process::exit(1);
    };
    ($error:expr) => {
        println!("Error: {}", $error);
        std::process::exit(1);
    }
}
fn main(){
    println!("HELELEAWIUYEGBAWUYHEBB");
    throw!("");
}