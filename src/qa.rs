use card::Card;
use file::CardsReader;
use scheduler::Schedule;
use std::collections::{HashMap, VecDeque};

pub struct Qa {
    queued: VecDeque<Card>, // Cards yet to be scheduled
    scheduled: HashMap<String, Card>,
    schedule: Schedule,
}


/// Question & Answer object consist of these parts:
///
/// * A spaced repetition learning scheduler. User can:
///   * get (and display) current card
///   * assess difficulty of current card which reschedules the card and moves
///     to the next card
/// * FIFO queue of cards not yet schedule (i.e. learned). User can put more
///   cards to scheduler.
impl Qa {
    /// Initialize Question & Answer object from cards iterator. Schedule is
    /// loaded from disk.
    pub fn load(reader: CardsReader) -> Result<Qa, String> {
        let schedule = Schedule::load()?;

        let mut qa = Qa {
            queued: VecDeque::new(),
            scheduled: HashMap::new(),
            schedule: schedule,
        };

        for card_result in reader {
            let card: Card = card_result?;

            if qa.schedule.has_item(card.id()) {
                qa.scheduled.insert(card.id().to_string(), card);
            } else {
                qa.queued.push_back(card);
            }
        }

        Ok(qa)
    }

    /// Save schedule to disk. Exiting schedule file is rewritten.
    pub fn save(&self) -> Result<(), String> {
        self.schedule.save()
    }

    /// Returns true if all cards scheduled for today has been learned.
    pub fn is_today_schedule_done(&self) -> bool {
        self.schedule.is_done()
    }

    /// Returns true if there is at least one card not yet scheduled.
    pub fn is_all_scheduled(&self) -> bool {
        return self.queued.is_empty()
    }

    /// Schedule `count` new cards for learning.
    pub fn schedule_more(&mut self, count: usize) {
        for _i in 0..count {
            let card: Card = match self.queued.pop_front() {
                Some(card) => card,
                None => break,
            };

            self.schedule.add_item(card.id().to_string());
            self.scheduled.insert(card.id().to_string(), card);
        }
    }

    /// Get "current" card.
    pub fn current_card<'a>(&'a self) -> &'a Card {
        let item_id = self.schedule.current();
        self.scheduled.get(item_id).unwrap()
    }

    /// Assess "easiness" of current card and move current the next one.
    /// Easiness spans from 0 to 5.
    pub fn assess_current(&mut self, q: u8) {
        self.schedule.update_current(q);
    }
}
