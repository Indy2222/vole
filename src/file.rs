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

impl Card {
    /// Serialize `Card` to a `String` of a single line; with line-feed at the
    /// end.
    pub fn to_line(&self) -> String {
        format!("{}\t{}\n", self.question(), self.answer())
    }

    /// Parse `Card` from a `&str` of a single line (ending with line-feed).
    pub fn from_line(line: &str) -> Result<Card, String> {
        let parts: Vec<&str> = line.split('\t').collect();

        if parts.len() != 2 {
            let reason = format!("Expected two TAB separated tokens, got: {}",
                                 line);
            return Err(reason);
        }

        Ok(Card::new(parts[0].to_string(), parts[1].to_string()))
    }
}

/// Append a `Card` into cards file. This opens cards wile in append mode and
/// writes a single line to it.
pub fn write_one(card: &Card) -> Result<(), String> {
    let cards_file_path = get_cards_file_path()?;

    let mut open_options = OpenOptions::new();
    open_options.append(true);
    let mut file = match open_options.open(&cards_file_path) {
        Ok(file) => file,
        Err(error) => {
            let reason = format!("Couldn't open file \"{}\": {}",
                                 cards_file_path.to_string_lossy(), error);
            return Err(reason);
        }
    };

    if let Err(error) = file.write_all(card.to_line().as_bytes()) {
        let reason = format!("Couldn't append to file \"{}\": {}",
                             cards_file_path.to_string_lossy(), error);
        return Err(reason);
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

        let card: Card = match Card::from_line(&line) {
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
