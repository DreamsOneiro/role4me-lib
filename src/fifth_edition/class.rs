use crate::common::{AP, profeciency::*};
use crate::common::profeciency::{Weapon::*, Skill::*};
use std::fmt;

pub use Classes::*;

/* ---------
   | Macro |
   --------- */
macro_rules! create_class {
    ($(
            $classes:ident{
                armor: $armor:expr,
                weapon: $weapon:expr,
                tools: $tools:expr,
                saving_throws: $saving_throws:expr,
                skill: $skill:expr,
                prof_point: $prof_point:expr
    }
    ),*) => {
        pub enum Classes {
            $($classes,)*
        }

        impl Classes {
            fn get_class(&self) -> Class {
                match self {
                    $(
                        Classes::$classes => Class {
                            class: stringify!($classes),
                            armor: $armor,
                            weapon: $weapon,
                            tools: $tools,
                            saving_throws: $saving_throws,
                            skill: $skill,
                            prof_point: $prof_point
                        }
                    ),*
                }
            }
        }
    };
}

/* ----------
   | Struct |
   ---------- */
pub struct Class {
    class: &'static str,
    armor: Vec<Armor>,
    weapon: Vec<Weapon>,
    tools: Vec<Tools>,
    saving_throws: Vec<AP>,
    skill: Vec<Skill>,
    prof_point: u8
}

impl Class {
    pub fn new() -> Class {
        Class {
            class: "Undefined",
            armor: vec![],
            weapon: vec![],
            tools: vec![],
            saving_throws: vec![],
            skill: vec![],
            prof_point: 0
        }
    }

    pub fn reset(&mut self) {
        *self = Class::new();
    }

    pub fn change_class(&mut self, class: Classes) {
        *self = class.get_class();
    }
}

impl fmt::Debug for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Class")
            .field("\nClass", &self.class)
            .field("\nArmor", &self.armor)
            .field("\nWeapon", &self.weapon)
            .field("\nTools", &self.tools)
            .field("\nSaving Throws", &self.saving_throws)
            .field("\nSkill", &self.skill)
            .field("\nProfeciency Points", &self.prof_point)
            .finish()
    }
}

/* ----------
   | Classes |
   ---------- */
create_class!(
    Cleric {
        armor: vec![Armor::Light, Armor::Heavy, Armor::Shield],
        weapon: vec![
            Club, Dagger, Greatclub, Handaxe, Javelin,
            LightHammer, Mace, Quaterstaff, Sickle, Spear, 
            LightCrossbow, Dart, Shortbow, Sling
        ],
        tools: vec![],
        saving_throws: vec![AP::WIS, AP::CHA],
        skill: vec![History, Insight, Medicine, Persuasion, Religion],
        prof_point: 2
    },

    Ranger {
        armor: vec![Armor::Light, Armor::Medium, Armor::Shield],
        weapon: vec![
            Club, Dagger, Greatclub, Handaxe, Javelin,
            LightHammer, Mace, Quaterstaff, Sickle, Spear, 
            LightCrossbow, Dart, Shortbow, Sling,
            Battleaxe, Flail, Glaive, Greataxe, Greatsword,
            Halberd, Lance, Longsword, Maul, Morningstar,
            Pike, Rapier, Scimitar, Shortsword, Trident,
            WarPick, Warhammer, Whip, Blowgun, HandCrossbow,
            HeavyCrossbow, Longbow, Net
        ],
        tools: vec![],
        saving_throws: vec![AP::STR, AP::DEX],
        skill: vec![
            AnimalHandling, Athletics, Insight, Investigation,
            Nature, Perception, Stealth, Survial
        ],
        prof_point: 3
    },

    UndefinedClass {
        armor: vec![],
        weapon: vec![],
        tools: vec![],
        saving_throws: vec![],
        skill: vec![],
        prof_point: 0
    }
);
