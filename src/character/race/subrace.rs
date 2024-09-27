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

// Human
derive_sub_race!(SubHuman, 
    Basic, Variant
);

impl SubHuman {
    pub fn get_ap(&self) -> [u8; 7] {
        match_sub!(self,
            Basic, [1,1,1,1,1,1,0],
            Variant, [0,0,0,0,0,0,2]
        )
    }

    pub fn get_language(&self) -> Vec<&'static str> {
        match self {
            Self::Basic => vec!["Common"],
            Self::Variant => vec!["Common"],
        }
    }

    pub fn get_lang_point(&self) -> u8 {
        match self {
            _ => 1
        }
    }
}

// Elf
derive_sub_race!(SubElf,
    Drow, High, Wood, Sea
);

impl SubElf {
    pub fn get_ap(&self) -> [u8; 7] {
        match_sub!(self,
            Drow, [0,2,0,0,0,1,0],
            High, [0,2,0,1,0,0,0],
            Wood, [0,2,0,0,1,0,0],
            Sea, [0,2,1,0,0,0,0]
        )
    }

    pub fn get_language(&self) -> Vec<&'static str> {
        match self {
            Self::Sea => vec!["Common", "Elven", "Aquan"],
            _ => vec!["Common", "Elven"]
        }
    }

    pub fn get_lang_point(&self) -> u8 {
        match self {
            Self::High => 1,
            _ => 0
        }
    }
}
