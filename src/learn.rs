use card::Card;
use file;
use prompt::{self, Command, CmdOption};
use rand::{thread_rng, Rng};

#[derive(PartialEq, Clone)]
enum UserAction {
    Continue,
    Finish,
}

/// Start question answer loop. Questions, answers and options are printed to
/// standard output and user commands are read from standard input. The loop
/// continues until user enters end command.
pub fn learning_loop() -> Result<(), String> {
    let cards: Vec<Card> = file::read_all_cards()?;

    if cards.is_empty() {
        return Err("Card list is empty.".to_string());
    }

    let mut next_action = UserAction::Continue;
    while next_action != UserAction::Finish {
        next_action = iteration(&cards);
    }

    Ok(())
}

fn iteration(cards: &Vec<Card>) -> UserAction {
    let card_count: usize = cards.len();
    let mut rng = thread_rng();
    let card_index: usize = rng.gen_range(0, card_count);
    let card: &Card = &cards[card_index];

    println!("Q: {}", card.question());

    let yes = CmdOption::new('y', "yes");
    let command = Command::new("Show answer", vec![yes]);
    read_option(&command);

    println!("A: {}", card.answer());

    let yes = CmdOption::new('y', "yes");
    let quit = CmdOption::new('q', "quit");
    let command = Command::new("Continue with another card", vec![yes, quit]);
    match *read_option(&command) {
        'y' => UserAction::Continue,
        'q' => UserAction::Finish,
        _ => panic!("Unrecognized option."),
    }
}

fn read_option(command: &Command) -> &char {
    prompt::prompt(&command).expect("Invalid option.").letter()
}
