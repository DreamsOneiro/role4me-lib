macro_rules! derive_sub_race {
    ( $name:ident, $($subrace:tt),* ) => {
        #[derive(PartialEq, Eq, Clone, Copy)]
        pub enum $name {
        $($subrace,)*
        }
    };
}

macro_rules! match_sub {
    ($self:expr, $($sub:tt, $point:expr),*) => {
        match $self {
        $(
        Self::$sub => $point,
        )*
        }
    };
}

#[derive(Clone)]
pub struct SubRace {
    ap: [u8; 7],
    lang: Vec<&'static str>,
    lang_point:u8,
    weap: Vec<&'static str>,
    armor: Vec<&'static str>,
    skill: Vec<&'static str>,
    speed: u8,
    size: &'static str
}

impl SubRace {
    pub fn get_ap(&self) -> [u8;7] {
        self.ap
    }
    pub fn get_lang(&self) -> Vec<&str> {
        self.lang.clone()
    }
    pub fn get_lang_point(&self) -> u8 {
        self.lang_point
    }
    pub fn get_weap(&self) -> Vec<&str> {
        self.weap.clone()
    }
    pub fn get_armor(&self) -> Vec<&str> {
        self.armor.clone()
    }
    pub fn get_skill(&self) -> Vec<&str> {
        self.skill.clone()
    }
    pub fn get_speed(&self) -> u8 {
        self.speed
    }
    pub fn get_size(&self) -> &str {
        self.size
    }
}

// Human
derive_sub_race!(SubHuman, 
    Basic, Variant
);

impl SubHuman {
    pub fn handle(&self) -> Option<SubRace> {
        Some(SubRace {
            ap: match_sub!(self,
                Basic, [1,1,1,1,1,1,0],
                Variant, [0,0,0,0,0,0,2]
            ),
            lang: match self {
                _ => vec!["Common"]
            },
            lang_point: match self {
                _ => 1 
            },
            weap: match self {
                _ => vec![]
            },
            armor: match self {
                _ => vec![]
            },
            skill: match self {
                _ => vec![]
            },
            speed: match self {
                _ => 30
            },
            size: match self {
                _ => "Medium"
            }
        })
    }
}

// Elf
derive_sub_race!(SubElf,
    Drow, High, Wood, Sea
);

impl SubElf {
    pub fn handle(&self) -> Option<SubRace> {
        Some (SubRace {
            ap: match_sub!(self,
                Drow, [0,2,0,0,0,1,0],
                High, [0,2,0,1,0,0,0],
                Wood, [0,2,0,0,1,0,0],
                Sea, [0,2,1,0,0,0,0]
            ),
            lang: match self {
                Self::Sea => vec!["Common", "Elven", "Aquan"],
                _ => vec!["Common", "Elven"]
            },
            lang_point: match self {
                Self::High => 1,
                _ => 0
            },
            weap: match self {
                Self::Drow => vec!["Rapier", "Shortsword", "Hand Corssbow"],
                Self::Sea => vec!["Spear", "Trident", "Light Crossbow", "Net"],
                _ => vec!["Longsword", "Shortsword", "Shortbow", "Longbow"]
            },
            armor: match self {
                _ => vec![]
            },
            skill: match self {
                _ => vec!["Perception"]
            },
            speed: match self {
                Self::Wood => 35,
                _ => 30
            },
            size: match self {
                _ => "Medium"
            }
        })
    }
}

// Dwarf
derive_sub_race!(SubDwarf,
    Duegar, Hill, Mountain
);

impl SubDwarf {
    pub fn handle(&self) -> Option<SubRace> {
        Some (SubRace {
            ap: match_sub!(self,
                Duegar, [1,0,2,0,0,0,0],
                Hill, [0,0,2,0,1,0,0],
                Mountain, [2,0,2,0,0,0,0]
            ),
            lang: match self {
                Self::Duegar => vec!["Common", "Dwarven", "Undercommon"],
                _ => vec!["Common", "Dwarven"]
            },
            lang_point: match self {
                _ => 0
            },
            weap: match self {
                _ => vec!["Battleaxe", "Handaxe", "Light Hammer", "Warhammer"]
            },
            armor: match self {
                Self::Mountain => vec!["Light", "Medium"],
                _ => vec![]
            },
            skill: match self {
                _ => vec![]
            },
            speed: match self {
                _ => 25
            },
            size: match self {
                _ => "Medium"
            }
        })
    }
}
