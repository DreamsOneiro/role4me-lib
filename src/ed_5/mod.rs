#![allow(dead_code)]
pub mod race;
mod tests;

use std::{collections::{BTreeSet, HashSet}, fmt::Debug};
use crate::common::{Race, Edition, Stat, dice};
pub use race::*;
pub use crate::common::{AP, Size, profeciency::*};

pub struct Character<'a> {
    edition: Edition,
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
            ap_seq: Err("No value".to_string()),
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
    /// player.select_race(ed_5::Human::Variant);
    /// ```
    /// ------------------------------------------------
    /// Refer to `role4me_lib::ed_5::race` for race enums
    pub fn select_race<T: Race + 'a>(&mut self, race: T) -> &mut Self {
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
    /// use role4me_lib::ed_5::{self, AP};
    ///
    /// let mut player = ed_5::Character::build();
    ///
    /// player
    ///     .select_race(ed_5::Human::Variant)
    ///     .use_race_point(AP::INT);
    ///
    /// assert_eq!(player.check_score(AP::INT), 1);
    /// ```
    pub fn use_race_point(&mut self, ability: AP) -> &mut Self {
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
    /// use role4me_lib::ed_5::{self, AP};
    ///
    /// let mut player = ed_5::Character::build();
    ///
    /// player
    ///     .select_race(ed_5::Human::Variant)
    ///     .use_race_point(AP::INT)
    ///     .remove_race_point(AP::INT);
    ///
    /// assert_eq!(player.check_score(AP::INT), 0);
    /// ```
    pub fn remove_race_point(&mut self, ability: AP) -> &mut Self {
        if self.race_used_ability.remove(&ability) {
            self.init_race_ap();
        }
        self
    }

    /// Remove all assigned points from race when applicable
    ///
    /// Example:
    /// ```
    /// use role4me_lib::ed_5::{self, AP};
    ///
    /// let mut player = ed_5::Character::build();
    ///
    /// player
    ///     .select_race(ed_5::Human::Variant)
    ///     .use_race_point(AP::INT)
    ///     .use_race_point(AP::STR);
    ///
    /// assert_eq!(player.check_all_scores(), [1,0,0,1,0,0]);
    ///
    /// player.clear_race_points();
    ///
    /// assert_eq!(player.check_all_scores(), [0,0,0,0,0,0]);
    /// ```
    pub fn clear_race_points(&mut self) -> &mut Self {
        self.race_used_ability = HashSet::new();
        self.init_race_ap();
        self
    }

    /// Return value of current ability score
    ///
    /// Example:
    /// ```
    /// use role4me_lib::ed_5::{self, AP};
    ///
    /// let mut player = ed_5::Character::build();
    ///
    /// player
    ///     .select_race(ed_5::Human::Variant)
    ///     .use_race_point(AP::INT);
    ///
    /// let val = player.check_score(AP::INT);
    ///
    /// assert_eq!(val, 1);
    /// ```
    pub fn check_score(&self, ap: AP) -> usize {
        let scores = self.check_all_scores();
        scores[ap.get_index()]
    }

    /// Calculate and return the total ability scores
    ///
    /// Example:
    /// ```
    /// use role4me_lib::ed_5::{self, AP};
    ///
    /// let mut player = ed_5::Character::build();
    ///
    /// player
    ///     .select_race(ed_5::Human::Variant)
    ///     .use_race_point(AP::INT);
    ///
    /// let val = player.check_all_scores();
    ///
    /// assert_eq!(val, [0,0,0,1,0,0]);
    /// ```
    pub fn check_all_scores(&self) -> [usize; 6] {
        let mut ability_scores = [0,0,0,0,0,0];
        self.calculate_race_default(&mut ability_scores);
        self.calculate_race_user(&mut ability_scores);
        self.calculate_base_ap(&mut ability_scores);
        ability_scores
    }

    /// Assign language manually from race when applicable
    ///
    /// Example:
    /// ```
    /// use role4me_lib::ed_5::{self, Language};
    ///
    /// let mut player = ed_5::Character::build();
    ///
    /// player
    ///     .select_race(ed_5::Human::Variant)
    ///     .use_race_lang(Language::Elven);
    ///
    /// assert!(player.known_lang().contains(&Language::Elven));
    /// ```
    pub fn use_race_lang(&mut self, language: Language) -> &mut Self {
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
    /// use role4me_lib::ed_5::{self, Language};
    ///
    /// let mut player = ed_5::Character::build();
    ///
    /// player
    ///     .select_race(ed_5::Human::Variant)
    ///     .use_race_lang(Language::Elven);
    ///
    /// assert!(player.known_lang().contains(&Language::Elven));
    ///
    /// player.remove_race_lang(Language::Elven);
    ///
    /// assert!(!player.known_lang().contains(&Language::Elven));
    /// ```
    pub fn remove_race_lang(&mut self, language: Language) -> &mut Self {
        if self.race_used_lang.remove(&language) {
            self.init_race_lang();
        }
        self
    }

    /// Clear all manually assigned languages
    ///
    /// Example:
    /// ```
    /// use role4me_lib::ed_5::{self, Language};
    ///
    /// let mut player = ed_5::Character::build();
    ///
    /// player
    ///     .select_race(ed_5::Human::Variant)
    ///     .use_race_lang(Language::Elven);
    ///
    /// assert!(player.known_lang().contains(&Language::Elven));
    ///
    /// player.clear_race_langs();
    ///
    /// assert!(!player.known_lang().contains(&Language::Elven));
    /// ```
    pub fn clear_race_langs(&mut self) -> &mut Self {
        self.race_used_lang = HashSet::new();
        self.init_race_lang();
        self
    }

    /// Return HashSet of known languages
    ///
    /// Example:
    /// ```
    /// use role4me_lib::ed_5::{self, Language};
    /// use std::collections::HashSet;
    ///
    /// let mut player = ed_5::Character::build();
    ///
    /// player.select_race(ed_5::Elf::Sea);
    ///
    /// assert_eq!(player.known_lang(), &HashSet::from([
    ///     Language::Aquan,
    ///     Language::Common,
    ///     Language::Elven
    /// ]))
    /// ```
    pub fn known_lang(&mut self) -> &HashSet<Language> {
        &self.lang
    }

    /// Use roll method to generate base stat
    ///
    /// Example:
    /// ```
    /// use role4me_lib::ed_5;
    ///
    /// let mut player = ed_5::Character::build();
    ///
    /// player.use_dice_roll();
    /// ```
    pub fn use_dice_roll(&mut self) -> &mut Self {
        self.ap_unassigned = dice::roll();
        self
    }

    /// Use Standard Array for base stat
    /// Follow up of assign_ap_seq() method required 
    ///
    /// Example:
    /// ```
    /// use role4me_lib::ed_5;
    ///
    /// let mut player = ed_5::Character::build();
    ///
    /// player.use_standard_array();
    ///
    /// assert_eq!(player.get_ap_unassigned(), &[15,14,13,12,10,8]);
    /// ```
    pub fn use_standard_array(&mut self) -> &mut Self {
        self.ap_unassigned = [15,14,13,12,10,8];
        self
    }

    /// Use Heroic Array for base stat
    /// Follow up of assign_ap_seq() method required 
    ///
    /// Example:
    /// ```
    /// use role4me_lib::ed_5;
    ///
    /// let mut player = ed_5::Character::build();
    ///
    /// player.use_heroic_array();
    ///
    /// assert_eq!(player.get_ap_unassigned(), &[17,16,14,14,12,10]);
    /// ```
    pub fn use_heroic_array(&mut self) -> &mut Self {
        self.ap_unassigned = [17,16,14,14,12,10];
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
    /// player.use_point_buy([8,13,14,15,12,10]);
    ///
    /// assert_eq!(player.check_all_scores(), [8,13,14,15,12,10]);
    /// ```
    pub fn use_point_buy(&mut self, points: [usize; 6]) -> &mut Self {
        for point in points {
            if !(point >= 8) | !(point <= 15) {
                return self
            }
        }
        self.ap_unassigned = points;
        self.assign_ap_seq([0,1,2,3,4,5]);
        self
    }

    /// Return the remainder as Result
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
    /// player.check_point_buy([8,13,14,15,12,10]), Ok(0) 
    /// );
    ///
    /// assert_eq!(
    /// player.check_point_buy([10,10,10,10,10,10]), Ok(15)
    /// );
    /// ```
    /// -----------------------------------------------------
    /// Refer to D&D rules for more information
    /// regarding point buy.
    pub fn check_point_buy(&mut self, points: [usize; 6]) -> Result<usize, String> {
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

    /// Return a rolled stat array
    ///
    /// Example:
    /// ```
    /// use role4me_lib::ed_5;
    ///
    /// let mut player = ed_5::Character::build();
    ///
    /// player.use_standard_array();
    ///
    /// println!("{:?}", player.get_ap_unassigned());
    /// assert_eq!(player.get_ap_unassigned(), &[15,14,13,12,10,8]);
    /// ```
    pub fn get_ap_unassigned(&self) -> &[usize; 6] {
        &self.ap_unassigned
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
    ///     .use_dice_roll()
    ///     .assign_ap_seq([2,1,3,5,4,0]);
    /// ```
    /// ---------------------------------------
    /// From the above example
    /// If rolled stat is [2,4,6,8,10,12]
    /// Rolled stat will be [12,4,2,6,10,8]
    pub fn assign_ap_seq(&mut self, sequence: [usize; 6])
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

    /// Print debug information
    /// Ends with empty line
    ///
    /// Example:
    /// ```
    /// use role4me_lib::ed_5::{self, AP};
    ///
    /// let mut player = ed_5::Character::build();
    ///
    /// player
    ///     .select_race(ed_5::Human::Variant)
    ///     .print_debug()
    ///     .use_race_point(AP::INT)
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
        let mut scores = self.check_all_scores();
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
            Race: \t{}\n\
            AP: \t{}\n\
            LP: \t{}\n\
            Languages: {:?}\n\
            Weapons: {:?}\n\
            Armor: {:?}\n\
            Skills: {:?}\n\
            Speed: {:?}\n\
            Size: {:?}\n\
            \n{}",
            self.edition.as_string(),
            score.next().unwrap(),
            score.next().unwrap(),
            score.next().unwrap(),
            score.next().unwrap(),
            score.next().unwrap(),
            score.next().unwrap(),
            self.race.as_string(),
            self.race_usable_ap,
            self.lang_point,
            self.lang,
            self.weap,
            self.armor,
            self.skill,
            self.speed,
            self.size,
            seq_error)
    }
}
