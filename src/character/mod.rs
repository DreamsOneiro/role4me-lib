#![allow(dead_code)]
pub mod race;
mod profeciency;
mod class;
mod feat;
mod background;

use std::collections::HashSet;

pub use profeciency::Profeciency;
pub use race::Race;
pub use feat::Feat;
pub use background::Background;

pub struct Character {
    race: Race,
    class: class::Class,
    background: Background,
    usable_ability: HashSet<String>,
    used_ability: HashSet<String>,
    base_ap: Option<[u8; 6]>,
    available_race_ap: Option<u8>,
    str: u8,
    dex: u8,
    con: u8,
    int: u8,
    wis: u8,
    cha: u8,
    profeciency: Profeciency,
    feat_point: u8,
    feat: Feat
}

impl Character {
    pub fn new() -> Character {
        Character {
            race: Race::None,
            class: class::Class::None,
            background: Background::None,
            usable_ability: HashSet::new(),
            used_ability: HashSet::new(),
            base_ap: None,
            available_race_ap: None,
            str: 0,
            dex: 0,
            con: 0,
            int: 0,
            wis: 0,
            cha: 0,
            profeciency: Profeciency::new(),
            feat_point: 0,
            feat: Feat::new()
        }
    }

    pub fn print_stat(&self) {
        println!("Str: {}, Dex: {}, Con: {}, Int: {}, Wis: {}, Cha: {}",
            self.str,
            self.dex,
            self.con,
            self.int,
            self.wis,
            self.cha);
        if self.available_race_ap != None {
            println!("Remaining Points: {}", self.available_race_ap.unwrap());
        }
    }

    pub fn select_race_5e(&mut self, race: race::Race) {
        self.remove_all_race_ap_5e();
        self.race = race;
        Race::init_ap(self);
    }

    pub fn use_race_ap_5e(&mut self, ap: &str) {
        if self.available_race_ap != None {
            let ap_lower = ap.to_lowercase();
            let points = self.available_race_ap.unwrap();
            if (points > 0) & (self.usable_ability.contains(&ap_lower)) {
                self.used_ability.insert(ap_lower);
            }
        }
        Race::init_ap(self);
    }

    pub fn remove_race_ap_5e(&mut self, ap: &str) {
        if self.available_race_ap != None {
            let ap_lower = ap.to_lowercase();
            if self.usable_ability.contains(&ap_lower) {
                self.used_ability.remove(&ap_lower);
            }
        }
        Race::init_ap(self);
    }

    pub fn remove_all_race_ap_5e(&mut self) {
        self.used_ability = HashSet::new();
        Race::init_ap(self);
    }
}
