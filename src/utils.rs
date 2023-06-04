use std::{fs::{create_dir_all, File, read_to_string}, path::{Path, PathBuf}, io::{Result, Write}};

pub fn write_data(data: &str, path: &str) {
    let mut file = File::create(path).unwrap();
    file.write_all(data.as_bytes()).unwrap();
}

pub fn makedirs(entry: &PathBuf, outpath: &Path) -> Result<PathBuf> {
    let out = Path::join(outpath, entry.parent().unwrap());
    match create_dir_all(&out) {
        Ok(_) => Ok(Path::join(out.as_path(), entry.file_name().unwrap())),
        Err(e) => {
            println!("Error creating directory: {}", entry.parent().unwrap().to_path_buf().to_str().unwrap());
            Err(e)
        },
    }
}

pub fn read_to_strings(target: &str) -> Vec<String> {
    read_to_string(target)
        .unwrap()
        .split("\n")
        .map(|x| x.to_owned())
        .collect()
}