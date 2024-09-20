#[derive(PartialEq, Eq, Clone, Copy)]
pub enum SubElf {
    Avariel,
    Drow,
    Eladrin,
    Grugach,
    High,
    HighAereni,
    HighValenar,
    MarkOfShadow,
    Pallid,
    Sea,
    ShadarKai,
    Wood,
    WoodAereni,
    WoodValenar
}

// [str,dex,con,int,wis,cha,new]
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
