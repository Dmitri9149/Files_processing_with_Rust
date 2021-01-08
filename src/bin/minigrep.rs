use std::env;
use std::fs;


// read args from stdin
fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file  {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong with file reading");

    println!("The file (text) content:\n{}", contents);

}