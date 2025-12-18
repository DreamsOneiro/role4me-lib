pub mod common;
pub mod fifth_edition;

pub mod prelude {
    pub use crate::common::{AP, Size};
    pub use crate::common::profeciency::{Language, Weapon, Skill, Armor, Tools};
}

pub mod ed_5 {
    pub use crate::fifth_edition::Character;
    pub use crate::fifth_edition::race::*;
    pub use crate::fifth_edition::class;
    pub use crate::fifth_edition::background;
}
