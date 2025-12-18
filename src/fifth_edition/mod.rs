#![allow(dead_code)]
pub mod class;
pub mod race;
pub mod background;
pub mod feature;

use crate::{common::Edition};
use class::{Class, Classes};
use race::{Race, Races};
use background::{Background, Backgrounds};

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
            bg: Background::new(),
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

    pub fn bg_select(&mut self, bg: Backgrounds) {
        self.bg.change_bg(bg);
    }

    pub fn bg_reset(&mut self) {
        self.bg.reset();
    }

    pub fn dbg(&self) {
        println!("{:?}\n", &self.race);
        println!("{:?}\n", &self.class);
        println!("{:?}\n", &self.bg);
        println!("-----------------------------------");
    }
}
