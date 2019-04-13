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

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use regex::{self, Regex};
use std::process;
use vole::{card::Card, file, learn};

fn main() {
    let app = App::new("VoLe")
        .version("0.1.0")
        .author("Martin Indra <martin.indra@mgn.cz>")
        .about("CLI for flashcard learning")
        .setting(AppSettings::SubcommandRequired)
        .setting(AppSettings::VersionlessSubcommands)
        .subcommand(
            SubCommand::with_name("add")
                .about("Stores a new flashcard.")
                .arg(
                    Arg::with_name("bidir")
                        .long("bidirectional")
                        .short("b")
                        .help(
                            "Stores a card bidirectionally, id est two \
                             versions with answer and question swapped.",
                        ),
                )
                .arg(Arg::with_name("question").required(true))
                .arg(Arg::with_name("answer").required(true)),
        )
        .subcommand(
            SubCommand::with_name("learn").about("Starts question and answer learning loop."),
        )
        .subcommand(
            SubCommand::with_name("find")
                .about(
                    "Searches all questions and answers with a regular \
                     expression and prints all matching cards.",
                )
                .arg(
                    Arg::with_name("regex")
                        .help(
                            "A matching regular expression with syntax \
                             documented at https://docs.rs/regex/1/regex/#syntax",
                        )
                        .required(true),
                ),
        );

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

        return if matches.is_present("bidir") {
            add(&[(question, answer), (answer, question)])
        } else {
            add(&[(question, answer)])
        };
    }

    if let Some(matches) = matches.subcommand_matches("find") {
        let regexp = matches.value_of("regex").unwrap();
        return find(regexp);
    }

    matches.subcommand_matches("learn").unwrap();
    learn::learning_loop()?;
    Ok(())
}

fn find(regex: &str) -> Result<(), String> {
    let regex = match Regex::new(regex) {
        Ok(regex) => regex,
        Err(why) => return Err(format!("Invalid regex: {}", why)),
    };

    let reader = file::read_cards()?;
    for card in reader {
        let card = match card {
            Ok(card) => card,
            Err(why) => return Err(why),
        };
        if !regex.is_match(card.question()) && !regex.is_match(card.answer()) {
            continue;
        }
        print!("{}", card.to_line());
    }

    Ok(())
}

fn add(qa: &[(&str, &str)]) -> Result<(), String> {
    let reader = file::read_cards()?;
    let last_id: u64 = match reader.last() {
        Some(Ok(card)) => card.id(),
        Some(Err(error)) => return Err(error),
        None => 0,
    };

    let cards: Vec<Card> = qa
        .iter()
        .scan(last_id, |last_id, &(q, a)| {
            *last_id += 1;
            Some(Card::new(*last_id, String::from(q), String::from(a)))
        })
        .collect();
    file::store_cards(&cards)
}
