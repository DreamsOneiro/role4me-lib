use character::Character;
use character::race::*;

pub mod character;

fn main() {
    let mut oneiro = Character::new_5e();
    oneiro.use_point_buy_5e([0,7,5,9,4,2]);
    oneiro.select_race_5e(Human(SubHuman::Variant));
    oneiro.use_race_ap_5e("int");
    oneiro.use_race_ap_5e("con");
    oneiro.init_base_point();
    oneiro.print_stat();
}
