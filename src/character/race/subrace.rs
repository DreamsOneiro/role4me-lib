macro_rules! derive_sub_race {
    ( $name:ident, $($subrace:tt),* ) => {
        #[derive(PartialEq, Eq, Clone, Copy)]
        pub enum $name {
            $($subrace,)*
        }
    };
}

// Human
derive_sub_race!(SubHuman, 
    Basic, Variant, MarkOfFinding, MarkOfHandling,
    MarkOfMaking, MarkOfPassage, MarkOfSentinel
);

impl SubHuman {
    pub fn get_ap(&self) -> [u8; 7] {
        match self {
            Self::Basic => [1,1,1,1,1,1,0],
            Self::Variant => [0,0,0,0,0,0,2],
            Self::MarkOfFinding => [0,0,1,0,2,0,0],
            Self::MarkOfHandling => [0,0,0,0,2,0,1],
            Self::MarkOfMaking => [0,0,0,2,0,0,1],
            Self::MarkOfPassage => [0,2,0,0,0,0,1],
            Self::MarkOfSentinel => [0,0,2,0,1,0,0]
        }
    }

    pub fn get_language(&self) -> Vec<&'static str> {
        match self {
            _ => vec!["Common"]
        }
    }
}

// Elf
derive_sub_race!(SubElf,
    Avariel, Drow, Eladrin, Grugach, High,
    HighAereni, HighValenar, MarkOfShadow,
    Pallid, Sea, ShadarKai, Wood, WoodAereni,
    WoodValenar
);

impl SubElf {
    pub fn get_ap(&self) -> [u8; 7] {
        match self {
            Self::Avariel => [0,2,0,0,0,0,0],
            Self::Drow => [0,2,0,0,0,1,0],
            Self::Eladrin => [0,2,0,0,0,1,0],
            Self::Grugach => [1,2,0,0,0,0,0],
            Self::High => [0,2,0,1,0,0,0],
            Self::HighAereni => [0,2,0,1,0,0,0],
            Self::HighValenar => [0,2,0,1,0,0,0],
            Self::MarkOfShadow => [0,2,0,0,0,1,0],
            Self::Pallid => [0,2,0,0,1,0,0],
            Self::Sea => [0,2,1,0,0,0,0],
            Self::ShadarKai => [0,2,1,0,0,0,0],
            Self::Wood => [0,2,0,0,1,0,0],
            Self::WoodAereni => [0,2,0,0,1,0,0],
            Self::WoodValenar => [0,2,0,0,1,0,0]
        }
    }
}
