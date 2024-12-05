use std::fmt::Debug;
/* ---------
   | Macro |
   --------- */
macro_rules! define_enum {
    ($name:ident {$($item:ident),*}) => {
        #[derive(PartialEq, Eq, Hash, Clone)]
        pub enum $name {
            $(
                $item,
            )*
        }

        impl Profeciency for $name {
            fn as_string(&self) -> String {
                match self {
                    $(
                        Self::$item => stringify!($item).to_string(),
                    )*
                }
            }
        }

        impl Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match self {
                    $(
                        Self::$item => write!(f, "{}", $name::$item.as_string()),
                    )*
                }
            }
        }
    };
}

/* ---------
   | Trait |
   --------- */
pub trait Profeciency {
    fn as_string(&self) -> String;
}

/* ---------------
   | Profeciency |
   --------------- */
define_enum!(Language {
    Common, Elven, Aquan, Dwarven,
    Undercommon
});

define_enum!(Weapon {
    Longsword, Shortsword, Shortbow, Longbow,
    Spear, Trident, LightCrossbow, Net,
    Rapier, HandCrossbow, Battleaxe, Handaxe,
    LightHammer, Warhammer
});

define_enum!(Skill {
    Athletics, Acrobatics, SleightOfHand, Stealth,
    Arcana, History, Investigation, Nature, Religion,
    AnimalHandling, Insight, Medicine, Perception,
    Survial, Deception, Intimidation, Performance,
    Persuasion
});

define_enum!(Armor {
    Light, Medium, Heavy
});
