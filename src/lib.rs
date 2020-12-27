extern crate csv;
use std::io;
#[allow(unused_imports)] use std::process;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;


// determened in crate ./bin/csv.rs
// read from stdin like 
//$ ./target/debug/csv < uspop.csv
#[allow(dead_code)]
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

// determened in crate ./bin/csv.rs
// read from file like
#[allow(dead_code)] 
fn run_file() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let mut rdr = csv::Reader::from_file(file_path);
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

// Returns the first positional argument sent to this process. If there are no
// positional arguments, then this returns an error.
#[allow(dead_code)]
fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}