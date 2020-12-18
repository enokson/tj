use std::{print, process::{Command, Stdio}};
use std::io;
use std::io::prelude::*;
use std::fs::{self, DirEntry};
use std::path::Path;
use console::Style;
use std::env;


#[derive(Debug)]
struct Options {
    search_transcriptions: bool,
    search_vocab: bool,
    search_l2: bool,
    search_l3: bool,
    filter: String
}

fn main() {
    // println!("Hello, world!");
    // to_stdout("/tmp/level-2-vocab.txt", "hello")
    let options = parse_options();
    let pwd = Path::new("/home/joshua/Documents/tajiki-l2");
    visit_dirs(&pwd, &options);
}

fn to_stdout2(file_path: &str, filter: &str) -> io::Result<Vec<String>> {
    let f = fs::File::open(file_path)?;
    let reader = io::BufReader::new(f);
    let mut search_results: Vec<String> = vec![];
    // let mut buffer = String::new();

    // read a line into buffer

    for ln_result in reader.lines() {
        if let Ok(ln) = ln_result {
            let cyan = Style::new().red();
            if ln.contains(filter) {
                
                search_results.push(ln.replace(filter, &cyan.apply_to(filter).to_string()));
                // println!("{}", ln);
            }
        }
    }

    // reader.read_line(&mut buffer)?;

    // println!("{}", buffer);
    Ok(search_results)
}

// one possible implementation of walking a directory only visiting files
fn visit_dirs(dir: &Path, options: &Options) -> io::Result<()> {
    fn mk_output(path_str: &str, filter: &str) -> io::Result<()> {
        let search_results = to_stdout2(path_str, filter)?;
        if search_results.len() != 0 {
            let cyan = Style::new().cyan();
            println!("\n{}",  cyan.apply_to(path_str));  // here we print the ln of the file scanned
            for ln in search_results.iter() {
                println!("{}", ln);
            }                        
        }
        Ok(())
    }

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, options)?;
            } else {                
                let entry_path = entry.path();
                let path_str_o = entry_path.to_str();
                let path_str = path_str_o.unwrap();
                if options.search_l2 == true && options.search_l3 == true {
                    if options.search_transcriptions == true && options.search_vocab == true {
                        if path_str.contains("level-") && path_str.contains(".txt") {
                            mk_output(&path_str, &options.filter)?;
                        }
                    } else if options.search_transcriptions == true {
                        if path_str.contains("level-")&& path_str.contains("-transcription") && path_str.contains(".txt") {
                            mk_output(path_str, &options.filter)?;
                        }
                    } else {
                        if path_str.contains("level-")&& path_str.contains("-vocab") && path_str.contains(".txt") {
                            mk_output(path_str, &options.filter)?;
                        }
                    }
                } else if options.search_l2 == true {
                    if options.search_transcriptions == true && options.search_vocab == true {
                        if path_str.contains("level-2") && path_str.contains(".txt") {
                            mk_output(path_str, &options.filter)?;
                        }
                    } else if options.search_transcriptions == true {
                        if path_str.contains("level-2")&& path_str.contains("-transcription") && path_str.contains(".txt") {
                            mk_output(path_str, &options.filter)?;
                        }
                    } else {
                        if path_str.contains("level-2")&& path_str.contains("-vocab") && path_str.contains(".txt") {
                            mk_output(path_str, &options.filter)?;
                        }                        
                    }
                } else {
                    if options.search_transcriptions == true && options.search_vocab == true {
                        if path_str.contains("level-3") && path_str.contains(".txt") {
                            mk_output(path_str, &options.filter)?;
                        }
                    } else if options.search_transcriptions == true {
                        if path_str.contains("level-3") && path_str.contains("-transcription") && path_str.contains(".txt") {
                            mk_output(path_str, &options.filter)?;
                        }
                    } else {
                        if path_str.contains("level-3")&& path_str.contains("-vocab") && path_str.contains(".txt") {
                            mk_output(path_str, &options.filter)?;
                        }                        
                    }
                }
                // if path_str.contains(".txt") {
                    
                //     // to_stdout(path_str, filter);
                //     let search_results = to_stdout2(path_str, filter)?;
                //     if search_results.len() != 0 {
                //         println!("\n{}", path_str);  // here we print the ln of the file scanned
                //         for ln in search_results.iter() {
                //             println!("{}", ln);
                //         }                        
                //     }
                // }                
            }
        }
    }
    Ok(())
}

fn parse_options() -> Options {
    let mut options = Options {
        search_transcriptions: false,
        search_vocab: false,
        search_l2: false,
        search_l3: false,
        filter: "".to_string()
    };
    for arg in env::args().into_iter() {
        if arg == "-l2" {
            options.search_l2 = true;
        } else if arg == "-l3" {
            options.search_l3 = true;
        } else if arg == "-v" {
            options.search_vocab = true;
        } else if arg == "-t" {
            options.search_transcriptions = true;
        }
    }
    options.filter = env::args().last().unwrap_or("jhskjhgfiushgiousdfhgiusfhgflsdyurunsdhgiuw".to_string());
    if options.search_l2 == false && options.search_l3 == false {
        options.search_l2 = true;
        options.search_l3 = true;
    }
    if options.search_transcriptions == false && options.search_vocab == false {
        options.search_transcriptions = true;
        options.search_vocab = true;
    }
    options
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