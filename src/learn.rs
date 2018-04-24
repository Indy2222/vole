use file::read_cards;
use prompt::{self, Command, CmdOption};
use qa::Qa;

#[derive(PartialEq, Clone)]
enum UserAction {
    Continue,
    AddMore,
    Quit,
}

#[derive(Clone)]
struct LoopOption {
    letter: char,
    doc: String,
    action: UserAction,
}


impl CmdOption for LoopOption {
    fn letter(&self) -> char {
        self.letter
    }

    fn doc(&self) -> &str {
        &self.doc
    }
}

struct AssessmentOption {
    q: u8,
    doc: &'static str,
}

lazy_static! {
    static ref ASSESSMENTS: Vec<AssessmentOption> = {
        vec![
            AssessmentOption {q: 0, doc: "complete blackout"},
            AssessmentOption {q: 1, doc: "incorrect response; the correct one \
                                          remembered"},
            AssessmentOption {q: 2, doc: "incorrect response; where the \
                                          correct one seemed easy to recall"},
            AssessmentOption {q: 3, doc: "correct response recalled with \
                                          serious difficulty"},
            AssessmentOption {q: 4, doc: "correct response after a \
                                          hesitation"},
            AssessmentOption {q: 5, doc: "perfect response"},
        ]
    };
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

    {
        let card = qa.current_card();
        println!("Q: {}", card.question());
        let command = Command::new("Show answer", &options);
        if read_option(&command) == UserAction::Quit {
            return UserAction::Quit;
        }

        println!("A: {}", card.answer());
    }

    let command = Command::new("How difficult was it", &*ASSESSMENTS);
    let q = prompt::prompt(&command).expect("Invalid option.").q;
    qa.assess_current(q);

    let command = Command::new("Continue with another card", &options);
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
                               &options);
    return read_option(&command);
}

