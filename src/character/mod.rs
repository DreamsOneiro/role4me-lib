#![allow(dead_code)]
pub mod race;
mod profeciency;
mod class;
mod feat;
mod background;

use std::collections::{HashMap, HashSet};

pub use profeciency::Profeciency;
pub use race::Race;
pub use feat::Feat;
pub use background::Background;

pub struct Character {
    edition: String,
    race: Race,
    class: class::Class,
    background: Background,
    usable_ability: HashSet<String>,
    used_ability: HashSet<String>,
    base_ap: Option<[u8; 6]>,
    additional_ap: Option<u8>,
    ability_scores: HashMap<String, u8>,
    profeciency: HashMap<String, HashSet<String>>,
    feat_point: u8,
    feat: Feat
}

impl Character {
    pub fn new_5e() -> Character {
        let mut empty_ap: HashMap<String, u8> = HashMap::new();
        empty_ap.insert(String::from("str"), 0);
        empty_ap.insert(String::from("dex"), 0);
        empty_ap.insert(String::from("con"), 0);
        empty_ap.insert(String::from("int"), 0);
        empty_ap.insert(String::from("wis"), 0);
        empty_ap.insert(String::from("cha"), 0);
        let mut empty_prof: HashMap<String, HashSet<String>> = HashMap::new();
        empty_prof.insert(String::from("Language"), HashSet::new());
        Character {
            edition: String::from("5e"),
            race: Race::None,
            class: class::Class::None,
            background: Background::None,
            usable_ability: HashSet::new(),
            used_ability: HashSet::new(),
            base_ap: None, // Base ap from dice rolls
            additional_ap: None, // Usable points
            ability_scores: empty_ap, // Total ability score at the end
            profeciency: empty_prof,
            feat_point: 0,
            feat: Feat::new()
        }
    }

    pub fn print_stat(&self) {
        if self.additional_ap != None {
            for (key, value) in &self.ability_scores {
                print!("[{}: {}]", key, value);
            }
            println!("\nRemaining Points: {}", self.additional_ap.unwrap());
            let languages = self.profeciency.get("Language");
            if languages != None {
                print!("Language: ");
                for lang in languages.unwrap() {
                    print!("{} ", lang);
                }
                print!("\n")
            }
        }
    }

    pub fn select_race_5e(&mut self, new_race: Race) {
        if self.edition == "5e" {
            // Clear used AP
            self.used_ability = HashSet::new();
            self.race = new_race;
            Race::init_ap(self);
            Race::init_prof(self);
        }
    }

    pub fn use_race_ap_5e(&mut self, ap: &str) {
        if self.edition == "5e" {
            if self.additional_ap != None {
                let points = self.additional_ap.unwrap();
                if (points > 0) & (self.usable_ability.contains(ap)) {
                    self.used_ability.insert(ap.to_string());
                }
            }
            Race::init_ap(self);
        }
    }

    pub fn remove_race_ap_5e(&mut self, ap: &str) {
        if self.edition == "5e" {
            if self.used_ability.remove(ap) {
                Race::init_ap(self);
            }
        }
    }

    pub fn clear_race_ap_5e(&mut self) {
        if self.edition == "5e" {
            self.used_ability = HashSet::new();
            Race::init_ap(self);
        }
    }
}
