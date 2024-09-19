use character::Character;
use character::Race::*;
use character::race::*;

pub mod character;

fn main() {
    let mut test = Character::new();
    test.select_race_5e(Human(SubHuman::Basic));
    test.print_stat();
    test.select_race_5e(Human(SubHuman::Variant));
    test.print_stat();
    test.use_race_ap_5e("str");
    test.print_stat();
    test.select_race_5e(Human(SubHuman::MarkOfHandling));
    test.print_stat();
    test.use_race_ap_5e("wis");
    test.print_stat();
    test.use_race_ap_5e("str");
    test.print_stat();
}
