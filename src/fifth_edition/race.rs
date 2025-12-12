use crate::common::{Size, first_letter_uppercase};
use crate::common::profeciency::{Armor, Language::{self, *}, Skill::{self, *}, Weapon::{self, *}};
use std::fmt;

/* ---------
   | Macro |
   --------- */
macro_rules! new_race {
    ($name:ident, $($sub_name:ident{
        ap: $ap:expr,
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
                            ap: $ap, 
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
pub struct Race<'a> {
    race: Box<dyn Races + 'a>,
    ap: [u8; 7],
    lp: u8,
    lang: Vec<Language>,
    weap: Vec<Weapon>,
    armor: Vec<Armor>,
    skill: Vec<Skill>,
    speed: u8,
    size: Size
}

impl<'a> Race<'a> {
    pub fn new() -> Self {
        Race {
            race: Box::new(Unknown::Unknown),
            ap: [0,0,0,0,0,0,0],
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
        *self = self.race.get_stat();
    }

    pub fn reset_all(&mut self) {
        self.race = Box::new(Unknown::Unknown);
        self.reset();
    }

    pub fn race_change<T: Races + 'a>(&mut self, new_race: T) {
        self.race = Box::new(new_race);
        self.reset();
    }
}

impl<'a> fmt::Debug for Race<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Race")
            .field("Race", &self.race.as_string())
            .field("AP", &self.ap)
            .field("Laguage Point", &self.lp)
            .field("Language", &self.lang)
            .field("Weapon", &self.weap)
            .field("Armor", &self.armor)
            .field("Skill", &self.skill)
            .field("Speed", &self.speed)
            .finish()
    }
}

/* ---------
   | Races |
   --------- */
// No Race
new_race!(Unknown,
    Unknown {
        ap: [0,0,0,0,0,0,0],
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
        ap: [1,1,1,1,1,1,0],
        lp: 1,
        lang: vec![Common],
        weap: vec![],
        armor: vec![],
        skill: vec![],
        speed: 30,
        size: Size::Medium
    }
    Variant {
        ap: [0,0,0,0,0,0,2],
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
        ap: [0,2,0,0,0,1,0],
        lp: 0,
        lang: vec![Common, Elven],
        weap: vec![Rapier, Shortsword, HandCrossbow],
        armor: vec![],
        skill: vec![Perception],
        speed: 30,
        size: Size::Medium
    }
    High {
        ap: [0,2,0,1,0,0,0],
        lp: 1,
        lang: vec![Common, Elven],
        weap: vec![Longsword, Shortsword, Shortbow, Longbow],
        armor: vec![],
        skill: vec![Perception],
        speed: 30,
        size: Size::Medium
    }
    Wood {
        ap: [0,2,0,0,1,0,0],
        lp: 0,
        lang: vec![Common, Elven],
        weap: vec![Longsword, Shortsword, Shortbow, Longbow],
        armor: vec![],
        skill: vec![Perception],
        speed: 35,
        size: Size::Medium
    }
    Sea {
        ap: [0,2,1,0,0,0,0],
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
        ap: [1,0,2,0,0,0,0],
        lp: 0,
        lang: vec![Common, Dwarven, Undercommon],
        weap: vec![Battleaxe, Handaxe, LightHammer, Warhammer],
        armor: vec![Armor::Light, Armor::Medium],
        skill: vec![],
        speed: 25,
        size: Size::Medium
    }
);
