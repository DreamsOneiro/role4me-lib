use super::Character;
use rand::{thread_rng, Rng};

pub fn roll_dice(c: &mut Character) {
    let mut rng = thread_rng();
    let mut points: [u8; 6] = [0,0,0,0,0,0];
    for i in 0..6 {
        let mut rolls = vec![
            rng.gen_range(1..7),
            rng.gen_range(1..7),
            rng.gen_range(1..7),
            rng.gen_range(1..7)
        ];
        rolls.sort();
        rolls.remove(0);
        let point = rolls[0] + rolls[1] + rolls[2];
        points[i] = point;
    }
    c.unassigned_base_ap = points;
}

pub fn calculate_point_buy(points: [u8;6]) -> [u8;6] {
    let mut assigned: [u8;6] = [0,0,0,0,0,0];
    for (i, point) in points.iter().enumerate() {
        if *point < 6 {
            assigned[i] = point + 8;
        } else if *point == 7 {
            assigned[i] = 6 + 8;
        } else {
            assigned[i] = 7 + 8;
        }
    }
    assigned
}
