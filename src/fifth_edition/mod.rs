#![allow(dead_code)]
pub mod class;
pub mod traits;
pub mod race;
pub mod background;

use crate::common::Edition;
use class::{Class, Classes};
use race::{Race, Races};
use background::Background;

pub struct Character {
    edition: Edition,
    class: Class,
    race: Race,
    bg: Background,
    base_ap: Option<[usize; 6]>,
}

impl Character {
    pub fn new() -> Character {
        Character {
            edition: Edition::FifithEdition,
            class: Class::new(),
            race: Race::new(),
            bg: Background::Undefined,
            base_ap: None
        }
    }

    pub fn class_select(&mut self, class: Classes) {
        self.class.change_class(class);
    }

    pub fn class_reset(&mut self) {
        self.class.reset();
    }

    pub fn race_select<T: Races + 'static>(&mut self, race: T) {
        self.race.change_race(race);
    }
    
    pub fn race_reset(&mut self) {
        self.race.reset();
    }

    pub fn dbg(&self) {
        println!("{:?}\n", &self.race);
        println!("{:?}\n", &self.class);
        println!("-----------------------------------");
    }
}
