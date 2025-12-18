use crate::common::profeciency::{*, Skill::*, Tools::*};
use std::fmt;

pub use Backgrounds::*;

/* ---------
   | Macro |
   --------- */
macro_rules! create_bg {
    ($(
            $bgs:ident{
                skill: $skill:expr,
                language: $language:expr,
                lp: $lp:expr,
                tools: $tools:expr,
                equipment: $equipment:expr,
                gp: $gp:expr

            }
    ),*) => {
        pub enum Backgrounds {
            $($bgs,)*
        }

        impl Backgrounds {
            fn get_stat(&self) -> Background {
                match self {
                    $(
                        Backgrounds::$bgs => Background {
                            bg: stringify!($bgs),
                            skill: $skill,
                            language: $language,
                            lp: $lp,
                            tools: $tools,
                            equipment: $equipment,
                            gp: $gp
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
pub struct Background {
    bg: &'static str,
    skill: Vec<Skill>,
    language: Vec<Language>,
    lp: u8,
    tools:  Vec<Tools>,
    equipment: Vec<&'static str>,
    gp: u8
}

impl Background {
    pub fn new() -> Background {
        Backgrounds::Undefiend.get_stat()
    }

    pub fn reset (&mut self) {
        *self = Background::new();
    }

    pub fn change_bg(&mut self, bg: Backgrounds) {
        *self = bg.get_stat();
    }
}

impl fmt::Debug for Background {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Background")
            .field("\nBackground", &self.bg)
            .field("\nSkill", &self.skill)
            .field("\nLanguage", &self.language)
            .field("\nLanguage Point", &self.lp)
            .field("\nTools", &self.tools)
            .field("\nEquipment", &self.equipment)
            .field("\nGold", &self.gp)
            .finish()
    }
}

/* ---------------
   | Backgrounds |
   --------------- */
create_bg!(
    Acolyte {
        skill: vec![Insight, Religion],
        language: vec![],
        lp: 2,
        tools: vec![],
        equipment: vec![
            "holy symbol", "prayer book or prayer wheel", 
            "5 sticks of incense", "vestments", 
            "a set of common clothes"
        ],
        gp: 15
    },

    Charlatan {
        skill: vec![Deception, SleightOfHand],
        language: vec![],
        lp: 0,
        tools: vec![DisguiseKit, ForgeryKit],
        equipment: vec![
            "a set of fine clothes", "disguise kit",
            "tools of the con of your choice (ten stoppered\
                bottles filled with colored liquid, a set of\
                weighted dice, a deck of marked cards, or\
                a signet ring of an imaginary duke)"
        ],
        gp: 0
    },

    Undefiend {
        skill: vec![],
        language: vec![],
        lp: 0,
        tools: vec![],
        equipment: vec![],
        gp: 0
    }
);
