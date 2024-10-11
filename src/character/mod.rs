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

use crate::SubRace;

macro_rules! check_edition_5e {
    ($self:expr) => {
        if !$self.edition.is_5e() {
            return false;
        }
    };
}

pub enum Edition {
    FifthEdition,
    DnDOne
}

impl Edition {
    fn is_5e(&self) -> bool {
        match self {
            Self::FifthEdition => true,
            _ => false
        }
    }
}

pub struct Character {
    edition: Edition,
    race: Race,
    class: class::Class,
    background: Background,
    usable_ability: HashSet<String>,
    used_ability: HashSet<String>,
    used_lang: HashSet<String>,
    base_ap: Option<[u8; 6]>,
    additional_race_ap: Option<u8>,
    additoinal_ap: Option<u8>,
    ability_scores: [u8; 6],
    profeciency: HashMap<String, HashSet<String>>,
    lang_point: Option<u8>,
    feat_point: u8,
    feat: Feat,
    speed: u8,
    size: String,
    buffer: Option<SubRace>
}

impl Character {
    pub fn new_5e() -> Character {
        let empty_prof: HashMap<String, HashSet<String>> = HashMap::new();
        Character {
            edition: Edition::FifthEdition,
            race: Race::Undefined,
            class: class::Class::None,
            background: Background::None,
            usable_ability: HashSet::new(),
            used_ability: HashSet::new(),
            used_lang: HashSet::new(),
            base_ap: None, // Base ap from dice rolls or point buy
            additional_race_ap: None, // Usable points from race
            additoinal_ap: None, // Extra usable points
            ability_scores: [0,0,0,0,0,0], // Total ability score at the end
            profeciency: empty_prof,
            lang_point: None,
            feat_point: 0,
            feat: Feat::new(),
            speed: 0,
            size: "Unknown".to_string(),
            buffer: None
        }
    }

    pub fn print_stat(&self) {
        if self.additional_race_ap != None {
            println!("[STR: {}],[DEX: {}],[CON: {}],[INT: {}],[WIS: {}],[CHA: {}]",
            self.ability_scores[0], self.ability_scores[1], self.ability_scores[2],
            self.ability_scores[3], self.ability_scores[4], self.ability_scores[5]);
            if self.additional_race_ap.unwrap() > 0 {
                println!("Remaining Points: {}", self.additional_race_ap.unwrap());
            }
            let languages = self.profeciency.get("Language");
            let weapons = self.profeciency.get("Weapon");
            let armors = self.profeciency.get("Armor");
            let skills = self.profeciency.get("Skill");
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
            if weapons != None {
                print!("Weapons: ");
                if weapons.unwrap().len() != 0 {
                    for weapon in weapons.unwrap() {
                        print!("{} ", weapon);
                    }
                } else {
                    print!("None");
                }
            }
            print!("\n");
            if armors != None {
                print!("Armors: ");
                if armors.unwrap().len() != 0 {
                    for armor in armors.unwrap() {
                        print!("{} ", armor);
                    }
                } else {
                    print!("None");
                }
            }
            print!("\n");
            if skills != None {
                print!("Skills: ");
                if skills.unwrap().len() != 0 {
                    for skill in skills.unwrap() {
                        print!("{} ", skill);
                    }
                } else {
                    print!("None");
                }
            }
            print!("\n");
            println!("Speed: {}", self.speed);
            println!("Size: {}\n", self.size);
        }
    }

    pub fn select_race_5e(&mut self, new_race: Race) -> bool {
        check_edition_5e!(self);
        // Clear used AP
        self.race = new_race;
        self.used_ability = HashSet::new();
        Race::init_buffer(self);
        Race::init_ap(self);
        Race::init_prof(self);
        true
    }

    // Return true only if succesffuly added point
    pub fn use_race_ap_5e(&mut self, ap: &str) -> bool {
        check_edition_5e!(self);
        let mut check: bool = false;
        if self.additional_race_ap != None {
            let points = self.additional_race_ap.unwrap();
            if (points > 0) & (self.usable_ability.contains(ap)) {
                if self.used_ability.insert(ap.to_string()) {
                    check = true;
                }
            }
        }
        Race::init_ap(self);
        check
    }

    // Return true only if successfully remove point
    pub fn remove_race_ap_5e(&mut self, ap: &str) -> bool {
        check_edition_5e!(self);
        if self.used_ability.remove(ap) {
            Race::init_ap(self);
            return true;
        }
        false
    }

    pub fn clear_race_ap_5e(&mut self) -> bool {
        check_edition_5e!(self);
        self.used_ability = HashSet::new();
        Race::init_ap(self);
        true
    }

    // Return true only if succesffuly added point
    pub fn use_lang_point_5e(&mut self, lang: &str) -> bool {
        check_edition_5e!(self);
        if self.lang_point != None {
            let point = self.lang_point.unwrap();
            if point > 0 {
                let lang_handle = self.profeciency.get_mut("Language").unwrap();
                if !self.used_lang.contains(lang) {
                    if lang_handle.insert(lang.to_string()) {
                        self.used_lang.insert(lang.to_string());
                        self.lang_point = Some(point-1);
                        return true;
                    }
                    
                }
            }
        }
        false
    }

    // Return true only if successfully remove point
    pub fn remove_lang_5e(&mut self, lang: &str) -> bool {
        check_edition_5e!(self);
        if self.used_lang.contains(lang) {
            let lang_handle = self.profeciency.get_mut("Language").unwrap();
            if lang_handle.remove(lang) {
                self.lang_point = Some(self.lang_point.unwrap()+1);
                return true;
            }
        }
        false
    }

    pub fn clear_lang_5e(&mut self) -> bool {
        check_edition_5e!(self);
        Race::init_lang(self);
        true
    }

    pub fn select_class_5e(&mut self) -> bool {
        check_edition_5e!(self);
        false
    }
}
