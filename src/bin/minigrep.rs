use std::env


// read args from stdin
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?"}, args);
}