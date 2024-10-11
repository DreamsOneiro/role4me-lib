use character::Character;
use character::race::*;

pub mod character;

fn main() {
    let mut some_char = Character::new_5e();
    some_char.select_race_5e(Human(SubHuman::Variant));
    some_char.print_stat();
    let val = some_char.use_lang_point_5e("Ethernal");
    some_char.print_stat();
    println!("{val}");
    let val = some_char.use_lang_point_5e("Ethernal");
    some_char.print_stat();
    println!("{val}");
    let val = some_char.use_race_ap_5e("str");
    some_char.print_stat();
    println!("{val}");
    let val = some_char.use_race_ap_5e("str");
    some_char.print_stat();
    println!("{val}");
}
