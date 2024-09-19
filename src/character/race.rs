mod human;
mod elf;

use std::collections::HashSet;

use super::Character;
pub use human::SubHuman;
use elf::SubElf;

macro_rules! clear_ap {
    ($c:expr) => {
        $c.str = 0;
        $c.dex = 0;
        $c.con = 0;
        $c.int = 0;
        $c.wis = 0;
        $c.cha = 0;
        $c.available_race_ap = None;
        $c.usable_ability = HashSet::new();
    };
}

macro_rules! assign_race_ap {
    ($c:expr) => {
        if $c.used_ability.len() > 0 {
            if $c.used_ability.contains("str") {
                $c.str += 1;
            }
            if $c.used_ability.contains("dex") {
                $c.dex += 1;
            }
            if $c.used_ability.contains("con") {
                $c.con += 1;
            }
            if $c.used_ability.contains("int") {
                $c.int += 1;
            }
            if $c.used_ability.contains("wis") {
                $c.wis += 1;
            }
            if $c.used_ability.contains("cha") {
                $c.cha += 1;
            }
        }
    };
}

pub enum Race {
    None,
    Human(SubHuman),
    Elf(SubElf),
}

impl Race {
    fn get_ap(&self) -> [u8; 7] {
        match self {
            Self::None => [0, 0, 0, 0, 0, 0, 0],
            Self::Human(sub) => sub.get_ap(),
            Self::Elf(_) => [0, 0, 0, 0, 0, 0, 0]
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

    pub fn init_ap(c: &mut Character) {
        clear_ap!(c);
        let ap: [u8; 7] = c.race.get_ap();
        let usable_ability = Self::get_usable_ability(ap);
        assign_race_ap!(c);
        c.str += ap[0];
        c.dex += ap[1];
        c.con += ap[2];
        c.int += ap[3];
        c.wis += ap[4];
        c.cha += ap[5];
        c.available_race_ap = Some(ap[6]-usize_to_u8(c.used_ability.len()));
        c.usable_ability = usable_ability;
    }
}

fn usize_to_u8(num: usize) -> u8 {
    let val: u8 = num.try_into().unwrap();
    val
}

