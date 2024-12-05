pub mod profeciency;
pub mod dice;

use std::fmt::Debug;
use profeciency::*;

/* ---------
   | Trait |
   --------- */
pub trait Race {
    fn as_string(&self) -> String;

    fn get_stat(&self) -> Stat;
}

/* ----------
   | Struct |
   ---------- */
#[derive(PartialEq, Eq, Debug)]
pub struct Stat {
    pub ap: [usize; 7],
    pub lang_point: usize,
    pub lang: Vec<Language>,
    pub weap: Vec<Weapon>,
    pub armor: Vec<Armor>,
    pub skill: Vec<Skill>,
    pub speed: usize,
    pub size: Size
}

/* --------
   | Enum |
   -------- */
pub enum Edition {
    FifithEdition
}

impl Edition {
    pub fn as_string(&self) -> String {
        match self {
            Self::FifithEdition => "5e".to_string()
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum AP {
    STR, DEX, CON,
    INT, WIS, CHA
}

impl AP {
    pub fn get_index(&self) -> usize {
        match self {
            Self::STR => 0,
            Self::DEX => 1,
            Self::CON => 2,
            Self::INT => 3,
            Self::WIS => 4,
            Self::CHA => 5,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Size {
    Unknown, Tiny, Small, Medium, Large
}

/* -------------
   | Functions |
   ------------- */
pub fn first_letter_uppercase(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
