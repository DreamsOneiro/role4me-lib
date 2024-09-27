pub mod subrace;

use std::collections::HashSet;
use super::Character;
pub use subrace::*;
pub use Race::*;


macro_rules! race_match_base {
    ( $subject:expr, $none:expr, $method:ident, $($race:tt),* ) => {
        match $subject {
            Self::None => $none,
        $(
            Self::$race(sub) => sub.$method(),
        )*
        }
    };
}

macro_rules! race_match_all {
    ($subject:expr, $none:expr, $method:ident) => {
        race_match_base!($subject, $none, $method,
            Human, Elf
        )
    };
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Race {
    None,
    Human(SubHuman),
    Elf(SubElf),
}

impl Race {
// ----------------------
// |   Public Funtion   |
// ----------------------
    pub fn init_ap(c: &mut Character) {
        let abilities: [&str; 6] = ["str", "dex", "con", "int", "wis", "cha"];
        let ap = c.race.get_ap();
        // Clear all points
        for val in abilities {
            let score = c.ability_scores.get_mut(val).unwrap();
            *score = 0;
        }
        // Assign points from race
        for (i, val) in abilities.iter().enumerate() {
            let score = c.ability_scores.get_mut(*val).unwrap();
            *score += ap[i];
        }
        // Assign points from manual select
        c.usable_ability = Self::get_usable_ability(ap);
        c.additional_race_ap = Some(ap[6]-usize_to_u8(c.used_ability.len()));
        for val in &c.used_ability {
            let score = c.ability_scores.get_mut(val).unwrap();
            *score += 1;
        }
    }

    pub fn init_prof(c: &mut Character) {
        Self::init_lang(c);
    }

    pub fn init_lang(c: &mut Character) {
        // Clear all language
        c.profeciency.insert("Language".to_string(), HashSet::new());
        c.used_lang = HashSet::new();
        // Get Language Profeciency
        let race_lang = c.race.get_language();
        let handler = c.profeciency.get_mut("Language").unwrap();
        for lang in &race_lang {
            handler.insert(lang.to_string());
        }
        let point = c.race.get_lang_point();
        if point > 0 {
            c.lang_point = Some(point);
        }
    }

// -----------------------
// |   Private Funtion   |
// -----------------------
    fn get_language(&self) -> Vec<&str> {
        race_match_all!(self, Vec::new(), get_language)
    }

    fn get_ap(&self) -> [u8; 7] {
        race_match_all!(self, [0,0,0,0,0,0,0], get_ap)
    }
    
    fn get_lang_point(&self) -> u8 {
        race_match_all!(self, 0, get_lang_point)
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
}


fn usize_to_u8(num: usize) -> u8 {
    let val: u8 = num.try_into().unwrap();
    val
}
