There are to be several bin crates to work with different files formats : zip, csv, big files. 
It is to be a file processing pipeline. At the later stages the shift is to be from processing a file to processing a data in a file. 

Now the project contains: 
 
- src/main.rs -> get the zip file from web  

- csv bin crate: src/bin/csv.rs -> bin crate to work with csv files using Rust tools (read, write etc..)   

- zip bin crate: src/bin/zip.rs -> bin crate for working with zip files (zip, unzip, read, write)  

- minigrep bin crate: src/bin/minigrep.rs -> some analog of 'grep': https://doc.rust-lang.org/book/ch12-00-an-io-project.html processing of text file content.

- data/csv : folder for csv files  

- data/text : folder for text files

- lib.rs : extract useful functions from bin crates for later use


