use std::fmt::Display;

use strum_macros::EnumIter;

#[derive(Debug, PartialEq, EnumIter)]
pub enum Encounter {
    Combat,
    Peril,
    Boss,
}

impl Display for Encounter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
