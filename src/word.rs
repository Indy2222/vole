pub struct Word {
    variant_a: String,
    variant_b: String,
}

impl Word {
    pub fn new(variant_a: String, variant_b: String) -> Word {
        Word {variant_a, variant_b}
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
}
