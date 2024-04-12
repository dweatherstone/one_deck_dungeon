use std::fmt::Display;

use strum_macros::EnumIter;

use crate::{
    dungeon::ChallengeBox,
    hero::{Attribute, Effect, Skill},
};

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

pub struct Peril {
    pub name: String,
    pub choice_one: Vec<ChallengeBox>,
    pub choice_one_time_cost: Option<i8>,
    pub choice_two: Vec<ChallengeBox>,
    pub choice_two_time_cost: Option<i8>,
    pub xp_reward: i8,
    pub item_reward: Vec<Attribute>,
    pub skill_reward: Skill,
}

impl Peril {}

pub struct Combat {
    pub name: String,
    pub special_ability: Effect,
    pub challenges: Vec<ChallengeBox>,
    pub xp_reward: i8,
    pub item_reward: Vec<Attribute>,
    pub skill_reward: Skill,
}
