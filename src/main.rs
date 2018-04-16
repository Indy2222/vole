extern crate clap;
extern crate rand;
extern crate vole;

use clap::{Arg, ArgMatches, App, SubCommand};
use std::process;
use vole::{file, card, learn};

fn main() {
    let mut app = App::new("VoLe").version("0.1.0")
        .author("Martin Indra <martin.indra@mgn.cz>")
        .about("CLI for flashcard learning");

    let question_arg = Arg::with_name("question").required(true);
    let answer_arg = Arg::with_name("answer").required(true);
    let add_sub_command = SubCommand::with_name("add")
        .about("Stores a new flashcard.")
        .arg(question_arg).arg(answer_arg);
    app = app.subcommand(add_sub_command);

    let learn_sub_command = SubCommand::with_name("learn")
        .about("Starts question and answer learning loop.");
    app = app.subcommand(learn_sub_command);

    let matches = app.get_matches();
    if let Err(report) = execute(matches) {
        eprintln!("{}", report);
        process::exit(1);
    }
}

fn execute(matches: ArgMatches) -> Result<(), String> {
    if let Some(matches) = matches.subcommand_matches("add") {
        let question = matches.value_of("question").unwrap().to_string();
        let answer = matches.value_of("answer").unwrap().to_string();
        let card = card::Card::with_random_id(question, answer);
        file::write_one(&card)?;
        return Ok(());
    }

    if let Some(_) = matches.subcommand_matches("learn") {
        learn::learning_loop()?;
        return Ok(());
    }

    panic!("Unrecognized command.")
}
