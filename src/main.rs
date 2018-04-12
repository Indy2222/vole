extern crate rand;
extern crate vole;

use rand::{thread_rng, Rng};
use std::env;
use std::iter::Iterator;
use std::process;
use vole::{file, card};

fn main() {
    let mut args = env::args().skip(1);

    let cmd = next_arg(&mut args);

    if let Err(report) = execute(&cmd, args) {
        eprint!("{}\n", report);
        process::exit(1);
    }
}

fn execute<T>(cmd: &str, mut args: T) -> Result<(), String>
where
    T: Iterator<Item = String>,
{
    if cmd == "add" {
        let value_a: String = next_arg(&mut args);
        let value_b: String = next_arg(&mut args);

        let card = card::Card::with_random_id(value_a, value_b);
        file::write_one(&card)?;
    } else if cmd == "rnd" {
        let cards: Vec<card::Card> = file::read_all_cards()?;
        let card_count: usize = cards.len();

        if card_count == 0 {
            return Err("Dictionary is empty.".to_string());
        }

        let mut rng = thread_rng();
        let card_index: usize = rng.gen_range(0, card_count);
        let card: &card::Card = &cards[card_index];

        println!("{} | {}", card.question(), card.answer());
    } else {
        let err = format!("Unrecognized argument: {}", cmd);
        return Err(err);
    }

    Ok(())
}

fn next_arg<T: Iterator<Item = String>>(args: &mut T) -> String {
    match args.next() {
        Some(arg) => arg,
        None => {
            eprint!(
                "Invalid arguments, try: \
                 vole add <variant-a> <variant-b>\n"
            );
            process::exit(1);
        }
    }
}
