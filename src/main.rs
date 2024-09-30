use character::Character;
use character::race::*;

pub mod character;

fn main() {
    let mut some_char = Character::new_5e();
    some_char.select_race_5e(Dwarf(SubDwarf::Duegar));
    some_char.print_stat();
    some_char.select_race_5e(Dwarf(SubDwarf::Mountain));
    some_char.print_stat();
    some_char.select_race_5e(Dwarf(SubDwarf::Hill));
    some_char.print_stat();
}
