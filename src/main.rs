// from https://users.rust-lang.org/t/download-file-from-web/19863/6
// from https://stackoverflow.com/questions/62851371/rust-download-and-save-zip-file


use error_chain::error_chain;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

extern crate zip;

use std::io::{Seek, Write};
use zip::result::ZipResult;
use zip::write::{FileOptions, ZipWriter};
use zip::read::ZipFile;
use zip::ZipArchive;


error_chain! {
     foreign_links {
         Io(std::io::Error);
         HttpRequest(reqwest::Error);
     }
}

#[tokio::main]
async fn main() -> Result<()> {
    let target = "http://www.manythings.org/anki/fra-eng.zip";
    let response = reqwest::get(target).await?;

    let path = Path::new("./download.zip");

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}", why),
        Ok(file) => file,
    };
    let content =  response.bytes().await?;
    let slice: &[u8] = content.as_ref();
    file.write_all(slice)?;

    let mut file_a = File::open("download.zip").expect("Couldn't open file");
    let files = browse_zip_archive(&mut file_a, |f| {
        Ok(format!("{}: {} -> {}", f.name(), f.size(), f.compressed_size()))
    });
    println!("{:?}", files);

    Ok(())
}

fn browse_zip_archive<T, F, U>(buf: &mut T, browse_func: F) -> ZipResult<Vec<U>>
    where T: Read + Seek,
          F: Fn(&ZipFile) -> ZipResult<U>
{
    let mut archive = ZipArchive::new(buf)?;
    (0..archive.len())
        .map(|i| archive.by_index(i).and_then(|file| browse_func(&file)))
        .collect()
}
