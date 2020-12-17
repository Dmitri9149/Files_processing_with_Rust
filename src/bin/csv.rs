// basing on the acticle about csv crate 
// https://docs.rs/csv/1.0.0/csv/tutorial/index.html  
//////////////////////////////////////////////////////

//tutorial-setup-01.rs
// This makes the csv crate accessible to your program.
extern crate csv;

// Import the standard library's I/O module so we can read from stdin.
use std::io;
use std::process;

// The `main` function is where your program starts executing.
fn main() {
    // Create a CSV parser that reads data from stdin.
    let mut rdr = csv::Reader::from_reader(io::stdin());
    // Loop over each record.
    for result in rdr.records() {
// exam. results ; OK -> Ok() ; problems -> print error message 
        match result {
            Ok(record) => println!("{:?}", record),
            Err(err) => { 
                println!("error reading file from <stdin>{}", err);
            process::exit(1);
            }

        }
    }
}