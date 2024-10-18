use character::Character;
use character::race::*;

pub mod character;

fn main() {
    let mut oneiro = Character::new_5e();
    oneiro.standard_array_5e();
}
