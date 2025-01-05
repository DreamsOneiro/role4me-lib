use crate::common::profeciency::{Tools, Armor, Weapon::{self, *}, Skill::{self, *}};
use crate::common::AP;

/* ---------
   | Macro |
   --------- */
macro_rules! create_class {
    ($($class:ident{
        armor: $armor:expr,
        weapon: $weapon:expr,
        tools: $tools:expr,
        saving_throws: $saving_throws:expr,
        skill: $skill:expr,
        prof_point: $prof_point:expr
    }),*) => {
        #[derive(Eq, PartialEq, Debug)]
        pub enum Class {
            $($class,)*
        }

        impl Class {
            fn get_armor_prof(&self) -> Vec<Armor> {
                match self {
                    $(
                        Class::$class => $armor,
                    )*
                }
            }

            fn get_weapon_prof(&self) -> Vec<Weapon> {
                match self {
                    $(
                        Class::$class => $weapon,
                    )*
                }
            }

            fn get_tools_prof(&self) -> Vec<Tools> {
                match self {
                    $(
                        Class::$class => $tools,
                    )*
                }
            }

            fn get_saving_throw_prof(&self) -> Vec<AP> {
                match self {
                    $(
                        Class::$class => $saving_throws,
                    )*
                }
            }

            fn get_skill(&self) -> Vec<Skill> {
                match self {
                    $(
                        Class::$class => $skill,
                    )*
                }
            }

            fn get_prof_point(&self) -> usize {
                match self {
                    $(
                        Class::$class => $prof_point,
                    )*
                }
            }
        }
    };
}

/* -----------
   | Classes |
   ----------- */
// Simple Weapons: 
//     [
//         Club, Dagger, Greatclub, Handaxe, Javelin,
//         LightHammer, Mace, Quaterstaff, Sickle, Spear, 
//         LightCrossbow, Dart, Shortbow, Sling
//     ]
// Martial Weapons: 
//     [
//         Battleaxe, Flail, Glaive, Greataxe, Greatsword,
//         Halberd, Lance, Longsword, Maul, Morningstar,
//         Pike, Rapier, Scimitar, Shortsword, Trident,
//         WarPick, Warhammer, Whip, Blowgun, HandCrossbow,
//         HeavyCrossbow, Longbow, Net
//     ]

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

    Unknown {
        armor: vec![],
        weapon: vec![],
        tools: vec![],
        saving_throws: vec![],
        skill: vec![],
        prof_point: 0
    }
);
