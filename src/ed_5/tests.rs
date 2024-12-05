#[cfg(test)]
use super::*;

#[test]
fn test_new_race() {
    let mut player = Character::build();

     player.select_race(Elf::High);

     assert_eq!(player.check_all_scores(), [0,2,0,1,0,0]);
     assert_eq!(player.race_usable_ap, 0);
     assert_eq!(player.race_used_ability, HashSet::new());
     assert_eq!(player.known_lang(), &HashSet::from([
             Language::Elven,
             Language::Common
     ]));
     assert_eq!(player.lang.len(), 2);
     assert_eq!(player.lang_point, 1);
     assert_eq!(player.weap, HashSet::from([
             Weapon::Longsword,
             Weapon::Shortsword,
             Weapon::Shortbow,
             Weapon::Longbow
     ]));
     assert_eq!(player.armor, HashSet::from([]));
     assert_eq!(player.skill, HashSet::from([Skill::Perception]));
     assert_eq!(player.speed, 30);
     assert_eq!(player.size, Size::Medium);
}

#[test]
fn test_change_race() {
    let mut player = Character::build();

    // Test clean slate when changing race
    player
        .select_race(Human::Variant)
        .use_race_point(AP::INT)
        .use_race_lang(Language::Elven)
        .select_race(Elf::High)
        .select_race(Unknown::Unknown);

    assert_eq!(player.check_all_scores(), [0,0,0,0,0,0]);
    assert_eq!(player.race_usable_ap, 0);
    assert_eq!(player.race_used_ability, HashSet::new());
    assert_eq!(player.known_lang().len(), 0);
    assert_eq!(player.lang_point, 0);
    assert_eq!(player.weap, HashSet::new());
    assert_eq!(player.armor, HashSet::new());
    assert_eq!(player.skill, HashSet::new());
    assert_eq!(player.speed, 0);
    assert_eq!(player.size, Size::Unknown);
    assert_eq!(player.buffer_race, None);
}

#[test]
fn test_use_ap() {
    let mut player = Character::build();

    // Test Unknown race
    player
        .use_race_point(AP::STR)
        .remove_race_point(AP::STR);

    assert_eq!(player.check_all_scores(), [0,0,0,0,0,0]);
    assert_eq!(player.race_usable_ap, 0);
    assert_eq!(player.race_used_ability, HashSet::new());

    // Test race with no usable AP
    player
        .select_race(Human::Basic)
        .use_race_point(AP::STR);

    assert_eq!(player.check_all_scores(), [1,1,1,1,1,1]);
    assert_eq!(player.race_usable_ap, 0);

    // Test assign same ability
    player
        .select_race(Human::Variant)
        .use_race_point(AP::STR)
        .use_race_point(AP::STR);

    assert_eq!(player.check_all_scores(), [1,0,0,0,0,0]);
    assert_eq!(player.race_usable_ap, 1);

    // Test use all ap
    player.use_race_point(AP::DEX);

    assert_eq!(player.check_all_scores(), [1,1,0,0,0,0]);
    assert_eq!(player.race_usable_ap, 0);
}

#[test]
fn test_remove_ap() {
    let mut player = Character::build();

    // Test Unknown race
    player
        .remove_race_point(AP::STR);

    assert_eq!(player.check_all_scores(), [0,0,0,0,0,0]);
    assert_eq!(player.race_used_ability, HashSet::new());

    // Assert point used on STR
    player
        .select_race(Human::Variant)
        .use_race_point(AP::STR);

    assert_eq!(player.check_all_scores(), [1,0,0,0,0,0]);
    assert_eq!(player.race_used_ability, HashSet::from([AP::STR]));

    // Test point removed
    player
        .remove_race_point(AP::STR);

    assert_eq!(player.check_all_scores(), [0,0,0,0,0,0]);
    assert_eq!(player.race_used_ability, HashSet::new());
}

#[test]
fn test_no_race() {
    let mut player = Character::build();

    // Test function error when no race is specified
    player
        .use_race_point(AP::STR)
        .use_race_lang(Language::Elven)
        .remove_race_point(AP::STR)
        .remove_race_lang(Language::Common)
        .clear_race_langs()
        .clear_race_points();

    assert_eq!(player.check_all_scores(), [0,0,0,0,0,0]);
    assert_eq!(player.race_used_ability, HashSet::new());
    assert_eq!(player.known_lang(), &HashSet::new());
}
