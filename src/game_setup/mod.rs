use crate::encounter::{Combat, Peril};

pub mod combat;
pub mod peril;

pub fn get_all_perils() -> Vec<Peril> {
    peril::get_all_perils()
}

pub fn get_all_combats() -> Vec<Combat> {
    combat::get_all_combats()
}
