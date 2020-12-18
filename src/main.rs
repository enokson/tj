use std::{print, process::{Command, Stdio}};
use std::io;
use std::io::prelude::*;
use std::fs::{self, DirEntry};
use std::path::Path;


fn main() {
    // println!("Hello, world!");
    // to_stdout("/tmp/level-2-vocab.txt", "hello")
    let pwd = Path::new("/home/joshua/Documents/tajiki-l2");
    visit_dirs(&pwd, "дар");
}

fn to_stdout2(file_path: &str, filter: &str) -> io::Result<Vec<String>> {
    let f = fs::File::open(file_path)?;
    let reader = io::BufReader::new(f);
    let mut search_results: Vec<String> = vec![];
    // let mut buffer = String::new();

    // read a line into buffer

    for ln_result in reader.lines() {
        if let Ok(ln) = ln_result {
            if ln.contains(filter) {
                search_results.push(ln);
                // println!("{}", ln);
            }
        }
    }

    // reader.read_line(&mut buffer)?;

    // println!("{}", buffer);
    Ok(search_results)
}

// one possible implementation of walking a directory only visiting files
fn visit_dirs(dir: &Path, filter: &str) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, filter)?;
            } else {                
                let entry_path = entry.path();
                let path_str_o = entry_path.to_str();
                let path_str = path_str_o.unwrap();
                if path_str.contains(".txt") {
                    
                    // to_stdout(path_str, filter);
                    let search_results = to_stdout2(path_str, filter)?;
                    if search_results.len() != 0 {
                        println!("\n{}", path_str);  // here we print the ln of the file scanned
                        for ln in search_results.iter() {
                            println!("{}", ln);
                        }                        
                    }
                }                
            }
        }
    }
    Ok(())
}

/* 

read input
parse options
concat ramaining string
``` tj -l2 to boil => cat level-2-vocab.txt | grep "to boil"
scan the directory
echo/grep files
scan child directories and repeat

*/