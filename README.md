There are to be several bin crates to work with zip (?) files. 
It is to be a file processing pipeline. At the later stages the shift is to be from processing a file to processing a data in a file. 
The final structure on the pipelines is still not clear, has to be aised within the project development. 

Now we have: 

main.rs -> get the zip file from web
read.rs unzip and read the zipped files

Now the project contains: 

csv.rs bin crate : how to work with csv files using Rust tools. 
data/csv : folder for csv files
lib.rs : extract useful functions from bin crates for later use

------------------------------------------------------
The project is intended for making pipelines in files processing + 
'initial' pipelines in data processing (mainly text data).

The processes files are to organized in simple 'file' system which 
reflects the stages of files processing. Something like : get .tgz file from web and place it to folder 'init_tgz' and open it and place it to folder 'opened_tgz' by some open tools in Rust.

The toolds are organized by bin crates -> files in src/bin/*.rs . The working cycle of the project is : Find -> Learn -> CreateCrateTool -> ExtractInLibrary -> UseInNewCrates -> NewProjects

It means : find the tools in Rust for getting files, working with csv, tgz, ... other files and data , learn the tools. Every 'tool' is formed as bin crate. Extract the needed functions to library -> lib.rs and later use the functions in new projects. The initial crates are used as examples.  

Collecting/testing/ercising of Rust tools which may be 
needed for getting (e.g. via web), reding, writing, 
processing of data resources (e.g. files in different formats).

The tools are to be organized as sepatate crates which may act 
on data resources (files, lines) and form consolidated pipelines of  
getting/reading/writing/processing of the resources. 
