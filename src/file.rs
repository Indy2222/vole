use std::env;
use std::fs::{OpenOptions, create_dir};
use std::io::Write;
use word::Word;

pub fn add_one(word: &Word) -> Result<(), String> {
    let mut file_path = match env::home_dir() {
        Some(path) => path,
        None => return Err("Couldn't locate home directory.".to_string())
    };
    file_path.push(".vole");

    if !file_path.exists() {
        if let Err(_) = create_dir(&file_path) {
            return Err("Couldn't create .vole directory in home directory."
                       .to_string());
        }
    }

    file_path.push("dictionary.txt");

    let mut open_options = OpenOptions::new();
    open_options.append(true).create(true);
    let mut file = match open_options.open(file_path) {
        Ok(file) => file,
        Err(_) => return Err("Couldn't open dictionary file.".to_string())
    };

    let line = word.serialize();
    if let Err(_) = file.write_all(line.as_bytes()) {
        return Err("Couldn't write to dictionary file.".to_string())
    }

    Ok(())
}
