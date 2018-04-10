use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use word::Word;

pub fn add_one(word: &Word) -> Result<(), &'static str> {
    let mut file_path = match env::home_dir() {
        Some(path) => path,
        None => return Err("Couldn't locate home directory.")
    };
    file_path.push(".vole");
    file_path.push("dictionary.txt");

    let mut open_options = OpenOptions::new();
    open_options.append(true).create(true);
    let mut file = match open_options.open(file_path) {
        Ok(file) => file,
        Err(_) => return Err("Couldn't open dictionary file.")
    };

    let line = word.serialize();
    if let Err(_) = file.write_all(line.as_bytes()) {
        return Err("Couldn't write to dictionary file.")
    }

    Ok(())
}
