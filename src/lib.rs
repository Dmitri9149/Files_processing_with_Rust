extern crate csv;
use std::io;
use std::process;
use std::error::Error;


// determened in crate ./bin/csv.rs
// read from stdin like 
//$ ./target/debug/csv < uspop.csv
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
fn run_file() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}