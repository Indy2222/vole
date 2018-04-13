extern crate rand;
extern crate vole;

use std::env;
use std::iter::Iterator;
use std::process;
use vole::{file, card, learn};

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
    } else if cmd == "learn" {
        learn::learning_loop()?;
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
