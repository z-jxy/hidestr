use std::{fs::{DirEntry, create_dir_all, self, File}, path::{Path, self}, io::{Result, Write}, os::{unix::prelude::PermissionsExt},};

use clap::Parser;


fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
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

fn remove_comments(lines: Vec<String>) -> String {
    let mut output = Vec::new();
    for line in lines {
        if line.contains("//") {

            if line.trim().starts_with("//") {
                continue;
            }
            
            let idx = line.find("//").unwrap();

            if line.contains("://") {
                output.push(line.clone());
                continue;
            }

            if line.contains('\'') || line.contains('"') {
                let q_idx = line.find('\'');
                if let Some(q_idx) = q_idx {
                    let r_q = line.find('\'').unwrap();
                    if q_idx < idx && idx < r_q {
                        output.push(line.clone());
                        continue;
                    }
                }

                let q_idx = line.find('"');
                if let Some(q_idx) = q_idx {
                    let r_q = line.find('"').unwrap();
                    if q_idx < idx && idx < r_q {
                        output.push(line.clone());
                        continue;
                    }
                }
            }

            output.push(format!("{}", line[..idx].trim()));
            continue;
        }
        output.push(line.clone());
    }
    //output.remove(0);

    let output = output
        .iter()
        .filter(|x| !x.is_empty())
        .map(|x| format!("{}", x))
        .collect::<Vec<String>>();
    output.join("\n")
}


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   
    #[arg(short, long)]
    file: String,

    #[arg(short, long)]
    extension: String,

    #[arg(short, long, default_value = "./obfuscated")]
    out: String,
}

fn write_data(data: &str, path: &str) {
    let mut file = File::create(path).unwrap();
    file.write_all(data.as_bytes()).unwrap();
}

fn makedirs(entry: &DirEntry, outpath: &Path) -> Result<()> {
    let parent = entry.path().parent().unwrap().to_path_buf();
    match create_dir_all(path::Path::join(outpath, &parent)) {
        Ok(_) => {
            let o = path::Path::join(outpath, 
                entry.path().to_str().unwrap().to_owned());
                //fs::set_permissions(o, Permissions::from_mode(0o777)).unwrap();
            Ok(())
        },
        Err(e) => {
            println!("Error creating directory: {}", parent.to_str().unwrap());
            Err(e)
        },
    }
}

fn main() {
    let args = Args::parse();

    let file = args.file;
    let extension = args.extension.as_str();
    let out_dir = args.out;

    let outpath = Path::new(&out_dir);

    fs::create_dir_all(outpath).unwrap();

    visit_dirs(Path::new(&file), &|entry: &DirEntry| {
        let binding = entry.path();
        
        let entry_file = binding.file_name().unwrap().to_str().unwrap();

        match entry_file.ends_with(extension) {
            true => {
                let file_path = binding.to_str().unwrap();

                println!("[*] Cleaning: {}", file_path);
                let data = fs::read_to_string(file_path).unwrap();

                let removed_comments = remove_comments(data.split("\n").map(|x | x.to_owned()).collect::<Vec<String>>());

                let parent = binding.parent().unwrap().to_path_buf();

                match create_dir_all(path::Path::join(outpath, &parent)) {
                    Ok(_) => {},
                    Err(_) => {
                        println!("Error creating directory: {}", parent.to_str().unwrap());
                    },
                }

                let out = path::Path::join(outpath, file_path);

                write_data(&removed_comments, out.to_str().unwrap());

                let mut perms = fs::metadata(out).unwrap().permissions();
                perms.set_mode(0o777);


                println!("[+] Comments removed from: {}", file_path);
            },
            false => {
                //let out = path::Path::join(outpath, entry.path().to_str().unwrap().to_owned());
                let out = path::Path::join(outpath, entry.path().to_str().unwrap().to_owned());
                match makedirs(entry, outpath) {
                    Ok(_) => {
                        std::fs::copy(entry.path(), &out).unwrap();
                        let mut perms = fs::metadata(out).unwrap().permissions();
                        perms.set_mode(0o777);
                    },
                    Err(_) => {
                        println!("Error creating directory: {}", entry.path().to_str().unwrap());
                    },
                }
            },
        }
    }).expect("Error cleaning files")

}
