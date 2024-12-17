mod fifth_edition;
mod common;

pub mod prelude {
    pub use crate::common::{AP, Size};
    pub use crate::common::profeciency::{Language, Weapon, Skill, Armor, Tools};
}

pub mod ed_5 {
    pub use crate::fifth_edition::Character;
    pub use crate::fifth_edition::race::*;
    pub use crate::fifth_edition::class::Class;
}
