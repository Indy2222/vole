// This file contains re-implementation of SuperMemo 2 algorithm. See
// http://supermemopedia.com/wiki/Licensing_SuperMemo_Algorithm
// for information about SM-2 algorithm licensing
//
// Algorithm SM-2, (C) Copyright SuperMemo World, 1991.
//
//    http://www.supermemo.com
//    http://www.supermemo.eu

use chrono::{Duration, NaiveDate, prelude::*};
use file::get_vole_dir;
use std::collections::{HashMap, VecDeque};
use std::fs::{rename, File};
use std::io::{Write};

const SCHEDULE_FILE_NAME: &str = "schedule.txt";

struct ScheduleItem {
    iteration: u32,
    ef: f32,
    last_revisit: NaiveDate,
    next_revisit: NaiveDate,
}

pub struct Schedule {
    items: HashMap<String, ScheduleItem>,
    stage: usize,
    hot_stage: VecDeque<String>,
    refresh_stage: VecDeque<String>,
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
    fn serialize(&self, id: &str) -> String {
        let last_revisit = self.last_revisit.format("%Y-%m-%d");
        let next_revisit = self.next_revisit.format("%Y-%m-%d");
        format!("{id}\t{next_revisit}\t{last_revisit}\t{iteration}\t{ef}\n",
                id=id,
                next_revisit=next_revisit,
                last_revisit=last_revisit,
                iteration=self.iteration,
                ef=self.ef)
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
        self.next_revisit = today() + Duration::days(days as i64);
    }

    /// Recompute easiness factor based on user assessed easiness (0 - 5).
    fn update_ef(&mut self, q: u8) {
        if q > 5 {
            panic!("Invalid easiness assessment: {}.", q);
        }

        let q = q as f32;
        self.ef = self.ef -0.8 + 0.28 * q -0.02 * q * q;
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

impl Schedule {
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
                    let reason = format!("Couldn't open file \"{}\": {}",
                                         tmp_path.to_string_lossy(), error);
                    return Err(reason);
                }
            };

            for (id, item) in &self.items {
                let line = item.serialize(id);
                if let Err(error) = file.write_all(line.as_bytes()) {
                    let reason = format!("Couldn't append to file \"{}\": {}",
                                         tmp_path.to_string_lossy(), error);
                    return Err(reason);
                }
            }
        }

        if let Err(error) = rename(&tmp_path, &path) {
            let reason = format!("Couldn't rename \"{}\" to \"{}\": {}",
                                 tmp_path.to_string_lossy(),
                                 path.to_string_lossy(), error);
            return Err(reason);
        }

        Ok(())
    }

    /// Returns true if item with given ID is already tracked in the schedule.
    pub fn has_item(&self, id: &str) -> bool {
        self.items.contains_key(id)
    }

    /// Creates a new freshly initialized item to be learned.
    ///
    /// # Panics
    ///
    /// This method panics if the added item has been already added in the
    /// past.
    pub fn add_item(&mut self, id: String) {
        if self.has_item(&id) {
            panic!("Item with ID {} is already scheduled.", id);
        }

        self.hot_stage.push_back(id.clone());
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
    pub fn current<'a>(&'a self) -> &'a str {
        if self.is_done() {
            panic!("No scheduled items.");
        }

        let current_stage = if self.stage == 0 {
            &self.hot_stage
        } else {
            &self.refresh_stage
        };

        current_stage.front().unwrap()
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
