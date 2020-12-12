There are to be several bin crates to work with different files formats : zip, csv, big files. 
It is to be a file processing pipeline. At the later stages the shift is to be from processing a file to processing a data in a file. 
The final structure on the pipelines is still not clear, has to be arised within the project development. 


Now the project contains: 

main.rs -> get the zip file from web
csv.rs -> bin crate to work with csv files using Rust tools (read, write etc..)
zip.rs -> bin crate for working with zip files (zip, unzip, read, write)
data/csv : folder for csv files
lib.rs : extract useful functions from bin crates for later use

------------------------------------------------------
The project is intended for making pipelines in files processing + 
'initial' pipelines in data processing (mainly text data).

The processes files are to organized in simple 'file' system which 
reflects the stages of files processing. Something like : get .zip file from web and place it to folder 'init_zip' and open it and place it to folder 'opened_tgz' by some open tools in Rust.

The tools are organized as bin crates -> files in src/bin/*.rs . The working cycle of the project is : Find -> Learn -> CreateCrateTool -> ExtractInLibrary -> UseInNewCrates -> NewProjects

It means : find the tools in Rust for getting files, working with csv, zip, ... other files and data , learn the tools. Every 'tool' is a bin crate. Extract the needed functions to library -> lib.rs and later use the functions in new projects. The initial crates are used as examples of the tools.  

So, there are files in 'Url'locations (local 'file system') and bin crates with tools which process and 'move' the files from location to location.

The project is in development.
