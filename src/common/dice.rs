use rand::{thread_rng, Rng};

pub fn roll() -> [usize; 6] {
    let mut rng = thread_rng();
    let mut array: [usize; 6] = [0,0,0,0,0,0];
    (0..6).for_each(|i| {
        let mut rolls = vec![
            rng.gen_range(1..7),
            rng.gen_range(1..7),
            rng.gen_range(1..7),
            rng.gen_range(1..7),
        ];
        rolls.sort();
        rolls.remove(0); // Remove the lowest value
        array[i] = rolls.iter().sum();
    });
    array
}
