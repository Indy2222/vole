use card::Card;
use file;
use prompt::{self, Command, CmdOption};
use rand::{thread_rng, Rng};

#[derive(PartialEq, Clone)]
enum UserAction {
    Continue,
    Quit,
}

#[derive(Clone)]
struct LoopOption {
    letter: char,
    doc: String,
    action: UserAction,
}


impl CmdOption for LoopOption {
    fn letter(&self) -> char {
        self.letter
    }

    fn doc(&self) -> &str {
        &self.doc
    }
}


/// Start question answer loop. Questions, answers and options are printed to
/// standard output and user commands are read from standard input. The loop
/// continues until user enters end command.
pub fn learning_loop() -> Result<(), String> {
    let cards: Vec<Card> = file::read_cards()?
        .collect::<Result<Vec<Card>, String>>()?;

    if cards.is_empty() {
        return Err("Card list is empty.".to_string());
    }

    while iteration(&cards) != UserAction::Quit {}

    return Ok(())
}

fn iteration(cards: &Vec<Card>) -> UserAction {
    let card_count: usize = cards.len();
    let mut rng = thread_rng();
    let card_index: usize = rng.gen_range(0, card_count);
    let card: &Card = &cards[card_index];

    let yes = LoopOption {
        letter:'y',
        doc: "yes".to_string(),
        action: UserAction::Continue,
    };
    let quit = LoopOption {
        letter: 'q',
        doc: "quit".to_string(),
        action: UserAction::Quit,
    };
    let options = vec![yes, quit];

    println!("Q: {}", card.question());
    let command = Command::new("Show answer", &options);
    if read_option(&command) == UserAction::Quit {
        return UserAction::Quit;
    }

    println!("A: {}", card.answer());
    let command = Command::new("Continue with another card", &options);
    read_option(&command)
}

fn read_option(command: &Command<LoopOption>) -> UserAction {
    prompt::prompt(&command).expect("Invalid option.").action.clone()
}
