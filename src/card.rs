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

    pub fn variants(&self) -> (&str, &str) {
        (&self.question, &self.answer)
    }

    pub fn serialize(&self) -> String {
        let mut line = String::new();

        line.push_str(&self.question);
        line.push_str("\t");
        line.push_str(&self.answer);
        line.push_str("\n");
        line
    }

    pub fn deserialize(line: &str) -> Result<Card, String> {
        let parts: Vec<&str> = line.split('\t').collect();

        if parts.len() != 2 {
            return Err("Invalid number of TAB separated tokens.".to_string());
        }

        let question: String = parts[0].to_string();
        let answer: String = parts[1].to_string();
        Ok(Card {
            question,
            answer,
        })
    }
}
