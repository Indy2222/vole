// Copyright (C) 2018, 2019  Martin Indra
//
// This file is part of VoLe.
//
// VoLe is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

extern crate clap;
extern crate rand;
extern crate vole;

use clap::{App, Arg, ArgMatches, SubCommand};
use std::process;
use vole::{card, file, learn};

fn main() {
    let mut app = App::new("VoLe")
        .version("0.1.0")
        .author("Martin Indra <martin.indra@mgn.cz>")
        .about("CLI for flashcard learning");

    let question_arg = Arg::with_name("question").required(true);
    let answer_arg = Arg::with_name("answer").required(true);
    let add_sub_command = SubCommand::with_name("add")
        .about("Stores a new flashcard.")
        .arg(question_arg)
        .arg(answer_arg);
    app = app.subcommand(add_sub_command);

    let variant_a_arg = Arg::with_name("variant-a").required(true);
    let variant_b_arg = Arg::with_name("variant-b").required(true);
    let biadd_sub_command = SubCommand::with_name("biadd")
        .about(
            "Stores a card bidirectionally, i.e. two versions with answer \
             and question swapped.",
        )
        .arg(variant_a_arg)
        .arg(variant_b_arg);
    app = app.subcommand(biadd_sub_command);

    let learn_sub_command =
        SubCommand::with_name("learn").about("Starts question and answer learning loop.");
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

    if matches.subcommand_matches("learn").is_some() {
        learn::learning_loop()?;
        return Ok(());
    }

    panic!("Unrecognized command.")
}

fn add(question: &str, answer: &str) -> Result<(), String> {
    let reader = file::read_cards()?;
    let id: u64 = match reader.last() {
        Some(Ok(card)) => card.id() + 1,
        Some(Err(error)) => return Err(error),
        None => 0,
    };

    let card = card::Card::new(id, question.to_string(), answer.to_string());
    file::write_one(&card)
}
