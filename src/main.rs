use character::Character;
use character::Race::*;
use character::race::*;

pub mod character;

fn main() {
    let mut oneiro = Character::new_5e();
    oneiro.select_race_5e(Human(SubHuman::Variant));
    oneiro.print_stat();
    oneiro.use_race_ap_5e("int");
    oneiro.print_stat();
    oneiro.use_race_ap_5e("dex");
    oneiro.print_stat();
    oneiro.remove_race_ap_5e("dex");
    oneiro.print_stat();
    oneiro.clear_race_ap_5e();
    oneiro.print_stat();
}
