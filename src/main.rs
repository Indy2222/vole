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

    let variant_a_arg = Arg::with_name("variant-a").required(true);
    let variant_b_arg = Arg::with_name("variant-b").required(true);
    let biadd_sub_command = SubCommand::with_name("biadd")
        .about("Stores a card bidirectionally, i.e. two versions with answer \
                and question swapped.")
        .arg(variant_a_arg).arg(variant_b_arg);
    app = app.subcommand(biadd_sub_command);

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
        let question = matches.value_of("question").unwrap();
        let answer = matches.value_of("answer").unwrap();
        return add(question, answer);
    }

    if let Some(matches) = matches.subcommand_matches("biadd") {
        let variant_a = matches.value_of("variant-a").unwrap();
        let variant_b = matches.value_of("variant-b").unwrap();
        add(variant_a, variant_b)?;
        return add(variant_b, variant_a);
    }

    if let Some(_) = matches.subcommand_matches("learn") {
        learn::learning_loop()?;
        return Ok(());
    }

    panic!("Unrecognized command.")
}

fn add(question: &str, answer: &str) -> Result<(), String> {
    let card = card::Card::with_random_id(
        question.to_string(), answer.to_string());
    file::write_one(&card)
}
