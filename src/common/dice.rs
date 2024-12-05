use rand::{thread_rng, Rng};

pub fn roll() -> [usize; 6] {
    let mut rng = thread_rng();
    let mut array: [usize; 6] = [0,0,0,0,0,0];
    for i in 0..6 {
        let mut rolls = vec![
            rng.gen_range(1..7),
            rng.gen_range(1..7),
            rng.gen_range(1..7),
            rng.gen_range(1..7),
        ];
        rolls.sort();
        rolls.remove(0);
        let point = rolls[0] + rolls[1] + rolls[2];
        array[i] = point;
    }
    array
}
