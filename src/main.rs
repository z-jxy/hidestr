use std::{
    fs::{DirEntry, create_dir_all, read_dir}, 
    path::{Path}, 
    io::{Result},
};

mod tokenizer;

mod comments;
mod args;
mod utils;

use args::Args;
use clap::Parser;
use utils::{makedirs, write_data};

use crate::{utils::read_to_strings, tokenizer::Token};

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

fn go(args: Args, target_token: &Token) {
    let (file, extension, out_dir) = (args.file, args.extension.as_str(), args.out);

    visit_dirs(Path::new(&file), &|entry: &DirEntry| {
        match &entry.clone().path().file_name().unwrap().to_str().unwrap().ends_with(extension) {
            true => {
                match clean(entry.path().to_str().unwrap(), &out_dir, &target_token) {
                    Ok(out_file) => { println!("[+] File saved to: {}", &out_file); },
                    Err(e) => { println!("[-] Error cleaning file: {} | {e}", entry.path().to_str().unwrap()); }
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

fn clean(target: &str, out_dir: &str, token: &Token) -> Result<String> {
    let target_path = Path::new(target);
    println!("[*] Cleaning file: {}", target); 
    let data: Vec<String> = read_to_strings(target);
    match makedirs(&target_path.to_path_buf(), Path::new(&out_dir)) {
        Ok(file_path) => {
            write_data(&comments::clean_comments(data, &token), file_path.to_str().unwrap());
            println!("[+] Comments removed from: {}", target);
            Ok(file_path.to_str().unwrap().to_owned())
        },
        Err(e) => { 
            println!("[-] Error creating directory: {} | {e}", target); 
            Err(e)
        }
    }
}

fn main() {
    let args = args::Args::parse();
    let metadata = std::fs::metadata(&args.file);
    match metadata {
        Ok(_) => { 
            match metadata.unwrap().is_dir() {
                true => { 
                    if !args.recursive {
                        println!("[-] Error: {} is a directory, use --recursive to clean all files in directory", &args.file);
                        return;
                    }
                 },
                false => {
                    match clean(&args.file, &args.out, &Token::get_token_type(Path::new(&args.file).extension().unwrap().to_str().unwrap()) ) {
                        Ok(out) => { println!("[+] File saved to: {}", &out); }
                        Err(e) => { println!("[-] Error cleaning file: {} | {e}", &args.file); }
                    }
                    return;
                }
            }
         }
        Err(e) => { println!("Error checking output directory: {} | {e}", &args.out); }
    }
    let target_token = Token::get_token_type(&args.extension);
    match create_dir_all(Path::new(&args.out)) {
        Ok(_) => { go(args, &target_token); }
        Err(e) => { println!("Error creating output directory: {} | {e}", &args.out); }
    }
}
