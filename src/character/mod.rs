#![allow(dead_code)]
pub mod race;
mod profeciency;
mod class;
mod feat;
mod background;

use std::collections::{HashMap, HashSet};

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
    used_lang: HashSet<String>,
    base_ap: Option<[u8; 6]>,
    additional_race_ap: Option<u8>,
    additoinal_ap: Option<u8>,
    ability_scores: HashMap<String, u8>,
    profeciency: HashMap<String, HashSet<String>>,
    lang_point: Option<u8>,
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
            used_lang: HashSet::new(),
            base_ap: None, // Base ap from dice rolls
            additional_race_ap: None, // Usable points from race
            additoinal_ap: None, // Extra usable points
            ability_scores: empty_ap, // Total ability score at the end
            profeciency: empty_prof,
            lang_point: None,
            feat_point: 0,
            feat: Feat::new()
        }
    }

    pub fn print_stat(&self) {
        if self.additional_race_ap != None {
            println!("[STR: {}],[DEX: {}],[CON: {}],[INT: {}],[WIS: {}],[CHA: {}]",
            &self.ability_scores.get("str").unwrap(), &self.ability_scores.get("dex").unwrap(),
            &self.ability_scores.get("con").unwrap(), &self.ability_scores.get("int").unwrap(),
            &self.ability_scores.get("wis").unwrap(), &self.ability_scores.get("cha").unwrap());
            println!("Remaining Points: {}", self.additional_race_ap.unwrap());
            let languages = self.profeciency.get("Language");
            if languages != None {
                print!("Language: ");
                for lang in languages.unwrap() {
                    print!("{} ", lang);
                }
                print!("\n")
            }
            if self.lang_point != None {
                println!("Langauge Point: {}", self.lang_point.unwrap());
            }
            print!("\n")
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
            if self.additional_race_ap != None {
                let points = self.additional_race_ap.unwrap();
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

    pub fn use_lang_point_5e(&mut self, lang: &str) {
        if self.lang_point != None {
            let point = self.lang_point.unwrap();
            if point > 0 {
                let lang_handler = self.profeciency.get_mut("Language").unwrap();
                if (!lang_handler.contains(lang)) & (!self.used_lang.contains(lang)) {
                    lang_handler.insert(lang.to_string());
                    self.used_lang.insert(lang.to_string());
                    self.lang_point = Some(point-1);
                }
            }
        }
    }

    pub fn remove_lang_5e(&mut self, lang: &str) {
        if self.used_lang.contains(lang) {
            let lang_handler = self.profeciency.get_mut("Language").unwrap();
            if lang_handler.remove(lang) {
                self.lang_point = Some(self.lang_point.unwrap()+1);
            }
        }
    }

    pub fn clear_lang_5e(&mut self) {
        Race::init_lang(self);
    }
}
