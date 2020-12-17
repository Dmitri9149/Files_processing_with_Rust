// basing on the acticle about csv crate 
// https://docs.rs/csv/1.0.0/csv/tutorial/index.html  
//////////////////////////////////////////////////////

//tutorial-setup-01.rs
// This makes the csv crate accessible to your program.
extern crate csv;

// Import the standard library's I/O module so we can read from stdin.
use std::io;
use std::process;
use std::error::Error;

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    // Create a CSV parser that reads data from stdin.
    let mut rdr = csv::Reader::from_reader(io::stdin());
    // Loop over each record.
    for result in rdr.records() {
// exam. results ; OK -> Ok() ; problems -> convert error to Box<error> 
// use ? syntactic sugare
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}