// basing on the acticle about csv crate 
// https://docs.rs/csv/1.0.0/csv/tutorial/index.html  
//////////////////////////////////////////////////////

extern crate csv;
use std::io;
use std::process;
use std::error::Error;

// work with file 
use std::ffi::OsString;
use std::fs::File;

fn main() {
    if let Err(err) = run_stdin() {
        println!("{}", err);
        process::exit(1);
    }
}

pub fn run_stdin() -> Result<(), Box<dyn Error>> {
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