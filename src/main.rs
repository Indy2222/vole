extern crate vole;

use std::env;
use std::iter::Iterator;
use std::process;
use vole::{word, file};

fn main() {
    let mut args = env::args().skip(2);
    let value_a: String = next_arg(&mut args);
    let value_b: String = next_arg(&mut args);

    let word = word::Word::new(value_a, value_b);

    if let Err(report) = file::add_one(&word) {
        eprint!("{}\n", report);
        process::exit(1);
    }
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
