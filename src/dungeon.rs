use std::{collections::HashMap, fmt::Display};

use crate::hero::{Attribute, AttributeType};

pub struct ChallengeBox {
    pub dice_type: Option<AttributeType>,
    pub total_value: i8,
    pub single_dice: bool,
    pub priority: bool,
    pub consequences: Vec<Attribute>,
}

impl Display for ChallengeBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        if self.priority {
            output.push_str("*PRIORITY* ");
        }
        if self.single_dice {
            output.push_str("1 x ");
        } else {
            output.push_str("Many ");
        }
        if let Some(dice_type) = &self.dice_type {
            output.push_str(&dice_type.to_string());
        } else {
            output.push_str("any");
        }
        output.push_str(&format!(" \u{2265} {} OR ", self.total_value));
        if self.consequences.is_empty() {
            output.push_str("nothing");
        } else {
            let consequences_string = self
                .consequences
                .iter()
                .map(|c| format!("{}", c))
                .collect::<Vec<String>>()
                .join(", ");
            output.push_str(&consequences_string);
        }

        write!(f, "{}", output)
    }
}

pub struct Dungeon {
    pub name: String,
    pub difficulty: usize,
    pub peril_challenges: HashMap<i8, Vec<ChallengeBox>>,
    pub combat_challenges: HashMap<i8, Vec<ChallengeBox>>,
}

impl Dungeon {
    pub fn get_phoenix_den() -> Dungeon {
        let mut peril_challenges = HashMap::new();
        peril_challenges.insert(
            1,
            vec![ChallengeBox {
                dice_type: None,
                total_value: 3,
                single_dice: true,
                priority: true,
                consequences: Vec::new(),
            }],
        );
        peril_challenges.insert(
            2,
            vec![ChallengeBox {
                dice_type: None,
                total_value: 5,
                single_dice: true,
                priority: false,
                consequences: vec![
                    Attribute {
                        attribute: AttributeType::Health,
                        quantity: Some(1),
                        ..Default::default()
                    },
                    Attribute {
                        attribute: AttributeType::Time,
                        quantity: Some(1),
                        ..Default::default()
                    },
                ],
            }],
        );
        peril_challenges.insert(
            3,
            vec![ChallengeBox {
                dice_type: None,
                total_value: 6,
                single_dice: true,
                priority: false,
                consequences: vec![Attribute {
                    attribute: AttributeType::Time,
                    quantity: Some(2),
                    ..Default::default()
                }],
            }],
        );
        let mut combat_challenges = HashMap::new();
        combat_challenges.insert(
            1,
            vec![
                ChallengeBox {
                    dice_type: Some(AttributeType::Magic),
                    total_value: 3,
                    single_dice: true,
                    priority: false,
                    consequences: vec![Attribute {
                        attribute: AttributeType::Health,
                        quantity: Some(1),
                        ..Default::default()
                    }],
                },
                ChallengeBox {
                    dice_type: Some(AttributeType::Strength),
                    total_value: 3,
                    single_dice: true,
                    priority: false,
                    consequences: vec![Attribute {
                        attribute: AttributeType::Time,
                        quantity: Some(2),
                        ..Default::default()
                    }],
                },
            ],
        );
        combat_challenges.insert(
            2,
            vec![ChallengeBox {
                dice_type: Some(AttributeType::Agility),
                total_value: 3,
                single_dice: true,
                priority: true,
                consequences: Vec::new(),
            }],
        );
        combat_challenges.insert(
            3,
            vec![
                ChallengeBox {
                    dice_type: Some(AttributeType::Strength),
                    total_value: 4,
                    single_dice: true,
                    priority: false,
                    consequences: vec![Attribute {
                        attribute: AttributeType::Health,
                        quantity: Some(2),
                        ..Default::default()
                    }],
                },
                ChallengeBox {
                    dice_type: Some(AttributeType::Strength),
                    total_value: 4,
                    single_dice: true,
                    priority: false,
                    consequences: vec![Attribute {
                        attribute: AttributeType::Time,
                        quantity: Some(2),
                        ..Default::default()
                    }],
                },
            ],
        );
        Dungeon {
            name: String::from("Phoenix's Den"),
            difficulty: 2,
            peril_challenges,
            combat_challenges,
        }
    }

    pub fn get_dragons_cave() -> Dungeon {
        let mut peril_challenges = HashMap::new();
        peril_challenges.insert(
            1,
            vec![ChallengeBox {
                dice_type: None,
                total_value: 2,
                single_dice: true,
                priority: true,
                consequences: Vec::new(),
            }],
        );
        peril_challenges.insert(
            2,
            vec![ChallengeBox {
                dice_type: None,
                total_value: 6,
                single_dice: true,
                priority: false,
                consequences: vec![Attribute {
                    attribute: AttributeType::Health,
                    quantity: Some(1),
                    ..Default::default()
                }],
            }],
        );
        peril_challenges.insert(
            3,
            vec![ChallengeBox {
                dice_type: None,
                total_value: 5,
                single_dice: true,
                priority: true,
                consequences: Vec::new(),
            }],
        );
        let mut combat_challenges = HashMap::new();
        combat_challenges.insert(
            1,
            vec![ChallengeBox {
                dice_type: Some(AttributeType::Strength),
                total_value: 3,
                single_dice: true,
                priority: false,
                consequences: vec![Attribute {
                    attribute: AttributeType::Health,
                    quantity: Some(1),
                    ..Default::default()
                }],
            }],
        );
        combat_challenges.insert(
            2,
            vec![ChallengeBox {
                dice_type: Some(AttributeType::Magic),
                total_value: 5,
                single_dice: true,
                priority: false,
                consequences: vec![Attribute {
                    attribute: AttributeType::Health,
                    quantity: Some(2),
                    ..Default::default()
                }],
            }],
        );
        combat_challenges.insert(
            3,
            vec![ChallengeBox {
                dice_type: Some(AttributeType::Strength),
                total_value: 10,
                single_dice: false,
                priority: true,
                consequences: Vec::new(),
            }],
        );

        Dungeon {
            name: String::from("Dragon's Cave"),
            difficulty: 1,
            peril_challenges,
            combat_challenges,
        }
    }

    pub fn get_hydras_reef() -> Dungeon {
        let mut peril_challenges = HashMap::new();
        peril_challenges.insert(
            1,
            vec![ChallengeBox {
                dice_type: None,
                total_value: 5,
                single_dice: true,
                priority: false,
                consequences: vec![
                    Attribute {
                        attribute: AttributeType::Health,
                        quantity: Some(1),
                        ..Default::default()
                    },
                    Attribute {
                        attribute: AttributeType::Time,
                        quantity: Some(1),
                        ..Default::default()
                    },
                ],
            }],
        );
        peril_challenges.insert(
            2,
            vec![ChallengeBox {
                dice_type: None,
                total_value: 3,
                single_dice: true,
                priority: true,
                consequences: Vec::new(),
            }],
        );
        peril_challenges.insert(
            3,
            vec![ChallengeBox {
                dice_type: None,
                total_value: 5,
                single_dice: true,
                priority: true,
                consequences: Vec::new(),
            }],
        );
        let mut combat_challenges = HashMap::new();
        combat_challenges.insert(
            1,
            vec![ChallengeBox {
                dice_type: Some(AttributeType::Agility),
                total_value: 3,
                single_dice: true,
                priority: false,
                consequences: vec![Attribute {
                    attribute: AttributeType::Health,
                    quantity: Some(1),
                    ..Default::default()
                }],
            }],
        );
        combat_challenges.insert(
            2,
            vec![
                ChallengeBox {
                    dice_type: Some(AttributeType::Agility),
                    total_value: 5,
                    single_dice: true,
                    priority: false,
                    consequences: vec![Attribute {
                        attribute: AttributeType::Health,
                        quantity: Some(1),
                        ..Default::default()
                    }],
                },
                ChallengeBox {
                    dice_type: Some(AttributeType::Strength),
                    total_value: 5,
                    single_dice: true,
                    priority: false,
                    consequences: vec![
                        Attribute {
                            attribute: AttributeType::Health,
                            quantity: Some(1),
                            ..Default::default()
                        },
                        Attribute {
                            attribute: AttributeType::Time,
                            quantity: Some(1),
                            ..Default::default()
                        },
                    ],
                },
            ],
        );
        combat_challenges.insert(
            3,
            vec![ChallengeBox {
                dice_type: Some(AttributeType::Magic),
                total_value: 5,
                single_dice: true,
                priority: false,
                consequences: vec![
                    Attribute {
                        attribute: AttributeType::Health,
                        quantity: Some(1),
                        ..Default::default()
                    },
                    Attribute {
                        attribute: AttributeType::Time,
                        quantity: Some(1),
                        ..Default::default()
                    },
                ],
            }],
        );

        Dungeon {
            name: String::from("Hydra's Reef"),
            difficulty: 2,
            peril_challenges,
            combat_challenges,
        }
    }

    pub fn get_yetis_cavern() -> Dungeon {
        let mut peril_challenges = HashMap::new();
        peril_challenges.insert(
            1,
            vec![ChallengeBox {
                dice_type: None,
                total_value: 4,
                single_dice: true,
                priority: true,
                consequences: vec![Attribute {
                    attribute: AttributeType::Time,
                    quantity: Some(1),
                    ..Default::default()
                }],
            }],
        );
        peril_challenges.insert(
            2,
            vec![ChallengeBox {
                dice_type: None,
                total_value: 5,
                single_dice: true,
                priority: false,
                consequences: vec![
                    Attribute {
                        attribute: AttributeType::Health,
                        quantity: Some(1),
                        ..Default::default()
                    },
                    Attribute {
                        attribute: AttributeType::Time,
                        quantity: Some(1),
                        ..Default::default()
                    },
                ],
            }],
        );
        peril_challenges.insert(
            3,
            vec![ChallengeBox {
                dice_type: None,
                total_value: 4,
                single_dice: true,
                priority: true,
                consequences: vec![Attribute {
                    attribute: AttributeType::Health,
                    quantity: Some(1),
                    ..Default::default()
                }],
            }],
        );
        let mut combat_challenges = HashMap::new();
        combat_challenges.insert(
            1,
            vec![
                ChallengeBox {
                    dice_type: Some(AttributeType::Magic),
                    total_value: 3,
                    single_dice: true,
                    priority: false,
                    consequences: vec![Attribute {
                        attribute: AttributeType::Time,
                        quantity: Some(1),
                        ..Default::default()
                    }],
                },
                ChallengeBox {
                    dice_type: Some(AttributeType::Magic),
                    total_value: 4,
                    single_dice: true,
                    priority: false,
                    consequences: vec![Attribute {
                        attribute: AttributeType::Time,
                        quantity: Some(1),
                        ..Default::default()
                    }],
                },
            ],
        );
        combat_challenges.insert(
            2,
            vec![ChallengeBox {
                dice_type: Some(AttributeType::Strength),
                total_value: 5,
                single_dice: true,
                priority: false,
                consequences: vec![Attribute {
                    attribute: AttributeType::Health,
                    quantity: Some(2),
                    ..Default::default()
                }],
            }],
        );
        combat_challenges.insert(
            3,
            vec![
                ChallengeBox {
                    dice_type: Some(AttributeType::Strength),
                    total_value: 3,
                    single_dice: true,
                    priority: true,
                    consequences: vec![Attribute {
                        attribute: AttributeType::Health,
                        quantity: Some(1),
                        ..Default::default()
                    }],
                },
                ChallengeBox {
                    dice_type: Some(AttributeType::Strength),
                    total_value: 3,
                    single_dice: true,
                    priority: true,
                    consequences: vec![Attribute {
                        attribute: AttributeType::Health,
                        quantity: Some(1),
                        ..Default::default()
                    }],
                },
            ],
        );

        Dungeon {
            name: String::from("Yeti's Cavern"),
            difficulty: 2,
            peril_challenges,
            combat_challenges,
        }
    }

    pub fn get_lichs_tomb() -> Dungeon {
        let mut peril_challenges = HashMap::new();
        peril_challenges.insert(
            1,
            vec![ChallengeBox {
                dice_type: None,
                total_value: 4,
                single_dice: true,
                priority: true,
                consequences: Vec::new(),
            }],
        );
        peril_challenges.insert(
            2,
            vec![ChallengeBox {
                dice_type: None,
                total_value: 6,
                single_dice: true,
                priority: false,
                consequences: vec![
                    Attribute {
                        attribute: AttributeType::Health,
                        quantity: Some(1),
                        ..Default::default()
                    },
                    Attribute {
                        attribute: AttributeType::Time,
                        quantity: Some(1),
                        ..Default::default()
                    },
                ],
            }],
        );
        peril_challenges.insert(
            3,
            vec![ChallengeBox {
                dice_type: None,
                total_value: 4,
                single_dice: true,
                priority: true,
                consequences: vec![Attribute {
                    attribute: AttributeType::Health,
                    quantity: Some(1),
                    ..Default::default()
                }],
            }],
        );
        let mut combat_challenges = HashMap::new();
        combat_challenges.insert(
            1,
            vec![
                ChallengeBox {
                    dice_type: Some(AttributeType::Magic),
                    total_value: 2,
                    single_dice: true,
                    priority: true,
                    consequences: Vec::new(),
                },
                ChallengeBox {
                    dice_type: Some(AttributeType::Strength),
                    total_value: 5,
                    single_dice: true,
                    priority: false,
                    consequences: vec![Attribute {
                        attribute: AttributeType::Time,
                        quantity: Some(2),
                        ..Default::default()
                    }],
                },
            ],
        );
        combat_challenges.insert(
            2,
            vec![ChallengeBox {
                dice_type: Some(AttributeType::Magic),
                total_value: 3,
                single_dice: true,
                priority: true,
                consequences: Vec::new(),
            }],
        );
        combat_challenges.insert(
            3,
            vec![ChallengeBox {
                dice_type: Some(AttributeType::Magic),
                total_value: 10,
                single_dice: false,
                priority: false,
                consequences: vec![Attribute {
                    attribute: AttributeType::Health,
                    quantity: Some(2),
                    ..Default::default()
                }],
            }],
        );

        Dungeon {
            name: String::from("Lich's Tomb"),
            difficulty: 3,
            peril_challenges,
            combat_challenges,
        }
    }

    pub fn get_minotaurs_maze() -> Dungeon {
        let mut peril_challenges = HashMap::new();
        peril_challenges.insert(
            1,
            vec![ChallengeBox {
                dice_type: None,
                total_value: 5,
                single_dice: true,
                priority: true,
                consequences: vec![Attribute {
                    attribute: AttributeType::Health,
                    quantity: Some(1),
                    ..Default::default()
                }],
            }],
        );
        peril_challenges.insert(
            2,
            vec![ChallengeBox {
                dice_type: None,
                total_value: 3,
                single_dice: true,
                priority: true,
                consequences: Vec::new(),
            }],
        );
        peril_challenges.insert(
            3,
            vec![ChallengeBox {
                dice_type: None,
                total_value: 6,
                single_dice: true,
                priority: false,
                consequences: vec![
                    Attribute {
                        attribute: AttributeType::Health,
                        quantity: Some(1),
                        ..Default::default()
                    },
                    Attribute {
                        attribute: AttributeType::Time,
                        quantity: Some(1),
                        ..Default::default()
                    },
                ],
            }],
        );
        let mut combat_challenges = HashMap::new();
        combat_challenges.insert(
            1,
            vec![
                ChallengeBox {
                    dice_type: Some(AttributeType::Agility),
                    total_value: 2,
                    single_dice: true,
                    priority: false,
                    consequences: vec![Attribute {
                        attribute: AttributeType::Health,
                        quantity: Some(1),
                        ..Default::default()
                    }],
                },
                ChallengeBox {
                    dice_type: Some(AttributeType::Agility),
                    total_value: 2,
                    single_dice: true,
                    priority: false,
                    consequences: vec![Attribute {
                        attribute: AttributeType::Health,
                        quantity: Some(1),
                        ..Default::default()
                    }],
                },
            ],
        );
        combat_challenges.insert(
            2,
            vec![
                ChallengeBox {
                    dice_type: Some(AttributeType::Strength),
                    total_value: 3,
                    single_dice: true,
                    priority: true,
                    consequences: Vec::new(),
                },
                ChallengeBox {
                    dice_type: Some(AttributeType::Strength),
                    total_value: 3,
                    single_dice: true,
                    priority: true,
                    consequences: Vec::new(),
                },
            ],
        );
        combat_challenges.insert(
            3,
            vec![ChallengeBox {
                dice_type: Some(AttributeType::Agility),
                total_value: 12,
                single_dice: false,
                priority: false,
                consequences: vec![Attribute {
                    attribute: AttributeType::Health,
                    quantity: Some(3),
                    ..Default::default()
                }],
            }],
        );

        Dungeon {
            name: String::from("Minotaur's Maze"),
            difficulty: 3,
            peril_challenges,
            combat_challenges,
        }
    }
}
