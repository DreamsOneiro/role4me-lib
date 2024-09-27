use character::Character;
use character::race::*;

pub mod character;

fn main() {
    let mut oneiro = Character::new_5e();
    oneiro.select_race_5e(Elf(SubElf::High));
    oneiro.print_stat();
    oneiro.use_lang_point_5e("Elven");
    oneiro.print_stat();
    oneiro.use_lang_point_5e("Dwarven");
    oneiro.print_stat();
    oneiro.remove_lang_5e("Dwarven");
    oneiro.print_stat();
    oneiro.remove_lang_5e("Elven");
    oneiro.print_stat();
    oneiro.clear_lang_5e();
    oneiro.print_stat();
}
