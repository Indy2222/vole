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

use std::io::{self, Write};

const HELP_LETTER: char = '?';

fn option_help(letter: char, doc: &str) -> String {
    format!("{} - {}\n", letter, doc)
}

pub trait CmdOption {
    fn letter(&self) -> char;

    fn doc(&self) -> &str;

    fn help(&self) -> String {
        option_help(self.letter(), self.doc())
    }
}

pub struct Command<'a, T>
where
    T: 'a + CmdOption,
{
    question: &'a str,
    options: &'a [T],
}

impl<'a, T> Command<'a, T>
where
    T: CmdOption,
{
    /// # Panics
    ///
    /// When list of options is empty.
    pub fn new(question: &'a str, options: &'a [T]) -> Self {
        if options.is_empty() {
            panic!("Got empty list of options.");
        }

        Command { question, options }
    }

    fn prompt(&self) -> String {
        let mut prompt = String::from(self.question);
        prompt.push_str(" [");
        for option in self.options.iter() {
            prompt.push(option.letter());
            prompt.push_str(", ");
        }
        prompt.push(HELP_LETTER);
        prompt.push_str("]? ");
        prompt
    }

    fn help(&self) -> String {
        let mut help = String::new();
        for option in self.options.iter() {
            help.push_str(&option.help());
        }
        help.push_str(&option_help(HELP_LETTER, "help"));
        help
    }

    fn parse(&self, input: &str) -> ParsingResult<&T> {
        let input: &str = input.trim();

        if input.len() != 1 {
            return ParsingResult::Err;
        }
        let input: char = input.chars().next().unwrap();

        if HELP_LETTER == input {
            return ParsingResult::Help;
        }

        for option in self.options.iter() {
            if option.letter() == input {
                return ParsingResult::Option(option);
            }
        }

        ParsingResult::Err
    }
}

enum ParsingResult<T> {
    Help,
    Option(T),
    Err,
}

/// Print question to standard output and read answer from standard input.
/// Give user multiple tries if she doesn't answer with a correct question.
///
/// # Errors
///
/// User didn't give a valid answer.
pub fn prompt<'a, T>(command: &'a Command<'a, T>) -> Result<&'a T, ()>
where
    T: CmdOption,
{
    let mut attempts = 0;
    loop {
        let mut out = io::stdout();

        out.write_all(command.prompt().as_bytes()).unwrap();
        out.flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match command.parse(&input) {
            ParsingResult::Help => {}
            ParsingResult::Option(option) => {
                return Ok(option);
            }
            ParsingResult::Err => attempts += 1,
        }

        if attempts >= 2 {
            break;
        }

        out.write_all(command.help().as_bytes()).unwrap();
        out.flush().unwrap();
    }

    Err(())
}
