use std::io::{self, Write};

const HELP_LETTER: char = '?';

pub struct CmdOption {
    letter: char,
    doc: String,
}

pub struct Command {
    question: String,
    options: Vec<CmdOption>,
}

impl CmdOption {
    pub fn new(letter: char, doc: &str) -> Self {
        CmdOption {
            letter: letter,
            doc: doc.to_string(),
        }
    }

    pub fn letter(&self) -> &char {
        &self.letter
    }

    pub fn help(&self) -> String {
        format!("{} - {}", self.letter, self.doc)
    }
}

impl Command {
    pub fn new(question: &str, options: Vec<CmdOption>) -> Self {
        if options.is_empty() {
            panic!("Got empty list of options.");
        }

        for option in options.iter() {
            if *option.letter() == '?' {
                panic!("Letter ? is not allowed.");
            }
        }

        let mut options = options;
        options.push(CmdOption::new(HELP_LETTER, "help"));

        Command {
            question: question.to_string(),
            options: options,
        }
    }

    fn prompt(&self) -> String {
        let mut prompt = String::from(&self.question[..]);
        prompt.push_str(" [");
        for (i, option) in self.options.iter().enumerate() {
            if i > 0 {
                prompt.push_str(", ");
            }
            prompt.push(option.letter().clone());
        }
        prompt.push_str("]? ");
        prompt
    }

    fn help(&self) -> String {
        let mut help = String::new();
        for option in self.options.iter() {
            help.push_str(&option.help());
            help.push('\n');
        }
        help
    }

    fn parse(&self, input: &str) -> Result<&CmdOption, ()> {
        let input = input.trim();

        for option in self.options.iter() {
            if option.letter().to_string() == input {
                return Ok(&option);
            }
        }

        Err(())
    }
}

/// Print question to standard output and read answer from standard input.
/// Give user multiple tries if she doesn't answer with a correct question.
///
/// # Errors
///
/// User didn't give a valid answer.
pub fn prompt(command: &Command) -> Result<&CmdOption, ()> {
    let mut attempts = 0;
    loop {
        let mut out = io::stdout();

        out.write(command.prompt().as_bytes()).unwrap();
        out.flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if let Ok(option) = command.parse(&input) {
            if *option.letter() == HELP_LETTER {
                attempts -= 1;
            } else {
                return Ok(option);
            }
        }

        if attempts >= 2 {
            break;
        }

        out.write(command.help().as_bytes()).unwrap();
        out.flush().unwrap();

        attempts += 1;
    }

    Err(())
}
