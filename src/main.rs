use role4me_lib::ed_5;

fn main() {
    let mut oneiro = ed_5::Character::build();

    oneiro.select_race(ed_5::Human::Variant)
        .use_race_point(ed_5::AP::INT)
        .use_race_point(ed_5::AP::DEX)
        .use_point_buy([8,13,14,15,12,10])
        .print_debug();

}
