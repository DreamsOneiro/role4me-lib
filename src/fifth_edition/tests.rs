#[cfg(test)]
use super::*;

#[test]
fn test_new_race() {
    let mut player = Character::build();

     player.race_select(Elf::High);

     assert_eq!(player.get_all_ability_score(), [0,2,0,1,0,0]);
     assert_eq!(player.race_usable_ap, 0);
     assert_eq!(player.race_used_ability, HashSet::new());
     assert_eq!(player.get_all_lang(), &HashSet::from([
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
        .race_select(Human::Variant)
        .race_use_ap(AP::INT)
        .race_use_lang(Language::Elven)
        .race_select(Elf::High)
        .race_select(Unknown::Unknown);

    assert_eq!(player.get_all_ability_score(), [0,0,0,0,0,0]);
    assert_eq!(player.race_usable_ap, 0);
    assert_eq!(player.race_used_ability, HashSet::new());
    assert_eq!(player.get_all_lang().len(), 0);
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
        .race_use_ap(AP::STR)
        .race_remove_ap(AP::STR);

    assert_eq!(player.get_all_ability_score(), [0,0,0,0,0,0]);
    assert_eq!(player.race_usable_ap, 0);
    assert_eq!(player.race_used_ability, HashSet::new());

    // Test race with no usable AP
    player
        .race_select(Human::Basic)
        .race_use_ap(AP::STR);

    assert_eq!(player.get_all_ability_score(), [1,1,1,1,1,1]);
    assert_eq!(player.race_usable_ap, 0);

    // Test assign same ability
    player
        .race_select(Human::Variant)
        .race_use_ap(AP::STR)
        .race_use_ap(AP::STR);

    assert_eq!(player.get_all_ability_score(), [1,0,0,0,0,0]);
    assert_eq!(player.race_usable_ap, 1);

    // Test use all ap
    player.race_use_ap(AP::DEX);

    assert_eq!(player.get_all_ability_score(), [1,1,0,0,0,0]);
    assert_eq!(player.race_usable_ap, 0);
}

#[test]
fn test_remove_ap() {
    let mut player = Character::build();

    // Test Unknown race
    player
        .race_remove_ap(AP::STR);

    assert_eq!(player.get_all_ability_score(), [0,0,0,0,0,0]);
    assert_eq!(player.race_used_ability, HashSet::new());

    // Assert point used on STR
    player
        .race_select(Human::Variant)
        .race_use_ap(AP::STR);

    assert_eq!(player.get_all_ability_score(), [1,0,0,0,0,0]);
    assert_eq!(player.race_used_ability, HashSet::from([AP::STR]));

    // Test point removed
    player
        .race_remove_ap(AP::STR);

    assert_eq!(player.get_all_ability_score(), [0,0,0,0,0,0]);
    assert_eq!(player.race_used_ability, HashSet::new());
}

#[test]
fn test_no_race() {
    let mut player = Character::build();

    // Test function error when no race is specified
    player
        .race_use_ap(AP::STR)
        .race_use_lang(Language::Elven)
        .race_remove_ap(AP::STR)
        .race_remove_lang(Language::Common)
        .race_clear_lang()
        .race_clear_ap();

    assert_eq!(player.get_all_ability_score(), [0,0,0,0,0,0]);
    assert_eq!(player.race_used_ability, HashSet::new());
    assert_eq!(player.get_all_lang(), &HashSet::new());
}
