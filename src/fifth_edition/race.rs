use crate::common::{Size, first_letter_uppercase};
use crate::common::profeciency::{self, Armor, Language::*, Skill::*, Weapon::*};
use std::fmt;

/* ---------
   | Macro |
   --------- */
macro_rules! new_race {
    ($name:ident, $($sub_name:ident{
        fixed_ap: $fixed_ap:expr,
        free_ap: $free_ap:expr,
        lp: $lang_point:expr, 
        lang: $lang:expr, 
        weap: $weap:expr,
        armor: $armor:expr,
        skill: $skill:expr,
        speed: $speed:expr,
        size: $size:expr})*) => {

        pub enum $name {
            $(
                $sub_name,
            )*
        }

        impl Races for $name {
            fn as_string(&self) -> String {
                match self {
                    $(
                        Self::$sub_name => {
                            let s1 = stringify!($name);
                            let s1 = first_letter_uppercase(s1);
                            let s2 = stringify!($sub_name);
                            let s2 = first_letter_uppercase(&s2);
                            String::from(format!("{s1}({s2})"))
                        }
                    )*
                }
            }

            fn get_stat(&self) -> Race {
                match self {
                    $(
                        Self::$sub_name => Race {
                            race: Box::new(Self::$sub_name),
                            fixed_ap: $fixed_ap, 
                            free_ap: $free_ap,
                            lp: $lang_point,
                            lang: $lang,
                            weap: $weap,
                            armor: $armor,
                            skill: $skill,
                            speed: $speed,
                            size: $size,
                        },
                    )*
                }
            }
        }
    }
}


/* ---------
   | Trait |
   --------- */
pub trait Races {
    fn as_string(&self) -> String;
    fn get_stat(&self) -> Race;
}


/* ----------
   | Struct |
   ---------- */
pub struct Race {
    race: Box<dyn Races>,
    fixed_ap: [u8; 6],
    free_ap: u8,
    lp: u8,
    lang: Vec<profeciency::Language>,
    weap: Vec<profeciency::Weapon>,
    armor: Vec<profeciency::Armor>,
    skill: Vec<profeciency::Skill>,
    speed: u8,
    size: Size
}

impl Race {
    pub fn new() -> Self {
        Race {
            race: Box::new(Unknown::Unknown),
            fixed_ap: [0,0,0,0,0,0],
            free_ap: 0,
            lp: 0,
            lang: vec![],
            weap: vec![],
            armor: vec![],
            skill: vec![],
            speed: 0,
            size: Size::Undefined
        }
    }

    pub fn reset(&mut self) {
        self.race = Box::new(Unknown::Unknown);
        self.stat_regen();
    }

    pub fn change_race<T: Races + 'static>(&mut self, new_race: T) {
        self.race = Box::new(new_race);
        self.reset();
    }

    pub fn use_lp(&mut self, lang: profeciency::Language) -> bool {
        if self.lp > 0 && !self.lang.contains(&lang) {
            self.lang.push(lang.clone());
            self.lp -= 1;
            true
        }
        else {
            false
        }
    }

    fn stat_regen(&mut self) {
        *self = self.race.get_stat();
    }
}

impl fmt::Debug for Race {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Race")
            .field("\nRace", &self.race.as_string())
            .field("\nAP", &self.fixed_ap)
            .field("\nAssignable AP", &self.free_ap)
            .field("\nLaguage Point", &self.lp)
            .field("\nLanguage", &self.lang)
            .field("\nWeapon", &self.weap)
            .field("\nArmor", &self.armor)
            .field("\nSkill", &self.skill)
            .field("\nSpeed", &self.speed)
            .finish()
    }
}

/* ---------
   | Races |
   --------- */
// No Race
new_race!(Unknown,
    Unknown {
        fixed_ap: [0,0,0,0,0,0],
        free_ap: 0,
        lp: 0,
        lang: vec![],
        weap: vec![],
        armor: vec![],
        skill: vec![],
        speed: 0,
        size: Size::Undefined
    }
);

// Human
new_race!(Human,
    Basic {
        fixed_ap: [1,1,1,1,1,1],
        free_ap: 0,
        lp: 1,
        lang: vec![Common],
        weap: vec![],
        armor: vec![],
        skill: vec![],
        speed: 30,
        size: Size::Medium
    }
    Variant {
        fixed_ap: [0,0,0,0,0,0],
        free_ap: 2,
        lp: 1,
        lang: vec![Common],
        weap: vec![],
        armor: vec![],
        skill: vec![],
        speed: 30,
        size: Size::Medium
    }
);

// Elf
new_race!(Elf,
    Drow {
        fixed_ap: [0,2,0,0,0,1],
        free_ap: 0,
        lp: 0,
        lang: vec![Common, Elven],
        weap: vec![Rapier, Shortsword, HandCrossbow],
        armor: vec![],
        skill: vec![Perception],
        speed: 30,
        size: Size::Medium
    }
    High {
        fixed_ap: [0,2,0,1,0,0],
        free_ap: 0,
        lp: 1,
        lang: vec![Common, Elven],
        weap: vec![Longsword, Shortsword, Shortbow, Longbow],
        armor: vec![],
        skill: vec![Perception],
        speed: 30,
        size: Size::Medium
    }
    Wood {
        fixed_ap: [0,2,0,0,1,0],
        free_ap: 0,
        lp: 0,
        lang: vec![Common, Elven],
        weap: vec![Longsword, Shortsword, Shortbow, Longbow],
        armor: vec![],
        skill: vec![Perception],
        speed: 35,
        size: Size::Medium
    }
    Sea {
        fixed_ap: [0,2,1,0,0,0],
        free_ap: 0,
        lp: 0,
        lang: vec![Common, Elven, Aquan],
        weap: vec![Spear, Trident, LightCrossbow, Net],
        armor: vec![],
        skill: vec![Perception],
        speed: 30,
        size: Size::Medium
    }
);

//Dwarf
new_race!(Dwarf,
    Duegar {
        fixed_ap: [1,0,2,0,0,0],
        free_ap: 0,
        lp: 0,
        lang: vec![Common, Dwarven, Undercommon],
        weap: vec![Battleaxe, Handaxe, LightHammer, Warhammer],
        armor: vec![Armor::Light, Armor::Medium],
        skill: vec![],
        speed: 25,
        size: Size::Medium
    }
);
