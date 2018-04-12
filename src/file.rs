use std::env;
use std::fs::{create_dir, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use card::Card;

/// All VoLe files are place to a directory inside users home directory. This
/// is name of the directory.
const VOLE_DIR_NAME: &str = ".vole";
/// File name of the file storing all cards.
const CARDS_FILE_NAME: &str = "cards.txt";

pub fn add_one(card: &Card) -> Result<(), String> {
    let cards_file_path = get_cards_file_path()?;

    let mut open_options = OpenOptions::new();
    open_options.append(true);
    let mut file = match open_options.open(cards_file_path) {
        Ok(file) => file,
        Err(_) => return Err("Couldn't open dictionary file.".to_string()),
    };

    let line = card.serialize();
    if let Err(_) = file.write_all(line.as_bytes()) {
        return Err("Couldn't write to dictionary file.".to_string());
    }

    Ok(())
}

pub fn read_all() -> Result<Vec<Card>, String> {
    let cards_file_path = get_cards_file_path()?;

    let file = match File::open(cards_file_path) {
        Ok(file) => file,
        Err(_) => return Err("Couldn't open dictionary file.".to_string()),
    };
    let reader = BufReader::new(&file);

    let mut cards: Vec<Card> = Vec::new();
    for (line_idx, line) in reader.lines().enumerate() {
        let line = match line {
            Ok(line) => line,
            Err(_) => return Err("Couldn't read dictionary file.".to_string()),
        };

        let card: Card = match Card::deserialize(&line) {
            Ok(card) => card,
            Err(reason) => {
                let reason = format!("Error on line {}: {}", line_idx + 1, reason);
                return Err(reason);
            }
        };
        cards.push(card);
    }

    Ok(cards)
}

/// This returns path to use's card file and creates vole directory and card
/// file along the way if necessary.
///
/// # Errors
///
/// In case of an I/O or other error a `String` with reason is returned.
fn get_cards_file_path() -> Result<PathBuf, String> {
    let mut file_path = match env::home_dir() {
        Some(path_buf) => path_buf,
        None => return Err("Couldn't locate home directory.".to_string()),
    };

    file_path.push(&VOLE_DIR_NAME);
    if !file_path.exists() {
        if let Err(error) = create_dir(&file_path) {
            let reason = format!("Couldn't create \"{}\" directory: {}",
                                 file_path.to_string_lossy(), error);
            return Err(reason);
        }
    }

    file_path.push(&CARDS_FILE_NAME);
    if !file_path.exists() {
        if let Err(error) = File::create(&file_path) {
            let reason = format!("Couldn't create \"{}\" file: {}",
                                 file_path.to_string_lossy(), error);
            return Err(reason);
        }
    }

    Ok(file_path)
}
