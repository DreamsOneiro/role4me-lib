#![allow(dead_code)]
pub mod class;
pub mod traits;
pub mod race;

use crate::common::Edition;
use class::Class;
use race::{Race, Races};

pub struct Character<'a> {
    edition: Edition,
    class: Class,
    race: Race<'a>,
    base_ap: [usize; 6],
}

impl<'a> Character<'a> {
    pub fn new() -> Character<'a> {
        Character {
            edition: Edition::FifithEdition,
            class: Class::Undefined,
            race: Race::new(),
            base_ap: [0,0,0,0,0,0]
        }
    }

    pub fn race_select<T: Races + 'a>(&mut self, race: T) {
        self.race.race_change(race);
    }

    pub fn dbg(&self) {
        println!("{:?}", &self.race);
    }
}
