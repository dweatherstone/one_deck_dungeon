use crate::{
    dungeon::ChallengeBox,
    encounter::{Encounter, Peril},
    hero::{Attribute, AttributeType, Effect, Skill},
};

pub fn get_all_perils() -> Vec<Peril> {
    vec![
        get_rune_puzzle(true),
        get_rune_puzzle(false),
        get_locked_door(true),
        get_locked_door(false),
        get_cave_in(true),
        get_cave_in(false),
        get_boulder(true),
        get_boulder(false),
        get_arrow_wall(true),
        get_arrow_wall(false),
        get_flame_statues(true),
        get_flame_statues(false),
        get_spiked_log(true),
        get_spiked_log(false),
        get_bear_traps(true),
        get_bear_traps(false),
        get_pit_of_spikes(true),
        get_pit_of_spikes(false),
        get_force_wall(true),
        get_force_wall(false),
    ]
}

fn get_rune_puzzle(is_option_one: bool) -> Peril {
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

fn get_locked_door(is_option_one: bool) -> Peril {
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
                quantity: Some(1),
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

fn get_cave_in(is_option_one: bool) -> Peril {
    let item_reward = if is_option_one {
        vec![Attribute {
            attribute: AttributeType::Magic,
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
            name: String::from("CRUSHING BLOW"),
            description: Some(String::from("Gain a STRENGTH value 6")),
            requirements: Some(Attribute {
                attribute: AttributeType::Strength,
                quantity: Some(1),
                ..Default::default()
            }),
            effect: Effect::Gain(vec![Attribute {
                attribute: AttributeType::Strength,
                quantity: Some(1),
                value: Some(6),
            }]),
            encounters: vec![Encounter::Combat, Encounter::Boss],
        }
    } else {
        Skill {
            name: String::from("FLAMEWEAVE"),
            description: Some(String::from("Gain a MAGIC value 5")),
            requirements: Some(Attribute {
                attribute: AttributeType::Magic,
                quantity: Some(1),
                value: Some(2),
            }),
            effect: Effect::Gain(vec![Attribute {
                attribute: AttributeType::Magic,
                quantity: Some(1),
                value: Some(5),
            }]),
            encounters: vec![Encounter::Combat, Encounter::Peril, Encounter::Boss],
        }
    };

    Peril {
        name: String::from("Cave-in"),
        choice_one: vec![ChallengeBox {
            dice_type: Some(AttributeType::Strength),
            single_dice: false,
            total_value: 6,
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
            dice_type: Some(AttributeType::Agility),
            single_dice: false,
            total_value: 11,
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

fn get_boulder(is_option_one: bool) -> Peril {
    let item_reward = if is_option_one {
        vec![
            Attribute {
                attribute: AttributeType::Strength,
                quantity: Some(1),
                ..Default::default()
            },
            Attribute {
                attribute: AttributeType::Health,
                quantity: Some(1),
                ..Default::default()
            },
        ]
    } else {
        vec![
            Attribute {
                attribute: AttributeType::Magic,
                quantity: Some(1),
                ..Default::default()
            },
            Attribute {
                attribute: AttributeType::Health,
                quantity: Some(1),
                ..Default::default()
            },
        ]
    };
    let skill_reward = if is_option_one {
        Skill {
            name: String::from("CRITICAL STRIKES"),
            description: Some(String::from(
                "Change one or two AGILITY dice into HEROIC dice of the same values.",
            )),
            requirements: None,
            effect: Effect::Change {
                attribute_type: AttributeType::Agility,
                value: 2,
            },
            encounters: vec![Encounter::Combat, Encounter::Boss],
        }
    } else {
        Skill {
            name: String::from("VALOR"),
            description: Some(String::from("Roll an HEROIC dice.")),
            requirements: None,
            effect: Effect::Roll(vec![Attribute {
                attribute: AttributeType::Heroic,
                quantity: Some(1),
                ..Default::default()
            }]),
            encounters: vec![Encounter::Combat, Encounter::Boss],
        }
    };

    Peril {
        name: String::from("Boulder"),
        choice_one: vec![ChallengeBox {
            dice_type: Some(AttributeType::Magic),
            single_dice: false,
            total_value: 11,
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
        choice_one_time_cost: Some(3),
        choice_two: vec![ChallengeBox {
            dice_type: Some(AttributeType::Agility),
            single_dice: false,
            total_value: 14,
            priority: false,
            consequences: vec![
                Attribute {
                    attribute: AttributeType::Health,
                    quantity: Some(4),
                    ..Default::default()
                },
                Attribute {
                    attribute: AttributeType::Time,
                    quantity: Some(1),
                    ..Default::default()
                },
            ],
        }],
        choice_two_time_cost: None,
        xp_reward: 4,
        item_reward,
        skill_reward,
    }
}

fn get_arrow_wall(is_option_one: bool) -> Peril {
    let item_reward = if is_option_one {
        vec![Attribute {
            attribute: AttributeType::Magic,
            quantity: Some(1),
            ..Default::default()
        }]
    } else {
        vec![Attribute {
            attribute: AttributeType::Strength,
            quantity: Some(1),
            ..Default::default()
        }]
    };
    let skill_reward = if is_option_one {
        Skill {
            name: String::from("MANA"),
            description: Some(String::from("Roll 3 x MAGIC dice")),
            requirements: Some(Attribute {
                attribute: AttributeType::Potion,
                quantity: Some(1),
                ..Default::default()
            }),
            effect: Effect::Roll(vec![Attribute {
                attribute: AttributeType::Magic,
                quantity: Some(3),
                ..Default::default()
            }]),
            encounters: vec![Encounter::Combat, Encounter::Peril, Encounter::Boss],
        }
    } else {
        Skill {
            name: String::from("DODGE"),
            description: Some(String::from(
                "Prevent 1 x HEALTH. In a boss fight prevent 2 x HEALTH.",
            )),
            requirements: Some(Attribute {
                attribute: AttributeType::Agility,
                quantity: Some(1),
                ..Default::default()
            }),
            effect: Effect::Prevent(AttributeType::Health),
            encounters: vec![Encounter::Combat, Encounter::Boss],
        }
    };

    Peril {
        name: String::from("Arrow Wall"),
        choice_one: vec![ChallengeBox {
            dice_type: Some(AttributeType::Magic),
            single_dice: false,
            total_value: 6,
            priority: false,
            consequences: vec![
                Attribute {
                    attribute: AttributeType::Health,
                    quantity: Some(2),
                    ..Default::default()
                },
                Attribute {
                    attribute: AttributeType::Time,
                    quantity: Some(3),
                    ..Default::default()
                },
            ],
        }],
        choice_one_time_cost: Some(1),
        choice_two: vec![ChallengeBox {
            dice_type: Some(AttributeType::Agility),
            single_dice: false,
            total_value: 11,
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

fn get_flame_statues(is_option_one: bool) -> Peril {
    let item_reward = if is_option_one {
        vec![Attribute {
            attribute: AttributeType::Magic,
            quantity: Some(1),
            ..Default::default()
        }]
    } else {
        vec![Attribute {
            attribute: AttributeType::Strength,
            quantity: Some(1),
            ..Default::default()
        }]
    };
    let skill_reward = if is_option_one {
        Skill {
            name: String::from("BACKSTAB"),
            description: Some(String::from("Roll 2 x STRENGTH dice.")),
            requirements: Some(Attribute {
                attribute: AttributeType::Agility,
                quantity: Some(1),
                ..Default::default()
            }),
            effect: Effect::Roll(vec![Attribute {
                attribute: AttributeType::Strength,
                quantity: Some(2),
                ..Default::default()
            }]),
            encounters: vec![Encounter::Combat, Encounter::Boss],
        }
    } else {
        Skill {
            name: String::from("HASTE"),
            description: Some(String::from("Roll 2 x AGILITY dice.")),
            requirements: Some(Attribute {
                attribute: AttributeType::Magic,
                quantity: Some(1),
                value: Some(3),
            }),
            effect: Effect::Gain(vec![Attribute {
                attribute: AttributeType::Agility,
                quantity: Some(2),
                ..Default::default()
            }]),
            encounters: vec![Encounter::Combat, Encounter::Boss],
        }
    };

    Peril {
        name: String::from("Flame Statues"),
        choice_one: vec![ChallengeBox {
            dice_type: Some(AttributeType::Magic),
            single_dice: false,
            total_value: 8,
            priority: false,
            consequences: vec![
                Attribute {
                    attribute: AttributeType::Health,
                    quantity: Some(2),
                    ..Default::default()
                },
                Attribute {
                    attribute: AttributeType::Time,
                    quantity: Some(3),
                    ..Default::default()
                },
            ],
        }],
        choice_one_time_cost: Some(3),
        choice_two: vec![ChallengeBox {
            dice_type: Some(AttributeType::Agility),
            single_dice: false,
            total_value: 14,
            priority: false,
            consequences: vec![
                Attribute {
                    attribute: AttributeType::Health,
                    quantity: Some(3),
                    ..Default::default()
                },
                Attribute {
                    attribute: AttributeType::Time,
                    quantity: Some(1),
                    ..Default::default()
                },
            ],
        }],
        choice_two_time_cost: None,
        xp_reward: 3,
        item_reward,
        skill_reward,
    }
}

fn get_spiked_log(is_option_one: bool) -> Peril {
    let skill_reward = if is_option_one {
        Skill {
            name: String::from("CLEAVE"),
            description: Some(String::from("Increase up to four of your dice by 1 each.")),
            requirements: Some(Attribute {
                attribute: AttributeType::Strength,
                quantity: Some(1),
                ..Default::default()
            }),
            effect: Effect::Increase(4),
            encounters: vec![Encounter::Combat, Encounter::Boss],
        }
    } else {
        Skill {
            name: String::from("STATIC BURST"),
            description: Some(String::from("Gain a value 4 STRENGTH and a value 4 AGILITY. Then, increase one of your dice by 1.")),
            requirements: Some(Attribute {
                attribute: AttributeType::Magic,
                quantity: Some(1),
                value: Some(4),
            }),
            effect: Effect::Gain(vec![Attribute {
                attribute: AttributeType::Strength,
                quantity: Some(1),
                value: Some(4),
            }, Attribute {
                attribute: AttributeType::Agility,
                quantity: Some(1),
                value: Some(4),
            }]),
            encounters: vec![Encounter::Combat, Encounter::Boss],
        }
    };

    Peril {
        name: String::from("Spiked Log"),
        choice_one: vec![ChallengeBox {
            dice_type: Some(AttributeType::Strength),
            single_dice: false,
            total_value: 8,
            priority: false,
            consequences: vec![
                Attribute {
                    attribute: AttributeType::Health,
                    quantity: Some(3),
                    ..Default::default()
                },
                Attribute {
                    attribute: AttributeType::Time,
                    quantity: Some(1),
                    ..Default::default()
                },
            ],
        }],
        choice_one_time_cost: Some(2),
        choice_two: vec![ChallengeBox {
            dice_type: Some(AttributeType::Agility),
            single_dice: false,
            total_value: 14,
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
        xp_reward: 3,
        item_reward: vec![Attribute {
            attribute: AttributeType::Agility,
            quantity: Some(1),
            ..Default::default()
        }],
        skill_reward,
    }
}

fn get_bear_traps(is_option_one: bool) -> Peril {
    let item_reward = if is_option_one {
        vec![Attribute {
            attribute: AttributeType::Magic,
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
            name: String::from("MANA"),
            description: Some(String::from("Roll 3 x MAGIC dice.")),
            requirements: Some(Attribute {
                attribute: AttributeType::Potion,
                quantity: Some(1),
                ..Default::default()
            }),
            effect: Effect::Roll(vec![Attribute {
                attribute: AttributeType::Magic,
                quantity: Some(3),
                ..Default::default()
            }]),
            encounters: vec![Encounter::Combat, Encounter::Peril, Encounter::Boss],
        }
    } else {
        Skill {
            name: String::from("INVISIBILITY"),
            description: Some(String::from(
                "Spend 2 x TIME before an encounter. Skip to the Claim Loot phase.",
            )),
            requirements: Some(Attribute {
                attribute: AttributeType::Potion,
                quantity: Some(1),
                ..Default::default()
            }),
            effect: Effect::Skip(Attribute {
                attribute: AttributeType::Time,
                quantity: Some(2),
                ..Default::default()
            }),
            encounters: vec![Encounter::Combat, Encounter::Peril, Encounter::Boss],
        }
    };

    Peril {
        name: String::from("Bear Traps"),
        choice_one: vec![ChallengeBox {
            dice_type: Some(AttributeType::Agility),
            single_dice: false,
            total_value: 6,
            priority: false,
            consequences: vec![
                Attribute {
                    attribute: AttributeType::Health,
                    quantity: Some(2),
                    ..Default::default()
                },
                Attribute {
                    attribute: AttributeType::Time,
                    quantity: Some(1),
                    ..Default::default()
                },
            ],
        }],
        choice_one_time_cost: Some(3),
        choice_two: vec![ChallengeBox {
            dice_type: Some(AttributeType::Agility),
            single_dice: false,
            total_value: 11,
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

fn get_pit_of_spikes(is_option_one: bool) -> Peril {
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
            name: String::from("DEXTERITY"),
            description: Some(String::from("Gain a value 6 AGILITY dice")),
            requirements: Some(Attribute {
                attribute: AttributeType::Agility,
                quantity: Some(1),
                ..Default::default()
            }),
            effect: Effect::Gain(vec![Attribute {
                attribute: AttributeType::Agility,
                value: Some(6),
                quantity: Some(1),
            }]),
            encounters: vec![Encounter::Combat, Encounter::Boss],
        }
    } else {
        Skill {
            name: String::from("ARMOR CRUSH"),
            description: Some(String::from(
                "Gain a value 6 HEROIC dice. You can only use it to cover a box with PRIORITY.",
            )),
            requirements: Some(Attribute {
                attribute: AttributeType::Strength,
                quantity: Some(1),
                ..Default::default()
            }),
            effect: Effect::Gain(vec![Attribute {
                attribute: AttributeType::Heroic,
                quantity: Some(1),
                value: Some(6),
            }]),
            encounters: vec![Encounter::Combat, Encounter::Boss],
        }
    };

    Peril {
        name: String::from("Pit of Spikes"),
        choice_one: vec![ChallengeBox {
            dice_type: Some(AttributeType::Strength),
            single_dice: false,
            total_value: 8,
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
        choice_one_time_cost: Some(3),
        choice_two: vec![ChallengeBox {
            dice_type: Some(AttributeType::Agility),
            single_dice: false,
            total_value: 14,
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
        xp_reward: 3,
        item_reward,
        skill_reward,
    }
}

fn get_force_wall(is_option_one: bool) -> Peril {
    let item_reward = if is_option_one {
        vec![
            Attribute {
                attribute: AttributeType::Strength,
                quantity: Some(1),
                ..Default::default()
            },
            Attribute {
                attribute: AttributeType::Health,
                quantity: Some(1),
                ..Default::default()
            },
        ]
    } else {
        vec![
            Attribute {
                attribute: AttributeType::Magic,
                quantity: Some(1),
                ..Default::default()
            },
            Attribute {
                attribute: AttributeType::Health,
                quantity: Some(1),
                ..Default::default()
            },
        ]
    };
    let skill_reward = if is_option_one {
        Skill {
            name: String::from("CRUSHING FIST"),
            description: Some(String::from("Gain 2 x STRENGTH with value of 6.")),
            requirements: Some(Attribute {
                attribute: AttributeType::Magic,
                quantity: Some(1),
                value: Some(6),
            }),
            effect: Effect::Gain(vec![Attribute {
                attribute: AttributeType::Strength,
                quantity: Some(2),
                value: Some(6),
            }]),
            encounters: vec![Encounter::Combat, Encounter::Boss],
        }
    } else {
        Skill {
            name: String::from("POISON"),
            description: Some(String::from("Prevent up to 2 x TIME.")),
            requirements: Some(Attribute {
                attribute: AttributeType::Agility,
                quantity: Some(1),
                ..Default::default()
            }),
            effect: Effect::Prevent(AttributeType::Time),
            encounters: vec![Encounter::Combat, Encounter::Boss],
        }
    };

    Peril {
        name: String::from("Force Wall"),
        choice_one: vec![ChallengeBox {
            dice_type: Some(AttributeType::Agility),
            single_dice: false,
            total_value: 11,
            priority: false,
            consequences: vec![
                Attribute {
                    attribute: AttributeType::Health,
                    quantity: Some(2),
                    ..Default::default()
                },
                Attribute {
                    attribute: AttributeType::Time,
                    quantity: Some(4),
                    ..Default::default()
                },
            ],
        }],
        choice_one_time_cost: Some(3),
        choice_two: vec![ChallengeBox {
            dice_type: Some(AttributeType::Magic),
            single_dice: false,
            total_value: 14,
            priority: false,
            consequences: vec![
                Attribute {
                    attribute: AttributeType::Health,
                    quantity: Some(4),
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
        xp_reward: 4,
        item_reward,
        skill_reward,
    }
}
