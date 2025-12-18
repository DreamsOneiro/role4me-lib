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
    Club, Dagger, Greatclub, Handaxe, Javelin,
    LightHammer, Mace, Quaterstaff, Sickle, Spear, 
    LightCrossbow, Dart, Shortbow, Sling,
    Battleaxe, Flail, Glaive, Greataxe, Greatsword,
    Halberd, Lance, Longsword, Maul, Morningstar,
    Pike, Rapier, Scimitar, Shortsword, Trident,
    WarPick, Warhammer, Whip, Blowgun, HandCrossbow,
    HeavyCrossbow, Longbow, Net
});

define_enum!(Skill {
    Athletics, Acrobatics, SleightOfHand, Stealth,
    Arcana, History, Investigation, Nature, Religion,
    AnimalHandling, Insight, Medicine, Perception,
    Survial, Deception, Intimidation, Performance,
    Persuasion
});

define_enum!(Armor {
    Light, Medium, Heavy, Shield
});

define_enum!(Tools {
    TheivesTools, TinkersTools, DisguiseKit,
    ForgeryKit
});
