use std::{
    fs::{DirEntry, create_dir_all, read_dir}, 
    path::{Path}, 
    io::{Result}
};

mod comments;
mod args;
mod utils;

use args::Args;
use clap::Parser;
use utils::{makedirs, write_data};

use crate::utils::read_to_strings;

fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> Result<()> {
    if dir.is_dir() {
        for entry in read_dir(dir)?
        .filter(|x| !x.as_ref().unwrap().path().to_str().unwrap().ends_with(".git") ) {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

fn go(args: Args) {
    let (file, extension, out_dir) = (args.file, args.extension.as_str(), args.out);
    
    visit_dirs(Path::new(&file), &|entry: &DirEntry| {
        match &entry.clone().path().file_name().unwrap().to_str().unwrap().ends_with(extension) {
            true => {
                let target = entry.path().to_str().unwrap().to_owned();
                println!("[*] Cleaning: {}", target);
                let data: Vec<String> = read_to_strings(&target);
                match makedirs(&entry.path(), Path::new(&out_dir)) {
                    Ok(_out_file) => {
                        write_data(&comments::clean_comments(data), _out_file.to_str().unwrap());
                        println!("[+] Comments removed from: {}", target);
                    },
                    Err(e) => { println!("[-] Error creating directory: {} | {e}", entry.path().to_str().unwrap()); }
                }
            },
            false => {
                match makedirs(&entry.path(), Path::new(&out_dir)) {
                    Ok(out_file) => { std::fs::copy(entry.path(), out_file).unwrap(); },
                    Err(e) => { println!("[-] Error creating directory: {} | {e}", entry.path().to_str().unwrap()); } 
                }
            },
        }
    }).expect("[-] Error cleaning files")
}

fn main() {
    let args = args::Args::parse();
    match create_dir_all(Path::new(&args.out)) {
        Ok(_) => { go(args); }
        Err(e) => { println!("Error creating output directory: {} | {e}", &args.out); }
    }
}
