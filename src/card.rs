use rand::{thread_rng, Rng};

pub struct Card {
    id: String,
    question: String,
    answer: String,
}

impl Card {
    pub fn new(id: String, question: String, answer: String) -> Card {
        Card {
            id,
            question,
            answer,
        }
    }

    pub fn with_random_id(question: String, answer: String) -> Card {
        let consonants = b"abcdfghjklmnpqrstvwxyz";
        let mut id = String::new();

        let mut rng = thread_rng();
        for _i in 0..20 {
            let random_char = rng.choose(consonants).cloned().unwrap().into();
            id.push(random_char);
        }

        Card {
            id,
            question,
            answer,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn question(&self) -> &str {
        &self.question
    }

    pub fn answer(&self) -> &str {
        &self.answer
    }
}
