#![allow(dead_code)]
pub mod class;
pub mod traits;
pub mod race;

use crate::common::Edition;
use class::Class;
use race::{Race, Races};

pub struct Character {
    edition: Edition,
    class: Class,
    race: Race,
    base_ap: [usize; 6],
}

impl Character {
    pub fn new() -> Character {
        Character {
            edition: Edition::FifithEdition,
            class: Class::Undefined,
            race: Race::new(),
            base_ap: [0,0,0,0,0,0]
        }
    }

    pub fn race_select<T: Races + 'static>(mut self, race: T) -> Character {
        self.race.race_change(race);
        self
    }

    pub fn dbg(self) -> Character {
        println!("{:?}", &self.race);
        self
    }
}
