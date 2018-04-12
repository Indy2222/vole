pub struct Card {
    question: String,
    answer: String,
}

impl Card {
    pub fn new(question: String, answer: String) -> Card {
        Card {
            question,
            answer,
        }
    }

    pub fn question(&self) -> &str {
        &self.question
    }

    pub fn answer(&self) -> &str {
        &self.answer
    }
}
