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
