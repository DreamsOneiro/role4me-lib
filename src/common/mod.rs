#![allow(dead_code)]
pub mod profeciency;

/* --------
   | Enum |
   -------- */
pub enum Edition {
    FifithEdition
}

pub enum Size {
    Undefined, Tiny, Small, Medium, Large
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum AP {
    STR, DEX, CON,
    INT, WIS, CHA
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
