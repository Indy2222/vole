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

pub struct Card {
    id: u64,
    question: String,
    answer: String,
}

impl Card {
    pub fn parse_id(id: &str) -> Result<u64, String> {
        u64::from_str_radix(id, 16)
            .map_err(|r| format!("Failed to parse ID: {}", r))
    }

    pub fn serialize_id(id: u64) -> String {
        format!("{:016x}", id)
    }

    pub fn new(id: u64, question: String, answer: String) -> Card {
        Card {
            id,
            question,
            answer,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn question(&self) -> &str {
        &self.question
    }

    pub fn answer(&self) -> &str {
        &self.answer
    }
}
