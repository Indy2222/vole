pub struct Word {
    variant_a: String,
    variant_b: String,
}

impl Word {
    pub fn new(variant_a: String, variant_b: String) -> Word {
        Word {
            variant_a,
            variant_b,
        }
    }

    pub fn variants(&self) -> (&str, &str) {
        (&self.variant_a, &self.variant_b)
    }

    pub fn serialize(&self) -> String {
        let mut line = String::new();

        line.push_str(&self.variant_a);
        line.push_str("\t");
        line.push_str(&self.variant_b);
        line.push_str("\n");
        line
    }

    pub fn deserialize(line: &str) -> Result<Word, String> {
        let parts: Vec<&str> = line.split('\t').collect();

        if parts.len() != 2 {
            return Err("Invalid number of TAB separated tokens.".to_string());
        }

        let variant_a: String = parts[0].to_string();
        let variant_b: String = parts[1].to_string();
        Ok(Word {
            variant_a,
            variant_b,
        })
    }
}
