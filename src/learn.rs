// Copyright (C) 2018  Martin Indra
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

use file::read_cards;
use prompt::{self, Command, CmdOption};
use qa::Qa;
use std::io;

#[derive(PartialEq, Clone)]
enum UserAction {
    Continue,
    AddMore,
    Quit,
}

#[derive(Clone)]
struct YesQuit {
    Yes,
    Quit
}

struct AssessmentOption {
    q: u8,
    doc: &'static str,
}


static YES_NO: [LoopOption; ]

static ASSESSMENTS: [AssessmentOption; 6] = [
    AssessmentOption {
        q: 0,
        doc: "complete blackout"
    },
    AssessmentOption {
        q: 1,
        doc: "incorrect response; the correct one remembered"
    },
    AssessmentOption {
        q: 2,
        doc: "incorrect response; where the correct one seemed easy to recall"
    },
    AssessmentOption {
        q: 3,
        doc: "correct response recalled with serious difficulty"
    },
    AssessmentOption {
        q: 4,
        doc: "correct response after a hesitation"
    },
    AssessmentOption {
        q: 5,
        doc: "perfect response"
    },
];


let yes = LoopOption {
        letter:'y',
        doc: "yes".to_string(),
        action: UserAction::Continue,
    };
    let quit = LoopOption {
        letter: 'q',
        doc: "quit".to_string(),
        action: UserAction::Quit,
    };
    let options = vec![yes, quit];



impl CmdOption for LoopOption {
    fn letter(&self) -> char {
        match self {
            LoopOption::Yes => 'y',
            LoopOption::Quit => 'q',
        }
    }

    fn doc(&self) -> &str {
        match self {
            LoopOption::Yes => "yes",
            LoopOption::Quit => "quit",
        }
    }
}



impl CmdOption for AssessmentOption {
    fn letter(&self) -> char {
        (self.q + 48) as char
    }

    fn doc(&self) -> &str {
        self.doc
    }
}


/// Start question answer loop. Questions, answers and options are printed to
/// standard output and user commands are read from standard input. The loop
/// continues until user enters end command.
pub fn learning_loop() -> Result<(), String> {
    let reader = read_cards()?;
    let mut qa = Qa::load(reader)?;

    let mut next_action = UserAction::Continue;
    while next_action != UserAction::Quit {
        if next_action == UserAction::AddMore {
            qa.schedule_more(8);
        }
        next_action = iteration(&mut qa);
    }

    qa.save()?;
    return Ok(())
}

fn iteration(qa: &mut Qa) -> UserAction {
    if qa.is_today_schedule_done() {
        ask_for_more(qa)
    } else {
        show_card(qa)
    }
}

fn read_option(command: &Command<LoopOption>) -> UserAction {
    prompt::prompt(&command).expect("Invalid option.").action.clone()
}

fn show_card(qa: &mut Qa) -> UserAction {
    {
        let card = qa.current_card();
        println!("Q: {}", card.question());
        {
            // show answer after user presses RET
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
        }
        println!("A: {}", card.answer());
    }


    let command = Command::new("How difficult was it", &ASSESSMENTS, None);
    let q = prompt::prompt(&command).expect("Invalid option.").q;
    qa.assess_current(q);

    let command = Command::new("Continue with another card", &options, Some(0));
    read_option(&command)
}

fn ask_for_more(qa: &Qa) -> UserAction {
    if qa.is_all_scheduled() {
        println!("This is it for today! There are no unscheduled cards.");
        return UserAction::Quit;
    }

    let yes = LoopOption {
        letter:'y',
        doc: "yes".to_string(),
        action: UserAction::AddMore,
    };
    let quit = LoopOption {
        letter: 'q',
        doc: "quit".to_string(),
        action: UserAction::Quit,
    };

    let options = vec![yes, quit];
    let command = Command::new("No more items planned for today, add more",
                               &options, None);
    return read_option(&command);
}

