#![allow(dead_code)]
pub mod race;
pub mod class;
mod tests;

use std::{collections::{BTreeSet, HashSet}, fmt::Debug};
use crate::common::{AP, Edition, Race, Stat, Size, dice};
use crate::common::profeciency::{Language, Weapon, Armor, Skill};
use race::*;
use class::Class;

pub struct Character<'a> {
    edition: Edition,
    class: Class,
    race: Box<dyn Race + 'a>,
    race_usable_ap: usize,
    race_used_ability: HashSet<AP>,
    race_used_lang: HashSet<Language>,
    lang_point: usize,
    lang: HashSet<Language>,
    weap: HashSet<Weapon>,
    armor: HashSet<Armor>,
    skill: HashSet<Skill>,
    speed: usize,
    size: Size,
    ap_unassigned: [usize; 6],
    ap_seq: Result<BTreeSet<usize>, String>,
    base_ap: [usize; 6],
    buffer_race: Option<Stat>
}

impl<'a> Character<'a> {
    /* ----------
       | Public |
       ---------- */
    /// Create new character
    /// 
    /// Example:
    /// ```
    /// use role4me_lib::ed_5;
    ///
    /// let mut player = ed_5::Character::build();
    /// ```
    pub fn build() -> Character<'a> {
        Character {
            edition: Edition::FifithEdition,
            class: Class::Unknown,
            race: Box::new(Unknown::Unknown),
            race_usable_ap: 0,
            race_used_ability: HashSet::new(),
            race_used_lang: HashSet::new(),
            lang_point: 0,
            lang: HashSet::new(),
            weap: HashSet::new(),
            armor: HashSet::new(),
            skill: HashSet::new(),
            ap_unassigned: [0,0,0,0,0,0],
            ap_seq: Err("Error: No sequence assigned".to_string()),
            base_ap: [0,0,0,0,0,0],
            speed: 0,
            size: Size::Unknown,
            buffer_race: None
        }
    }

    /// Select/change character race
    /// 
    /// Example:
    /// ```
    /// use role4me_lib::ed_5;
    ///
    /// let mut player = ed_5::Character::build();
    ///
    /// player.race_select(ed_5::Human::Variant);
    ///
    /// assert_eq!(player.get_race(), "Human(Variant)".to_string());
    /// ```
    /// ------------------------------------------------
    /// Refer to `role4me_lib::ed_5::race` for race enums
    pub fn race_select<T: Race + 'a>(&mut self, race: T) -> &mut Self {
        if race.as_string() != self.race.as_string() {
            self.race = Box::new(race);
            self.buffer_race = Some(self.race.get_stat());
            // Clean slate
            self.race_used_ability = HashSet::new();
            self.race_used_lang = HashSet::new();
            // Initialisation
            self.init_race_ap()
                .init_race_lang()
                .init_race_weap()
                .init_race_armor()
                .init_race_skill()
                .init_speed()
                .init_size();
            // Remove race buffer if race is Unkown
            if self.race.as_string() == Unknown::Unknown.as_string() {
                self.buffer_race = None;
            }
        }
        self
    }

    /// Assign points manually from race when applicable
    ///
    /// Example:
    /// ```
    /// use role4me_lib::ed_5;
    /// use role4me_lib::prelude::*;
    ///
    /// let mut player = ed_5::Character::build();
    ///
    /// player
    ///     .race_select(ed_5::Human::Variant)
    ///     .race_use_ap(AP::INT);
    ///
    /// assert_eq!(player.get_ability_score(AP::INT), 1);
    /// ```
    pub fn race_use_ap(&mut self, ability: AP) -> &mut Self {
        if self.race_usable_ap > 0 {
            if self.race_used_ability.insert(ability) {
                self.init_race_ap();
            }
        }
        self
    }

    /// Remove assigned points from race when applicable
    ///
    /// Example:
    /// ```
    /// use role4me_lib::ed_5;
    /// use role4me_lib::prelude::*;
    ///
    /// let mut player = ed_5::Character::build();
    ///
    /// player
    ///     .race_select(ed_5::Human::Variant)
    ///     .race_use_ap(AP::INT)
    ///     .race_remove_ap(AP::INT);
    ///
    /// assert_eq!(player.get_ability_score(AP::INT), 0);
    /// ```
    pub fn race_remove_ap(&mut self, ability: AP) -> &mut Self {
        if self.race_used_ability.remove(&ability) {
            self.init_race_ap();
        }
        self
    }

    /// Remove all assigned points from race when applicable
    ///
    /// Example:
    /// ```
    /// use role4me_lib::ed_5;
    /// use role4me_lib::prelude::*;
    ///
    /// let mut player = ed_5::Character::build();
    ///
    /// player
    ///     .race_select(ed_5::Human::Variant)
    ///     .race_use_ap(AP::INT)
    ///     .race_use_ap(AP::STR);
    ///
    /// assert_eq!(player.get_all_ability_score(), [1,0,0,1,0,0]);
    ///
    /// player.race_clear_ap();
    ///
    /// assert_eq!(player.get_all_ability_score(), [0,0,0,0,0,0]);
    /// ```
    pub fn race_clear_ap(&mut self) -> &mut Self {
        self.race_used_ability = HashSet::new();
        self.init_race_ap();
        self
    }

    /// Assign language manually from race when applicable
    ///
    /// Example:
    /// ```
    /// use role4me_lib::ed_5;
    /// use role4me_lib::prelude::*;
    ///
    /// let mut player = ed_5::Character::build();
    ///
    /// player
    ///     .race_select(ed_5::Human::Variant)
    ///     .race_use_lang(Language::Elven);
    ///
    /// assert!(player.get_all_lang().contains(&Language::Elven));
    /// ```
    pub fn race_use_lang(&mut self, language: Language) -> &mut Self {
        if (!self.lang.contains(&language)) & (self.lang_point > 0) {
            if self.race_used_lang.insert(language) {
                self.init_race_lang();
            }
        }
        self
    }

    /// Remove manually selected langauge when applicable
    ///
    /// Example:
    /// ```
    /// use role4me_lib::ed_5;
    /// use role4me_lib::prelude::*;
    ///
    /// let mut player = ed_5::Character::build();
    ///
    /// player
    ///     .race_select(ed_5::Human::Variant)
    ///     .race_use_lang(Language::Elven);
    ///
    /// assert!(player.get_all_lang().contains(&Language::Elven));
    ///
    /// player.race_remove_lang(Language::Elven);
    ///
    /// assert!(!player.get_all_lang().contains(&Language::Elven));
    /// ```
    pub fn race_remove_lang(&mut self, language: Language) -> &mut Self {
        if self.race_used_lang.remove(&language) {
            self.init_race_lang();
        }
        self
    }

    /// Clear all manually assigned languages
    ///
    /// Example:
    /// ```
    /// use role4me_lib::ed_5;
    /// use role4me_lib::prelude::*;
    ///
    /// let mut player = ed_5::Character::build();
    ///
    /// player
    ///     .race_select(ed_5::Human::Variant)
    ///     .race_use_lang(Language::Elven);
    ///
    /// assert!(player.get_all_lang().contains(&Language::Elven));
    ///
    /// player.race_clear_lang();
    ///
    /// assert!(!player.get_all_lang().contains(&Language::Elven));
    /// ```
    pub fn race_clear_lang(&mut self) -> &mut Self {
        self.race_used_lang = HashSet::new();
        self.init_race_lang();
        self
    }

    /// Use roll method to generate base stat
    ///
    /// Example:
    /// ```
    /// use role4me_lib::ed_5;
    ///
    /// let mut player = ed_5::Character::build();
    ///
    /// player.ap_dice_roll();
    /// ```
    pub fn ap_dice_roll(&mut self) -> &mut Self {
        self.ap_unassigned = dice::roll();
        // Reset ap_seq
        self.ap_seq = Err("Error: No sequence assigned".to_string());
        self
    }

    /// Use Standard Array for base stat
    /// Follow up of ap_assign_seq() method required 
    ///
    /// Example:
    /// ```
    /// use role4me_lib::ed_5;
    ///
    /// let mut player = ed_5::Character::build();
    ///
    /// player.ap_standard_array();
    ///
    /// assert_eq!(player.get_ap_unassigned(), &[15,14,13,12,10,8]);
    /// ```
    pub fn ap_standard_array(&mut self) -> &mut Self {
        self.ap_unassigned = [15,14,13,12,10,8];
        // Reset ap_seq
        self.ap_seq = Err("Error: No sequence assigned".to_string());
        self
    }

    /// Use Heroic Array for base stat
    /// Follow up of ap_assign_seq() method required 
    ///
    /// Example:
    /// ```
    /// use role4me_lib::ed_5;
    ///
    /// let mut player = ed_5::Character::build();
    ///
    /// player.ap_heroic_array();
    ///
    /// assert_eq!(player.get_ap_unassigned(), &[17,16,14,14,12,10]);
    /// ```
    pub fn ap_heroic_array(&mut self) -> &mut Self {
        self.ap_unassigned = [17,16,14,14,12,10];
        // Reset ap_seq
        self.ap_seq = Err("Error: No sequence assigned".to_string());
        self
    }

    /// Use point buy method for base AP
    /// 
    /// Example:
    /// ```
    /// use role4me_lib::ed_5;
    ///
    /// let mut player = ed_5::Character::build();
    ///
    /// player.ap_point_buy([8,13,14,15,12,10]);
    ///
    /// assert_eq!(player.get_all_ability_score(), [8,13,14,15,12,10]);
    /// ```
    pub fn ap_point_buy(&mut self, points: [usize; 6]) -> &mut Self {
        for point in points {
            if !(point >= 8) | !(point <= 15) {
                return self
            }
        }
        self.ap_unassigned = points;
        self.ap_assign_seq([0,1,2,3,4,5]);
        // Reset ap_seq
        self.ap_seq = Err("Error: No sequence assigned".to_string());
        self
    }

    /// Return the remainder from point buy if applicable
    /// Error returns a string that can be printed
    /// for debugging
    ///
    /// Example:
    /// ```
    /// use role4me_lib::ed_5;
    ///
    /// let mut player = ed_5::Character::build();
    ///
    /// assert_eq!(
    /// player.ap_check_point_buy([8,13,14,15,12,10]), Ok(0) 
    /// );
    ///
    /// assert_eq!(
    /// player.ap_check_point_buy([10,10,10,10,10,10]), Ok(15)
    /// );
    /// ```
    /// -----------------------------------------------------
    /// Refer to D&D rules for more information
    /// regarding point buy.
    pub fn ap_check_point_buy(&mut self, points: [usize; 6]) -> Result<usize, String> {
        let mut sum: usize = 0;
        for point in points {
            // Check if point within rule's limit
            if (point >= 8) & (point <= 15) {
                let x = point - 8;
                let b = (x-(x%5))/5; // Match magic, try big brain it yourself
                sum += x + (b * x%5);
                println!("{}", x + (b * x%5));
            }
            else {
                return Err(String::from(format!("Error: Point must be within 8 & 15\n\
                            Current input: {:?}", points)));
            }
        }
        println!("{:?}", points);
        if sum <= 27 {
            Ok(27-sum)
        }
        else {
            Err(String::from(format!("Error: Points sum exceeded limit, \
                    Limit: 27 Assigned: {}", sum)))
        }
    }

    /// Assign sequence to rolled stat
    /// Important:
    /// Value must start from 0, and ends at 5
    ///
    /// Example:
    /// ```
    /// use role4me_lib::ed_5;
    ///
    /// let mut player = ed_5::Character::build();
    ///
    /// player
    ///     .ap_dice_roll()
    ///     .ap_assign_seq([2,1,3,5,4,0]);
    /// ```
    /// ---------------------------------------
    /// From the above example
    /// If rolled stat is [2,4,6,8,10,12]
    /// Rolled stat will be [12,4,2,6,10,8]
    pub fn ap_assign_seq(&mut self, sequence: [usize; 6])
        -> &mut Self {
        let mut set = BTreeSet::new();
        for val in sequence {
            if val < 6 {set.insert(val);}
        }
        if set.len() == 6 {
            self.ap_seq = Ok(set);
        }
        else {
            self.ap_seq = Err(
                format!("Error: Assigned sequence does not \
                    meet requirement. Sequence: {:?}\n\
                    All value must be between 0 and 5 with \
                    no repetition.", sequence)
                );
        }
        self.init_base_ap()
    }

    /// Select/Change character class
    ///
    /// Example: 
    /// ```
    /// use role4me_lib::ed_5;
    /// use role4me_lib::prelude::*;
    ///
    /// let mut player = ed_5::Character::build();
    ///
    /// player.class_select(ed_5::Class::Cleric);
    ///
    /// assert_eq!(player.get_class(), &ed_5::Class::Cleric);
    /// ```
    pub fn class_select(&mut self, class: Class) -> &mut Self {
        self.class = class;
        self
    }

    /// Return race name as String
    ///
    /// Example:
    /// ```
    /// use role4me_lib::ed_5;
    /// use role4me_lib::prelude::*;
    ///
    /// let mut player = ed_5::Character::build();
    ///
    /// assert_eq!(player.get_race(), "Unknown(Unknown)".to_string());
    /// ```
    pub fn get_race(&self) -> String {
        self.race.as_string()
    }

    /// Return refernce to value of assignable ability
    /// score gained from certain race
    ///
    /// Example:
    /// ```
    /// use role4me_lib::ed_5;
    /// use role4me_lib::prelude::*;
    ///
    /// let mut player = ed_5::Character::build();
    ///
    /// player.race_select(ed_5::Human::Variant);
    ///
    /// assert_eq!(player.get_race_unused_ap(), &2);
    ///
    /// player.race_use_ap(AP::STR);
    ///
    /// assert_eq!(player.get_race_unused_ap(), &1);
    /// ```
    pub fn get_race_unused_ap(&self) -> &usize {
        &self.race_usable_ap
    }

    /// Return value of specific ability score from
    /// the final/total caculated ability scores
    ///
    /// Example:
    /// ```
    /// use role4me_lib::ed_5;
    /// use role4me_lib::prelude::*;
    ///
    /// let mut player = ed_5::Character::build();
    ///
    /// player
    ///     .race_select(ed_5::Human::Variant)
    ///     .race_use_ap(AP::INT);
    ///
    /// let val = player.get_ability_score(AP::INT);
    ///
    /// assert_eq!(val, 1);
    /// ```
    pub fn get_ability_score(&self, ap: AP) -> usize {
        let scores = self.get_all_ability_score();
        scores[ap.get_index()]
    }

    /// Calculate and return the total ability scores
    ///
    /// Example:
    /// ```
    /// use role4me_lib::ed_5;
    /// use role4me_lib::prelude::*;
    ///
    /// let mut player = ed_5::Character::build();
    ///
    /// player
    ///     .race_select(ed_5::Human::Variant)
    ///     .race_use_ap(AP::INT);
    ///
    /// let val = player.get_all_ability_score();
    ///
    /// assert_eq!(val, [0,0,0,1,0,0]);
    /// ```
    pub fn get_all_ability_score(&self) -> [usize; 6] {
        let mut ability_scores = [0,0,0,0,0,0];
        self.calculate_race_default(&mut ability_scores);
        self.calculate_race_user(&mut ability_scores);
        self.calculate_base_ap(&mut ability_scores);
        ability_scores
    }

    /// Return reference to a HashSet of known languages
    ///
    /// Example:
    /// ```
    /// use role4me_lib::ed_5;
    /// use role4me_lib::prelude::*;
    /// use std::collections::HashSet;
    ///
    /// let mut player = ed_5::Character::build();
    ///
    /// player.race_select(ed_5::Elf::Sea);
    ///
    /// assert_eq!(player.get_all_lang(), &HashSet::from([
    ///     Language::Aquan,
    ///     Language::Common,
    ///     Language::Elven
    /// ]))
    /// ```
    pub fn get_all_lang(&self) -> &HashSet<Language> {
        &self.lang
    }

    /// Return reference to array of current ap stat 
    /// without/bofore applying seqeunce
    /// (Refer to ap_assign_seq() method)
    ///
    /// Example:
    /// ```
    /// use role4me_lib::ed_5;
    ///
    /// let mut player = ed_5::Character::build();
    ///
    /// player.ap_standard_array();
    ///
    /// println!("{:?}", player.get_ap_unassigned());
    /// assert_eq!(player.get_ap_unassigned(), &[15,14,13,12,10,8]);
    /// ```
    pub fn get_ap_unassigned(&self) -> &[usize; 6] {
        &self.ap_unassigned
    }

    /// Return assigned ap sequence when applicable
    ///
    /// Example:
    /// ```
    /// use role4me_lib::ed_5;
    /// use role4me_lib::prelude::*;
    ///
    /// let mut player = ed_5::Character::build();
    ///
    /// player.race_select(ed_5::Human::Variant)
    ///     .ap_standard_array()
    ///     .ap_assign_seq([0,1,2,3,4,5]);
    /// 
    /// assert_eq!(player.get_ap_seq(), Ok([0,1,2,3,4,5]));
    /// ```
    pub fn get_ap_seq(&self) -> Result<[usize; 6], &str> {
        match &self.ap_seq {
            Ok(seq) => {
                let mut val = seq.iter();
                let ap_seq_array = [
                    val.next().unwrap().clone(),
                    val.next().unwrap().clone(),
                    val.next().unwrap().clone(),
                    val.next().unwrap().clone(),
                    val.next().unwrap().clone(),
                    val.next().unwrap().clone(),
                ];
                Ok(ap_seq_array)
            }
            Err(_) => Err("Error: No usable sequence found")
        }
    }

    /// Return class name as String
    ///
    /// Example:
    /// ```
    /// use role4me_lib::ed_5;
    /// use role4me_lib::prelude::*;
    ///
    /// let mut player = ed_5::Character::build();
    ///
    /// assert_eq!(player.get_class(), &ed_5::Class::Unknown);
    /// ```
    pub fn get_class(&self) -> &Class {
        &self.class
    }

    /// Print debug information
    /// Ends with empty line
    ///
    /// Example:
    /// ```
    /// use role4me_lib::ed_5;
    /// use role4me_lib::prelude::*;
    ///
    /// let mut player = ed_5::Character::build();
    ///
    /// player
    ///     .race_select(ed_5::Human::Variant)
    ///     .print_debug()
    ///     .race_use_ap(AP::INT)
    ///     .print_debug();
    /// ```
    /// ----------------------------------------
    /// `println!("{:?}", player)` will print the
    /// same debug message
    pub fn print_debug(&mut self) -> &mut Self {
        println!("{:?}", self);
        self
    }

    /* -----------
       | Private |
       ----------- */
    // Calculate points assigned from race by default
    fn calculate_race_default(&self, ability_scores: &mut [usize; 6]) {
        if self.buffer_race != None {
            let buff_ptr = &self.buffer_race.as_ref().unwrap();
            let mut race_ap = buff_ptr.ap.iter();
            for score in ability_scores {
                *score += race_ap.next().unwrap();
            }
        }
    }

    // Calculate points assigned from race by user
    fn calculate_race_user(&self, ability_scores: &mut [usize; 6]) {
        for used_ability in &self.race_used_ability {
            ability_scores[used_ability.get_index()] += 1;
        }
    }

    // Add base points to ablity scores
    fn calculate_base_ap(&self, ability_scores: &mut [usize; 6])  {
        // Add base_ap to ability scores
        let mut ptr = ability_scores.iter_mut();
        for point in self.base_ap {
            *ptr.next().unwrap() += point;
        }
    }

    // Calculate base points
    fn init_base_ap(&mut self) -> &mut Self {
        // Clear base_ap
        self.base_ap = [0,0,0,0,0,0];
        // Insert rolled stat to base_ap according to sequence
        if let Ok(seq) = &self.ap_seq {
            let mut base_ap_ptr = self.base_ap.iter_mut();
            for val in seq {
                *base_ap_ptr.next().unwrap() = self.ap_unassigned[*val];
            }
        }
        self
    }

    /// Initialise race ap
    fn init_race_ap(&mut self) -> &mut Self {
        if self.buffer_race != None {
            let buff_ptr = &self.buffer_race.as_ref().unwrap();
            self.race_usable_ap = buff_ptr.ap[6] - self.race_used_ability.len();
        }
        self
    }

    // Initialise race languages
    fn init_race_lang(&mut self) -> &mut Self {
        if self.buffer_race != None {
            let buff_ptr = &self.buffer_race.as_ref().unwrap();
            // Clear all languages
            self.lang = HashSet::new();
            // Initialise default race language(s)
            for lang in &buff_ptr.lang {
                self.lang.insert(lang.clone());
            }
            // Initialise manually assigned language(s)
            for lang in self.race_used_lang.iter() {
                self.lang.insert(lang.clone());
            }
            // Initialise usable language point(s)
            self.lang_point = buff_ptr.lang_point - self.race_used_lang.len();
        }
        self
    }

    // Intialise weapons
    fn init_race_weap(&mut self) -> &mut Self {
        if self.buffer_race != None {
            let buff_ptr = &self.buffer_race.as_ref().unwrap();
            // Clear all weapons
            self.weap = HashSet::new();
            // Initialize weapon profeciency from race
            for weap in &buff_ptr.weap {
                self.weap.insert(weap.clone());
            }
        }
        self
    }

    // Intialise armor
    fn init_race_armor(&mut self) -> &mut Self {
        if self.buffer_race != None {
            let buff_ptr = &self.buffer_race.as_ref().unwrap();
            // Clear all weapons
            self.armor = HashSet::new();
            // Initialize weapon profeciency from race
            for armor in &buff_ptr.armor {
                self.armor.insert(armor.clone());
            }
        }
        self
    }

    // Initialise skills
    fn init_race_skill(&mut self) -> &mut Self {
        if self.buffer_race != None {
            let buff_ptr = &self.buffer_race.as_ref().unwrap();
            // Clear all skills
            self.skill = HashSet::new();
            // Initialize skill profeciency from race
            for skill in &buff_ptr.skill {
                self.skill.insert(skill.clone());
            }
        }
        self
    }

    // Initialize speed
    fn init_speed(&mut self) -> &mut Self {
        if self.buffer_race != None {
            let buff_ptr = &self.buffer_race.as_ref().unwrap();
            self.speed = buff_ptr.speed;
        }
        self
    }

    // Initialize size
    fn init_size(&mut self) -> &mut Self {
        if self.buffer_race != None {
            let buff_ptr = &self.buffer_race.as_ref().unwrap();
            self.size = buff_ptr.size.clone();
        }
        self
    }
}

impl<'a> Debug for Character<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut scores = self.get_all_ability_score();
        let mut score = scores.iter_mut();
        let seq_error;
        if let Err(e) = &self.ap_seq {
            seq_error = e.to_string();
        }
        else {
            seq_error = "".to_string();
        }
        write!(f, "Edition: {}\n\
            STR DEX CON INT WIS CHA\n\
            [{}] [{}] [{}] [{}] [{}] [{}]\n\
            Class: \t{:?}\n\
            Race: \t{}\n\
            Languages: {:?}\n\
            Weapons:   {:?}\n\
            Armor: \t{:?}\n\
            Skills: {:?}\n\
            Speed: \t{:?}\n\
            Size: \t{:?}\n\
            AP: \t{}\n\
            LP: \t{}\n\
            \n{}
            ",
            self.edition.as_string(),
            score.next().unwrap(), score.next().unwrap(),
            score.next().unwrap(), score.next().unwrap(),
            score.next().unwrap(), score.next().unwrap(),
            self.class,
            self.race.as_string(),
            self.lang,
            self.weap,
            self.armor,
            self.skill,
            self.speed,
            self.size,
            self.race_usable_ap,
            self.lang_point,
            seq_error)
    }
}
