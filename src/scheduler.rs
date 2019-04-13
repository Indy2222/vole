// Source Code, Copyright (C) 2018, 2019  Martin Indra
// Algorithm SM-2, (C) Copyright SuperMemo World, 1991.
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

// SuperMemo 2 Algorithm
//
// This file contains re-implementation of SuperMemo 2 algorithm. See
// http://supermemopedia.com/wiki/Licensing_SuperMemo_Algorithm
// for information about SM-2 algorithm licensing
//
//    http://www.supermemo.com
//    http://www.supermemo.eu

use crate::card::Card;
use crate::file::get_vole_dir;
use chrono::{prelude::*, Duration, NaiveDate};
use fnv::FnvHashMap;
use std::collections::VecDeque;
use std::fs::{rename, File};
use std::io::{BufRead, BufReader, ErrorKind, Write};

const SCHEDULE_FILE_NAME: &str = "schedule.txt";

struct ScheduleItem {
    iteration: u32,
    ef: f32,
    last_revisit: NaiveDate,
    next_revisit: NaiveDate,
}

pub struct Schedule {
    items: FnvHashMap<u64, ScheduleItem>,
    stage: usize,
    hot_stage: VecDeque<u64>,
    refresh_stage: VecDeque<u64>,
}

fn today() -> NaiveDate {
    Local::today().naive_local()
}

impl Default for ScheduleItem {
    fn default() -> ScheduleItem {
        ScheduleItem {
            iteration: 0,
            ef: 2.5,
            // note that time between last_revisit and today does play any
            // role only after first two visits so it is possible set today
            // without any harm.
            last_revisit: today(),
            next_revisit: today(),
        }
    }
}

impl ScheduleItem {
    fn serialize(&self, id: u64) -> String {
        let last_revisit = self.last_revisit.format("%Y-%m-%d");
        let next_revisit = self.next_revisit.format("%Y-%m-%d");
        format!(
            "{id}\t{next_revisit}\t{last_revisit}\t{iteration}\t{ef}\n",
            id = Card::serialize_id(id),
            next_revisit = next_revisit,
            last_revisit = last_revisit,
            iteration = self.iteration,
            ef = self.ef
        )
    }

    /// Parse `ScheduleItem` and its ID (hence the tuple) from a text line.
    fn deserialize(line: &str) -> Result<(u64, ScheduleItem), String> {
        fn parse_date(source: &str) -> Result<NaiveDate, String> {
            match NaiveDate::parse_from_str(source, "%Y-%m-%d") {
                Ok(date) => Ok(date),
                Err(reason) => Err(format!("Failed to parse date: {}", reason)),
            }
        }

        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() != 5 {
            let reason = format!("Expected five TAB separated tokens, got: {}", line);
            return Err(reason);
        }

        let id: u64 = Card::parse_id(parts[0])?;
        let next_revisit: NaiveDate = parse_date(parts[1])?;
        let last_revisit: NaiveDate = parse_date(parts[2])?;
        let iteration: u32 = match parts[3].parse() {
            Ok(iteration) => iteration,
            Err(reason) => {
                let msg = format!("Failed to parse iteration: {}", reason);
                return Err(msg);
            }
        };
        let ef: f32 = match parts[4].parse() {
            Ok(ef) => ef,
            Err(reason) => {
                let msg = format!("Failed to parse EF: {}", reason);
                return Err(msg);
            }
        };

        let item = ScheduleItem {
            iteration,
            ef,
            last_revisit,
            next_revisit,
        };

        Ok((id, item))
    }

    /// Returns number of days since last revisit.
    fn days_since(&self) -> u32 {
        let duration = self.last_revisit - today();
        let days = duration.num_days();
        if days < 0 {
            panic!("Item was visited in the future.");
        }
        days as u32
    }

    /// Marks `self` as visited today.
    fn revisit(&mut self) {
        self.last_revisit = today();
    }

    /// Schedule `self` n days to the future.
    fn reschedule(&mut self, days: u32) {
        self.next_revisit = today() + Duration::days(i64::from(days));
    }

    /// Recompute easiness factor based on user assessed easiness (0 - 5).
    fn update_ef(&mut self, q: u8) {
        if q > 5 {
            panic!("Invalid easiness assessment: {}.", q);
        }

        let q = f32::from(q);
        self.ef = self.ef - 0.8 + 0.28 * q - 0.02 * q * q;
        if self.ef < 1.3 {
            self.ef = 1.3;
        }
    }

    /// Reschedule, recompute easiness and reset iteration based on user
    /// provided easiness assessment.
    fn update(&mut self, q: u8) {
        self.update_ef(q);

        if q < 3 {
            self.iteration = 0;
        } else {
            self.iteration += 1;
            if self.iteration < 1 {
                panic!("Iteration cannot be smaller than 1 at this stage.");
            }
            if self.iteration == 1 {
                self.reschedule(1);
            } else if self.iteration == 2 {
                self.reschedule(6);
            } else {
                let interval: u32 = (self.ef * self.days_since() as f32) as u32;
                self.reschedule(interval);
            }
        }

        self.revisit();
    }
}

impl Default for Schedule {
    fn default() -> Schedule {
        Schedule {
            items: FnvHashMap::default(),
            stage: 0,
            hot_stage: VecDeque::new(),
            refresh_stage: VecDeque::new(),
        }
    }
}

impl Schedule {
    pub fn load() -> Result<Schedule, String> {
        let mut path = get_vole_dir()?;
        path.push(&SCHEDULE_FILE_NAME);

        let mut schedule: Schedule = Default::default();

        let file = match File::open(&path) {
            Ok(file) => file,
            Err(error) => {
                if let ErrorKind::NotFound = error.kind() {
                    return Ok(schedule);
                }
                let reason = format!(
                    "Couldn't open file \"{}\": {}",
                    path.to_string_lossy(),
                    error
                );
                return Err(reason);
            }
        };

        let reader = BufReader::new(&file);
        let numbered_lines = reader
            .lines()
            .enumerate()
            .map(|(i, result)| (i + 1, result));

        for (line_nr, result) in numbered_lines {
            let line = match result {
                Ok(line) => line,
                Err(error) => {
                    let reason = format!(
                        "Couldn't read file \"{}\": {}",
                        path.to_string_lossy(),
                        error
                    );
                    return Err(reason);
                }
            };

            let (id, item) = match ScheduleItem::deserialize(&line) {
                Ok(parsed) => parsed,
                Err(reason) => {
                    let msg = format!("Error on line {}: {}", line_nr, reason);
                    return Err(msg);
                }
            };
            if item.next_revisit <= today() {
                schedule.hot_stage.push_back(id);
            }
            schedule.items.insert(id, item);
        }

        Ok(schedule)
    }

    /// Saves schedule to disc and overwrites schedule file if it already
    /// exists.
    pub fn save(&self) -> Result<(), String> {
        let mut path = get_vole_dir()?;
        let mut tmp_path = path.clone();
        path.push(&SCHEDULE_FILE_NAME);
        tmp_path.push(format!("{}.tmp", &SCHEDULE_FILE_NAME));

        {
            let mut file = match File::create(&tmp_path) {
                Ok(file) => file,
                Err(error) => {
                    let reason = format!(
                        "Couldn't open file \"{}\": {}",
                        tmp_path.to_string_lossy(),
                        error
                    );
                    return Err(reason);
                }
            };

            for (id, item) in &self.items {
                let line = item.serialize(*id);
                if let Err(error) = file.write_all(line.as_bytes()) {
                    let reason = format!(
                        "Couldn't append to file \"{}\": {}",
                        tmp_path.to_string_lossy(),
                        error
                    );
                    return Err(reason);
                }
            }
        }

        if let Err(error) = rename(&tmp_path, &path) {
            let reason = format!(
                "Couldn't rename \"{}\" to \"{}\": {}",
                tmp_path.to_string_lossy(),
                path.to_string_lossy(),
                error
            );
            return Err(reason);
        }

        Ok(())
    }

    /// Returns true if item with given ID is already tracked in the schedule.
    pub fn has_item(&self, id: u64) -> bool {
        self.items.contains_key(&id)
    }

    /// Creates a new freshly initialized item to be learned.
    ///
    /// # Panics
    ///
    /// This method panics if the added item has been already added in the
    /// past.
    pub fn add_item(&mut self, id: u64) {
        if self.has_item(id) {
            panic!("Item with ID {} is already scheduled.", id);
        }

        self.hot_stage.push_back(id);
        let item: ScheduleItem = Default::default();
        self.items.insert(id, item);
    }

    /// Returns true if all items have been learned for today. New items to be
    /// learned can be added with `self.add_item()`.
    pub fn is_done(&self) -> bool {
        self.hot_stage.is_empty() && self.refresh_stage.is_empty()
    }

    /// Provides ID of the next item to be displayed and assessed. Call
    /// `self.update_current()` after the item is asses by the user.
    pub fn current(&self) -> u64 {
        if self.is_done() {
            panic!("No scheduled items.");
        }

        let current_stage = if self.stage == 0 {
            &self.hot_stage
        } else {
            &self.refresh_stage
        };

        *current_stage.front().unwrap()
    }

    /// Asses first item in the queue of items to be assessed and move to the
    /// next. Call `self.current()` to get the next item.
    pub fn update_current(&mut self, q: u8) {
        if self.is_done() {
            panic!("Unexpected update.");
        }

        let item_id = if self.stage == 0 {
            self.hot_stage.pop_front().unwrap()
        } else {
            self.refresh_stage.pop_front().unwrap()
        };

        let item = self.items.get_mut(&item_id).unwrap();

        if self.stage == 0 {
            item.update(q);
        }

        if q < 3 {
            self.hot_stage.push_back(item_id);
        } else if q == 3 {
            self.refresh_stage.push_back(item_id);
        }

        self.stage = 0;
        if self.hot_stage.is_empty() && !self.refresh_stage.is_empty() {
            self.stage = 1;
        }
    }
}
