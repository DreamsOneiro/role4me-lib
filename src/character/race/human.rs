pub enum SubHuman {
    Basic,
    Variant,
    MarkOfFinding,
    MarkOfHandling,
    MarkOfMaking,
    MarkOfPassage,
    MarkOfSentinel
}

// [str,dex,con,int,wis,cha,new]
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

    pub fn get_prof(&self) {
    }
}
