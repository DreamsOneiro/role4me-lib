pub mod subrace;

use core::panic;
use std::collections::HashSet;
use super::Character;
pub use subrace::*;
pub use Race::*;


macro_rules! race_match_base {
    ( $subject:expr, $none:expr, $method:ident, $($race:tt),* ) => {
        match $subject {
            Self::Undefined => $none,
        $(
            Self::$race(sub) => sub.$method(),
        )*
        }
    };
}

macro_rules! race_match_all {
    ($subject:expr, $none:expr, $method:ident) => {
        race_match_base!($subject, $none, $method,
            Human, Elf, Dwarf
        )
    };
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Race {
    Undefined,
    Human(SubHuman),
    Elf(SubElf),
    Dwarf(SubDwarf),
}

impl Race {
// ----------------------
// |   Public Funtion   |
// ----------------------
    pub fn init_buffer(c: &mut Character) {
        c.buffer = c.race.get_buff();
    }

    pub fn init_ap(c: &mut Character) {
        let race_ap = c.buffer.as_mut().unwrap().get_ap();
        // Clear all pointj
        c.ability_scores = [0,0,0,0,0,0];
        // Assign points from race
        for i in 0..6 {
            c.ability_scores[i] = race_ap[i];
        }
        // Assign points from manual select
        c.usable_ability = Self::get_usable_ability(race_ap);
        c.additional_race_ap = Some(race_ap[6]-usize_to_u8(c.used_ability.len()));
        for val in &c.used_ability {
            let index = Self::get_usable_abiltiy_index(val);
            c.ability_scores[index] += 1;
        }
    }

    pub fn init_prof(c: &mut Character) {
        Self::init_lang(c);
        Self::init_weap(c);
        Self::init_armor(c);
        Self::init_skill(c);
        Self::init_speed(c);
        Self::init_size(c);
    }

    pub fn init_lang(c: &mut Character) {
        // Clear all language
        c.profeciency.insert("Language".to_string(), HashSet::new());
        c.used_lang = HashSet::new();
        c.lang_point = None;
        // Get Language Profeciency
        let race_lang = c.buffer.as_mut().unwrap().get_lang();
        let handle = c.profeciency.get_mut("Language").unwrap();
        for lang in &race_lang {
            handle.insert(lang.to_string());
        }
        let point = c.buffer.as_mut().unwrap().get_lang_point();
        if point > 0 {
            c.lang_point = Some(point);
        }
    }

    pub fn init_weap(c: &mut Character) {
        // Clear all weapons
        c.profeciency.insert("Weapon".to_string(), HashSet::new());
        // Get Weapon Profeciency
        let race_weap = c.buffer.as_mut().unwrap().get_weap();
        let handle = c.profeciency.get_mut("Weapon").unwrap();
        for weap in race_weap {
            handle.insert(weap.to_string());
        }
    }

    pub fn init_armor(c: &mut Character) {
        // Clear all armors
        c.profeciency.insert("Armor".to_string(), HashSet::new());
        // Get Armor Profeciency
        let race_armor = c.buffer.as_mut().unwrap().get_armor();
        let handle = c.profeciency.get_mut("Armor").unwrap();
        for armor in race_armor {
            handle.insert(armor.to_string());
        }
    }

    pub fn init_skill(c: &mut Character) {
        // Clear all skills
        c.profeciency.insert("Skill".to_string(), HashSet::new());
        // Get Skill Profeciencies
        let race_skill = c.buffer.as_mut().unwrap().get_skill();
        let handle = c.profeciency.get_mut("Skill").unwrap();
        for skill in race_skill {
            handle.insert(skill.to_string());
        }
    }

    pub fn init_speed(c: &mut Character) {
        c.speed = c.buffer.as_mut().unwrap().get_speed();
    }

    pub fn init_size(c: &mut Character) {
        c.size = c.buffer.as_mut().unwrap().get_size().to_string();
    }

// -----------------------
// |   Private Funtion   |
// -----------------------
    fn get_buff(&self) -> Option<SubRace> {
        race_match_all!(self, None, handle)
    }

    // Return a list of abilities allowed to increase by 1 point
    fn get_usable_ability(ap: [u8; 7]) -> HashSet<String> {
        let abilities: [&str; 6] = ["str", "dex", "con", "int", "wis", "cha"];
        let mut available_stat: HashSet<String> = HashSet::new();
        for (i, val) in ap.iter().enumerate() {
            if (*val == 0) & (i < 6) {
                available_stat.insert(abilities[i].to_string());
            }
        }
        available_stat
    }

    fn get_usable_abiltiy_index(ap: &str) -> usize {
        let abilities = [
            "str", "dex", "con",
            "int", "wis", "cha"
        ];
        for (i, ability) in abilities.iter().enumerate() {
            if ap.to_lowercase() == *ability {
                return i;
            }
        }
        panic!();
    }
}


fn usize_to_u8(num: usize) -> u8 {
    let val: u8 = num.try_into().unwrap();
    val
}
