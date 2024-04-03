use std::fmt::Display;

use strum_macros::EnumIter;

use crate::{
    dungeon::ChallengeBox,
    hero::{Attribute, AttributeType, Effect, Skill},
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

impl Peril {
    pub fn get_rune_puzzle(is_option_one: bool) -> Peril {
        let item_reward = if is_option_one {
            vec![Attribute {
                attribute: AttributeType::Strength,
                quantity: Some(1),
                ..Default::default()
            }]
        } else {
            vec![Attribute {
                attribute: AttributeType::Agility,
                quantity: Some(1),
                ..Default::default()
            }]
        };

        let skill_reward = if is_option_one {
            Skill {
                name: String::from("CLARITY"),
                description: Some(String::from(
                    "Reroll all your 1s and 2s. Roll 1 x HEROIC DICE",
                )),
                requirements: Some(Attribute {
                    attribute: AttributeType::Potion,
                    quantity: Some(1),
                    ..Default::default()
                }),
                effect: Effect::Reroll(AttributeType::Value(vec![1, 2])),
                encounters: vec![Encounter::Combat, Encounter::Peril, Encounter::Boss],
            }
        } else {
            Skill {
                name: String::from("PERSISTENCE"),
                description: Some(String::from("Roll 1 x STRENGTH DICE and 1 x HEROIC DICE.")),
                requirements: Some(Attribute {
                    attribute: AttributeType::Strength,
                    quantity: Some(2),
                    ..Default::default()
                }),
                effect: Effect::Roll(vec![
                    Attribute {
                        attribute: AttributeType::Strength,
                        quantity: Some(1),
                        ..Default::default()
                    },
                    Attribute {
                        attribute: AttributeType::Heroic,
                        quantity: Some(1),
                        ..Default::default()
                    },
                ]),
                encounters: vec![Encounter::Combat, Encounter::Peril, Encounter::Boss],
            }
        };

        Peril {
            name: String::from("Rune Puzzle"),
            choice_one: vec![ChallengeBox {
                dice_type: Some(AttributeType::Magic),
                total_value: 6,
                single_dice: false,
                priority: false,
                consequences: vec![
                    Attribute {
                        attribute: AttributeType::Health,
                        quantity: Some(1),
                        ..Default::default()
                    },
                    Attribute {
                        attribute: AttributeType::Time,
                        quantity: Some(3),
                        ..Default::default()
                    },
                ],
            }],
            choice_one_time_cost: Some(2),
            choice_two: vec![ChallengeBox {
                dice_type: Some(AttributeType::Strength),
                total_value: 11,
                single_dice: false,
                priority: false,
                consequences: vec![
                    Attribute {
                        attribute: AttributeType::Health,
                        quantity: Some(3),
                        ..Default::default()
                    },
                    Attribute {
                        attribute: AttributeType::Time,
                        quantity: Some(2),
                        ..Default::default()
                    },
                ],
            }],
            choice_two_time_cost: None,
            xp_reward: 2,
            item_reward,
            skill_reward,
        }
    }

    pub fn get_locked_door(is_option_one: bool) -> Peril {
        let item_reward = if is_option_one {
            vec![Attribute {
                attribute: AttributeType::Strength,
                quantity: Some(1),
                ..Default::default()
            }]
        } else {
            vec![Attribute {
                attribute: AttributeType::Magic,
                quantity: Some(1),
                ..Default::default()
            }]
        };

        let skill_reward = if is_option_one {
            Skill {
                name: String::from("SHIMMERBLAST"),
                description: Some(String::from("Roll 1 x HEROIC DICE")),
                requirements: Some(Attribute {
                    attribute: AttributeType::Magic,
                    value: Some(3),
                    ..Default::default()
                }),
                effect: Effect::Roll(vec![Attribute {
                    attribute: AttributeType::Heroic,
                    quantity: Some(1),
                    ..Default::default()
                }]),
                encounters: vec![Encounter::Combat, Encounter::Boss],
            }
        } else {
            Skill {
                name: String::from("ACCURACY"),
                description: Some(String::from("Gain a HEROIC DICE 6")),
                requirements: Some(Attribute {
                    attribute: AttributeType::Agility,
                    quantity: Some(3),
                    ..Default::default()
                }),
                effect: Effect::Gain(vec![Attribute {
                    attribute: AttributeType::Heroic,
                    value: Some(6),
                    ..Default::default()
                }]),
                encounters: vec![Encounter::Combat, Encounter::Peril, Encounter::Boss],
            }
        };

        Peril {
            name: String::from("Locked Door"),
            choice_one: vec![ChallengeBox {
                dice_type: Some(AttributeType::Agility),
                total_value: 8,
                single_dice: false,
                priority: false,
                consequences: vec![
                    Attribute {
                        attribute: AttributeType::Health,
                        quantity: Some(1),

                        ..Default::default()
                    },
                    Attribute {
                        attribute: AttributeType::Time,
                        quantity: Some(4),
                        ..Default::default()
                    },
                ],
            }],
            choice_one_time_cost: Some(1),
            choice_two: vec![ChallengeBox {
                dice_type: Some(AttributeType::Strength),
                total_value: 11,
                single_dice: false,
                priority: false,
                consequences: vec![
                    Attribute {
                        attribute: AttributeType::Health,
                        quantity: Some(2),
                        ..Default::default()
                    },
                    Attribute {
                        attribute: AttributeType::Time,
                        quantity: Some(2),
                        ..Default::default()
                    },
                ],
            }],
            choice_two_time_cost: None,
            xp_reward: 2,
            item_reward,
            skill_reward,
        }
    }
}
