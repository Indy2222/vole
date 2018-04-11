use std::env;
use std::fs::{create_dir, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use word::Word;

static DICT_FILE_NAME: &'static str = "dictionary.txt";

pub fn add_one(word: &Word) -> Result<(), String> {
    let mut file_path = get_vole_dir()?;

    if !file_path.exists() {
        if let Err(_) = create_dir(&file_path) {
            return Err("Couldn't create .vole directory in home directory.".to_string());
        }
    }

    file_path.push(DICT_FILE_NAME);

    let mut open_options = OpenOptions::new();
    open_options.append(true).create(true);
    let mut file = match open_options.open(file_path) {
        Ok(file) => file,
        Err(_) => return Err("Couldn't open dictionary file.".to_string()),
    };

    let line = word.serialize();
    if let Err(_) = file.write_all(line.as_bytes()) {
        return Err("Couldn't write to dictionary file.".to_string());
    }

    Ok(())
}

pub fn read_all() -> Result<Vec<Word>, String> {
    let mut file_path = get_vole_dir()?;
    file_path.push(DICT_FILE_NAME);

    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => return Err("Couldn't open dictionary file.".to_string()),
    };
    let reader = BufReader::new(&file);

    let mut words: Vec<Word> = Vec::new();
    for (line_idx, line) in reader.lines().enumerate() {
        let line = match line {
            Ok(line) => line,
            Err(_) => return Err("Couldn't read dictionary file.".to_string()),
        };

        let word: Word = match Word::deserialize(&line) {
            Ok(word) => word,
            Err(reason) => {
                let reason = format!("Error on line {}: {}", line_idx + 1, reason);
                return Err(reason);
            }
        };
        words.push(word);
    }

    Ok(words)
}

fn get_vole_dir() -> Result<PathBuf, String> {
    let mut file_path = match env::home_dir() {
        Some(path) => path,
        None => return Err("Couldn't locate home directory.".to_string()),
    };
    file_path.push(".vole");
    Ok(file_path)
}
