mod human;
mod elf;

use std::collections::HashSet;

use super::Character;
pub use human::SubHuman;
pub use elf::SubElf;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Race {
    None,
    Human(SubHuman),
    Elf(SubElf),
}

impl Race {
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
        c.additional_ap = Some(ap[6]-usize_to_u8(c.used_ability.len()));
        for val in &c.used_ability {
            let score = c.ability_scores.get_mut(val).unwrap();
            *score += 1;
        }
    }

    pub fn init_prof(c: &mut Character) {
        let race_lang: Vec<&str> = match c.race {
            Self::Human(sub) => sub.get_language(),
            _ => Vec::new()
        };
        if c.profeciency.get_mut("Language") == None {
            c.profeciency.insert(String::from("Language"), HashSet::new());
        }
        let language = c.profeciency.get_mut("Language").unwrap();
        for lang in &race_lang {
            language.insert(lang.to_string());
        }
    }

    fn get_ap(&self) -> [u8; 7] {
        match self {
            Self::None => [0, 0, 0, 0, 0, 0, 0],
            Self::Human(sub) => sub.get_ap(),
            Self::Elf(sub) => sub.get_ap()
        }
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

