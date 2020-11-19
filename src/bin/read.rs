// https://www.reddit.com/r/rust/comments/d5i74n/accessing_zipped_file/
use std::fs::File;
// use std::fs;

fn doit() -> zip::result::ZipResult<()> {
    use std::io::prelude::*;
    
    let mut f = File::open("download.zip")?;                
        let mut zip = zip::ZipArchive::new(f)?;
    for i in 0..zip.len()
    {   
        if i == 0 {
            let mut buffer = String::new();
            zip.by_index(0).unwrap().read_to_string(&mut buffer)
            .expect("Something went wrong reading the file");
            println!("With text:\n{:?}", buffer);
        } 

        let mut file = zip.by_index(i).unwrap();
        println!("Filename: {}", file.name());
        let first_byte = file.bytes().next().unwrap()?;
        println!("{}", first_byte);
    }

    Ok(())
}

fn main() {
    doit();
}