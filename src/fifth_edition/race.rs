use crate::common::{Stat, Race, Size, first_letter_uppercase};
use crate::common::profeciency::{Language::*, Weapon::*, Skill::*, Armor};

/* ---------
   | Macro |
   --------- */
macro_rules! new_race {
    ($name:ident, $($sub_name:ident{ap: $ap:expr,
        lang_point: $lang_point:expr, 
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

        impl Race for $name {
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

            fn get_stat(&self) -> Stat {
                match self {
                    $(
                        Self::$sub_name => Stat {
                        ap: $ap, 
                        lang_point: $lang_point,
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
   | Races |
   --------- */
// Human
new_race!(Human,
    Basic {
        ap: [1,1,1,1,1,1,0],
        lang_point: 1,
        lang: vec![Common],
        weap: vec![],
        armor: vec![],
        skill: vec![],
        speed: 30,
        size: Size::Medium
    }
    Variant {
        ap: [0,0,0,0,0,0,2],
        lang_point: 1,
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
        lang_point: 0,
        lang: vec![Common, Elven],
        weap: vec![Rapier, Shortsword, HandCrossbow],
        armor: vec![],
        skill: vec![Perception],
        speed: 30,
        size: Size::Medium
    }
    High {
        ap: [0,2,0,1,0,0,0],
        lang_point: 1,
        lang: vec![Common, Elven],
        weap: vec![Longsword, Shortsword, Shortbow, Longbow],
        armor: vec![],
        skill: vec![Perception],
        speed: 30,
        size: Size::Medium
    }
    Wood {
        ap: [0,2,0,0,1,0,0],
        lang_point: 0,
        lang: vec![Common, Elven],
        weap: vec![Longsword, Shortsword, Shortbow, Longbow],
        armor: vec![],
        skill: vec![Perception],
        speed: 35,
        size: Size::Medium
    }
    Sea {
        ap: [0,2,1,0,0,0,0],
        lang_point: 0,
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
        lang_point: 0,
        lang: vec![Common, Dwarven, Undercommon],
        weap: vec![Battleaxe, Handaxe, LightHammer, Warhammer],
        armor: vec![Armor::Light, Armor::Medium],
        skill: vec![],
        speed: 25,
        size: Size::Medium
    }
);

// No Race
new_race!(Unknown,
    Unknown {
        ap: [0,0,0,0,0,0,0],
        lang_point: 0,
        lang: vec![],
        weap: vec![],
        armor: vec![],
        skill: vec![],
        speed: 0,
        size: Size::Unknown
    }
);
