#![allow(dead_code)]
pub mod race;
mod profeciency;
mod class;
mod feat;
mod background;
mod ability;

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
    pub race: Race,
    pub class: class::Class,
    pub background: Background,
    pub usable_ability: HashSet<String>,
    used_ability: HashSet<String>,
    used_lang: HashSet<String>,
    pub base_ap: Option<[u8; 6]>, // Base ap from dice rolls or point buy
    pub additional_race_ap: Option<u8>, // Usable points from race
    pub unassigned_base_ap: [u8; 6], // Store stats from rolls or point buy
    pub additoinal_ap: Option<u8>, // Usable points from feat
    pub ability_scores: [u8; 6], // Total ability score at the end
    pub profeciency: HashMap<String, HashSet<String>>,
    pub lang_point: Option<u8>,
    pub feat_point: u8,
    pub feat: Feat,
    pub speed: u8,
    pub size: String,
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
            base_ap: None,
            additional_race_ap: None,
            unassigned_base_ap: [0,0,0,0,0,0],
            additoinal_ap: None,
            ability_scores: [0,0,0,0,0,0],
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

    pub fn roll_stats_5e(&mut self) -> bool {
        check_edition_5e!(self);
        ability::roll_dice(self);
        true
    }

    pub fn use_standard_array_5e(&mut self) -> bool {
        check_edition_5e!(self);
        self.unassigned_base_ap = [15,14,13,12,10,8];
        true
    }

    pub fn use_heroic_array_5e(&mut self) -> bool {
        check_edition_5e!(self);
        self.unassigned_base_ap = [17,16,15,14,12,10];
        true
    }

    // Pass in array for points, valued between 0 to 9
    // 6 & 8 are not allowed (refer to rule book)
    // i.e. [1,5,7,9,3,2] 
    pub fn use_point_buy_5e(&mut self, points: [u8; 6]) -> bool {
        // Remove points created from other method
        self.unassigned_base_ap = [0,0,0,0,0,0];
        let mut sum: u8 = 0;
        for point in points {
            if (point > 9) || (point == 6) || (point == 8) {return false;}
            sum += point;
        } if sum > 27 {
            return false;
        }
        self.base_ap = Some(ability::calculate_point_buy(points));
        true
    }

    pub fn init_base_point(&mut self) {
        for i in 0..6 {
            self.ability_scores[i] += self.base_ap.unwrap()[i];
        }
    }

    // Pass in sequence to assign ap from 1 to 6
    // i.e [2,5,1,3,4,6]
    pub fn assign_rolls_5e(&mut self, rolls: [u8; 6]) -> bool {
        check_edition_5e!(self);
        if self.unassigned_base_ap[0] != 0 {
            let mut rolls_set: HashSet<u8> = HashSet::new();
            for i in rolls {
                if (i == 0) || (i > 6) {
                    return false;
                }
                rolls_set.insert(i);
            }
            if rolls_set.len() == 6 {
                let i1: usize = rolls[0].into();
                let i2: usize = rolls[1].into();
                let i3: usize = rolls[2].into();
                let i4: usize = rolls[3].into();
                let i5: usize = rolls[4].into();
                let i6: usize = rolls[5].into();
                self.base_ap = Some([
                    self.unassigned_base_ap[i1-1],
                    self.unassigned_base_ap[i2-1],
                    self.unassigned_base_ap[i3-1],
                    self.unassigned_base_ap[i4-1],
                    self.unassigned_base_ap[i5-1],
                    self.unassigned_base_ap[i6-1],
                ]);
                return true;
            }
        }
        false
    }
}
