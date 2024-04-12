use crate::{
    dungeon::ChallengeBox,
    encounter::{Combat, Encounter},
    hero::{Attribute, AttributeType, Effect, Skill},
};

pub fn get_all_combats() -> Vec<Combat> {
    vec![
        get_goblin(true),
        get_goblin(false),
        get_ogre(true),
        get_ogre(false),
        get_beetle(true),
        get_beetle(false),
        get_phantom(true),
        get_phantom(false),
        get_bandit(true),
        get_bandit(false),
        get_shadow(true),
        get_shadow(false),
        get_plague_rat(true),
        get_plague_rat(false),
        get_skeleton(true),
        get_skeleton(false),
        get_glooping_ooze(true),
        get_glooping_ooze(false),
        get_ice_elemental(true),
        get_ice_elemental(false),
        get_fire_elemental(true),
        get_fire_elemental(false),
        get_wraith(true),
        get_wraith(false),
    ]
}

fn get_goblin(is_option_one: bool) -> Combat {
    let skill_reward = if is_option_one {
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
    } else {
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
    };
    Combat {
        name: String::from("Goblin"),
        special_ability: Effect::Swarm,
        challenges: vec![
            ChallengeBox {
                dice_type: Some(AttributeType::Strength),
                total_value: 4,
                single_dice: false,
                priority: true,
                consequences: Vec::new(),
            },
            ChallengeBox {
                dice_type: Some(AttributeType::Strength),
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
                dice_type: Some(AttributeType::Agility),
                total_value: 4,
                single_dice: true,
                priority: false,
                consequences: vec![Attribute {
                    attribute: AttributeType::Time,
                    quantity: Some(1),
                    ..Default::default()
                }],
            },
            ChallengeBox {
                dice_type: Some(AttributeType::Agility),
                total_value: 4,
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
            ChallengeBox {
                dice_type: Some(AttributeType::Strength),
                total_value: 5,
                single_dice: true,
                priority: false,
                consequences: vec![Attribute {
                    attribute: AttributeType::Health,
                    quantity: Some(2),
                    ..Default::default()
                }],
            },
        ],
        xp_reward: 2,
        item_reward: vec![Attribute {
            attribute: AttributeType::Strength,
            quantity: Some(1),
            ..Default::default()
        }],
        skill_reward,
    }
}

fn get_ogre(is_option_one: bool) -> Combat {
    let item_reward = if is_option_one {
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
    } else {
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
    };
    let skill_reward = if is_option_one {
        Skill {
            name: String::from("BRUTE FORCE"),
            description: Some(String::from(
                "Discard any number of value 5 dice. Gain that many value 6 STRENGTH dice.",
            )),
            requirements: None,
            effect: Effect::Gain(vec![Attribute {
                attribute: AttributeType::Strength,
                value: Some(6),
                ..Default::default()
            }]),
            encounters: vec![Encounter::Combat, Encounter::Peril, Encounter::Boss],
        }
    } else {
        Skill {
            name: String::from("CHAOTIC AURA"),
            description: Some(String::from(
                "Pick a value. Change up to five of your dice of that value to sixes.",
            )),
            requirements: Some(Attribute {
                attribute: AttributeType::Magic,
                value: Some(6),
                quantity: Some(1),
            }),
            effect: Effect::Change {
                attribute_type: AttributeType::Default,
                value: 6,
            },
            encounters: vec![Encounter::Combat, Encounter::Boss],
        }
    };
    Combat {
        name: String::from("Ogre"),
        special_ability: Effect::None,
        challenges: vec![
            ChallengeBox {
                dice_type: Some(AttributeType::Strength),
                total_value: 6,
                single_dice: false,
                priority: false,
                consequences: vec![Attribute {
                    attribute: AttributeType::Health,
                    quantity: Some(1),
                    ..Default::default()
                }],
            },
            ChallengeBox {
                dice_type: Some(AttributeType::Agility),
                total_value: 4,
                single_dice: true,
                priority: false,
                consequences: vec![Attribute {
                    attribute: AttributeType::Time,
                    quantity: Some(1),
                    ..Default::default()
                }],
            },
            ChallengeBox {
                dice_type: Some(AttributeType::Strength),
                total_value: 9,
                single_dice: false,
                priority: false,
                consequences: vec![Attribute {
                    attribute: AttributeType::Health,
                    quantity: Some(2),
                    ..Default::default()
                }],
            },
            ChallengeBox {
                dice_type: Some(AttributeType::Agility),
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
            ChallengeBox {
                dice_type: Some(AttributeType::Strength),
                total_value: 12,
                single_dice: false,
                priority: false,
                consequences: vec![Attribute {
                    attribute: AttributeType::Health,
                    quantity: Some(3),
                    ..Default::default()
                }],
            },
            ChallengeBox {
                dice_type: Some(AttributeType::Agility),
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
            },
        ],
        xp_reward: 4,
        item_reward,
        skill_reward,
    }
}

fn get_beetle(is_option_one: bool) -> Combat {
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
    Combat {
        name: String::from("Beetle"),
        special_ability: Effect::Survivor,
        challenges: vec![
            ChallengeBox {
                dice_type: Some(AttributeType::Agility),
                total_value: 3,
                single_dice: true,
                priority: true,
                consequences: Vec::new(),
            },
            ChallengeBox {
                dice_type: Some(AttributeType::Agility),
                total_value: 4,
                single_dice: true,
                priority: true,
                consequences: Vec::new(),
            },
            ChallengeBox {
                dice_type: Some(AttributeType::Strength),
                total_value: 5,
                single_dice: true,
                priority: true,
                consequences: Vec::new(),
            },
            ChallengeBox {
                dice_type: Some(AttributeType::Magic),
                total_value: 3,
                single_dice: true,
                priority: true,
                consequences: Vec::new(),
            },
            ChallengeBox {
                dice_type: Some(AttributeType::Agility),
                total_value: 4,
                single_dice: true,
                priority: false,
                consequences: vec![Attribute {
                    attribute: AttributeType::Time,
                    quantity: Some(2),
                    ..Default::default()
                }],
            },
            ChallengeBox {
                dice_type: Some(AttributeType::Strength),
                total_value: 6,
                single_dice: true,
                priority: false,
                consequences: vec![
                    Attribute {
                        attribute: AttributeType::Time,
                        quantity: Some(1),
                        ..Default::default()
                    },
                    Attribute {
                        attribute: AttributeType::Health,
                        quantity: Some(1),
                        ..Default::default()
                    },
                ],
            },
        ],
        xp_reward: 2,
        item_reward: vec![Attribute {
            attribute: AttributeType::Magic,
            quantity: Some(1),
            ..Default::default()
        }],
        skill_reward,
    }
}

fn get_phantom(is_option_one: bool) -> Combat {
    let item_reward = if is_option_one {
        vec![
            Attribute {
                attribute: AttributeType::Agility,
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
    };
    let skill_reward = if is_option_one {
        Skill {
            name: String::from("TRIPLE STRIKE"),
            description: Some(String::from("Gain value 5 STRENGTH, value 5 AGILITY, and value 5 MAGIC dice. Then make one of them a 6.")),
            requirements: Some(Attribute {
                attribute: AttributeType::Strength,
                quantity: Some(3),
                ..Default::default()
            }),
            effect: Effect::Gain(vec![Attribute {
                attribute: AttributeType::Strength,
                quantity: Some(1),
                value: Some(5),
            },
            Attribute {
                attribute: AttributeType::Agility,
                quantity: Some(1),
                value: Some(5),
            },
            Attribute {
                attribute: AttributeType::Magic,
                quantity: Some(1),
                value: Some(5),
            }]),
            encounters: vec![Encounter::Combat, Encounter::Boss],
        }
    } else {
        Skill {
            name: String::from("STEADY HANDS"),
            description: Some(String::from("Roll a HEROIC DICE.")),
            requirements: None,
            effect: Effect::Roll(vec![Attribute {
                attribute: AttributeType::Heroic,
                quantity: Some(1),
                ..Default::default()
            }]),
            encounters: vec![Encounter::Peril, Encounter::Boss],
        }
    };
    Combat {
        name: String::from("Phantom"),
        special_ability: Effect::Ethereal,
        challenges: vec![
            ChallengeBox {
                dice_type: Some(AttributeType::Strength),
                total_value: 4,
                single_dice: true,
                priority: true,
                consequences: Vec::new(),
            },
            ChallengeBox {
                dice_type: Some(AttributeType::Agility),
                total_value: 4,
                single_dice: true,
                priority: true,
                consequences: Vec::new(),
            },
            ChallengeBox {
                dice_type: Some(AttributeType::Agility),
                total_value: 5,
                single_dice: true,
                priority: false,
                consequences: vec![Attribute {
                    attribute: AttributeType::Health,
                    quantity: Some(2),
                    ..Default::default()
                }],
            },
            ChallengeBox {
                dice_type: Some(AttributeType::Agility),
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
            ChallengeBox {
                dice_type: Some(AttributeType::Strength),
                total_value: 6,
                single_dice: true,
                priority: false,
                consequences: vec![Attribute {
                    attribute: AttributeType::Health,
                    quantity: Some(2),
                    ..Default::default()
                }],
            },
        ],
        xp_reward: 4,
        item_reward,
        skill_reward,
    }
}

fn get_bandit(is_option_one: bool) -> Combat {
    let item_reward = if is_option_one {
        vec![Attribute {
            attribute: AttributeType::Agility,
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
    };
    Combat {
        name: String::from("Bandit"),
        special_ability: Effect::Dodge,
        challenges: vec![
            ChallengeBox {
                dice_type: Some(AttributeType::Agility),
                total_value: 8,
                single_dice: false,
                priority: false,
                consequences: vec![Attribute {
                    attribute: AttributeType::Health,
                    quantity: Some(2),
                    ..Default::default()
                }],
            },
            ChallengeBox {
                dice_type: Some(AttributeType::Agility),
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
                dice_type: Some(AttributeType::Agility),
                total_value: 4,
                single_dice: true,
                priority: false,
                consequences: vec![Attribute {
                    attribute: AttributeType::Time,
                    quantity: Some(1),
                    ..Default::default()
                }],
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
        xp_reward: 3,
        item_reward,
        skill_reward,
    }
}

fn get_shadow(is_option_one: bool) -> Combat {
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
        vec![Attribute {
            attribute: AttributeType::Agility,
            quantity: Some(1),
            ..Default::default()
        }]
    };
    let skill_reward = if is_option_one {
        Skill {
            name: String::from("HEROISM"),
            description: Some(String::from(
                "Change two of your non-HEROIC dice to by sixes.",
            )),
            requirements: Some(Attribute {
                attribute: AttributeType::Potion,
                quantity: Some(1),
                ..Default::default()
            }),
            effect: Effect::Change {
                attribute_type: AttributeType::Default,
                value: 6,
            },
            encounters: vec![Encounter::Combat, Encounter::Peril, Encounter::Boss],
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
    Combat {
        name: String::from("Shadow"),
        special_ability: Effect::Fade,
        challenges: vec![
            ChallengeBox {
                dice_type: Some(AttributeType::Agility),
                total_value: 10,
                single_dice: false,
                priority: true,
                consequences: Vec::new(),
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
            ChallengeBox {
                dice_type: Some(AttributeType::Magic),
                total_value: 3,
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
            ChallengeBox {
                dice_type: Some(AttributeType::Magic),
                total_value: 4,
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
            ChallengeBox {
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
            },
        ],
        xp_reward: 3,
        item_reward,
        skill_reward,
    }
}

fn get_plague_rat(is_option_one: bool) -> Combat {
    let skill_reward = if is_option_one {
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
    Combat {
        name: String::from("Plague Rat"),
        special_ability: Effect::Swarm,
        challenges: vec![
            ChallengeBox {
                dice_type: Some(AttributeType::Agility),
                total_value: 4,
                single_dice: false,
                priority: true,
                consequences: Vec::new(),
            },
            ChallengeBox {
                dice_type: Some(AttributeType::Agility),
                total_value: 3,
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
            ChallengeBox {
                dice_type: Some(AttributeType::Strength),
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
            ChallengeBox {
                dice_type: Some(AttributeType::Agility),
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
        xp_reward: 2,
        item_reward: vec![Attribute {
            attribute: AttributeType::Agility,
            quantity: Some(1),
            ..Default::default()
        }],
        skill_reward,
    }
}

fn get_skeleton(is_option_one: bool) -> Combat {
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
    Combat {
        name: String::from("Skeleton"),
        special_ability: Effect::Undying,
        challenges: vec![
            ChallengeBox {
                dice_type: Some(AttributeType::Magic),
                total_value: 2,
                single_dice: true,
                priority: true,
                consequences: Vec::new(),
            },
            ChallengeBox {
                dice_type: Some(AttributeType::Magic),
                total_value: 4,
                single_dice: true,
                priority: true,
                consequences: Vec::new(),
            },
            ChallengeBox {
                dice_type: Some(AttributeType::Strength),
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
            ChallengeBox {
                dice_type: Some(AttributeType::Agility),
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
            ChallengeBox {
                dice_type: Some(AttributeType::Strength),
                total_value: 6,
                single_dice: true,
                priority: false,
                consequences: vec![Attribute {
                    attribute: AttributeType::Time,
                    quantity: Some(2),
                    ..Default::default()
                }],
            },
        ],
        xp_reward: 2,
        item_reward,
        skill_reward,
    }
}

fn get_glooping_ooze(is_option_one: bool) -> Combat {
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
        vec![Attribute {
            attribute: AttributeType::Magic,
            quantity: Some(1),
            ..Default::default()
        }]
    };
    let skill_reward = if is_option_one {
        Skill {
            name: String::from("HEROISM"),
            description: Some(String::from(
                "Change two of your non-HEROIC dice to by sixes.",
            )),
            requirements: Some(Attribute {
                attribute: AttributeType::Potion,
                quantity: Some(1),
                ..Default::default()
            }),
            effect: Effect::Change {
                attribute_type: AttributeType::Default,
                value: 6,
            },
            encounters: vec![Encounter::Combat, Encounter::Peril, Encounter::Boss],
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
    Combat {
        name: String::from("Glooping Ooze"),
        special_ability: Effect::Split,
        challenges: vec![
            ChallengeBox {
                dice_type: Some(AttributeType::Magic),
                total_value: 2,
                single_dice: true,
                priority: true,
                consequences: Vec::new(),
            },
            ChallengeBox {
                dice_type: Some(AttributeType::Magic),
                total_value: 3,
                single_dice: true,
                priority: true,
                consequences: Vec::new(),
            },
            ChallengeBox {
                dice_type: Some(AttributeType::Strength),
                total_value: 4,
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
            ChallengeBox {
                dice_type: Some(AttributeType::Strength),
                total_value: 4,
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
            ChallengeBox {
                dice_type: Some(AttributeType::Magic),
                total_value: 5,
                single_dice: true,
                priority: false,
                consequences: vec![Attribute {
                    attribute: AttributeType::Time,
                    quantity: Some(2),
                    ..Default::default()
                }],
            },
            ChallengeBox {
                dice_type: Some(AttributeType::Strength),
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
            },
        ],
        xp_reward: 3,
        item_reward,
        skill_reward,
    }
}

fn get_ice_elemental(is_option_one: bool) -> Combat {
    let skill_reward = if is_option_one {
        Skill {
            name: String::from("LUCKY FAMILIAR"),
            description: Some(String::from(
                "Reroll one of your dice OR increase one of your dice by 1.",
            )),
            requirements: None,
            effect: Effect::Reroll(AttributeType::Default),
            encounters: vec![Encounter::Combat, Encounter::Peril, Encounter::Boss],
        }
    } else {
        Skill {
            name: String::from("FLURRY"),
            description: Some(String::from("Roll 2 x AGILITY, 1 x STRENGTH dice.")),
            requirements: Some(Attribute {
                attribute: AttributeType::Strength,
                quantity: Some(2),
                ..Default::default()
            }),
            effect: Effect::Roll(vec![
                Attribute {
                    attribute: AttributeType::Agility,
                    quantity: Some(2),
                    ..Default::default()
                },
                Attribute {
                    attribute: AttributeType::Strength,
                    quantity: Some(1),
                    ..Default::default()
                },
            ]),
            encounters: vec![Encounter::Combat, Encounter::Boss],
        }
    };
    Combat {
        name: String::from("Ice Elemental"),
        special_ability: Effect::Frost,
        challenges: vec![
            ChallengeBox {
                dice_type: Some(AttributeType::Strength),
                total_value: 11,
                single_dice: false,
                priority: true,
                consequences: Vec::new(),
            },
            ChallengeBox {
                dice_type: Some(AttributeType::Strength),
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
                    attribute: AttributeType::Health,
                    quantity: Some(2),
                    ..Default::default()
                }],
            },
            ChallengeBox {
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
            },
            ChallengeBox {
                dice_type: Some(AttributeType::Magic),
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
            },
        ],
        xp_reward: 4,
        item_reward: vec![
            Attribute {
                attribute: AttributeType::Agility,
                quantity: Some(1),
                ..Default::default()
            },
            Attribute {
                attribute: AttributeType::Health,
                quantity: Some(1),
                ..Default::default()
            },
        ],
        skill_reward,
    }
}

fn get_fire_elemental(is_option_one: bool) -> Combat {
    let item_reward = if is_option_one {
        vec![
            Attribute {
                attribute: AttributeType::Agility,
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
            name: String::from("MANA FONT"),
            description: Some(String::from(
                "Choose MAGIC, STRENGTH, or AGILITY. Increase all of your dice of that color by 1.",
            )),
            requirements: Some(Attribute {
                attribute: AttributeType::Magic,
                quantity: Some(1),
                value: Some(2),
            }),
            effect: Effect::Increase(1),
            encounters: vec![Encounter::Combat, Encounter::Boss],
        }
    } else {
        Skill {
            name: String::from("CONSISTENCY"),
            description: Some(String::from("Change any or all of your dice to 4s.")),
            requirements: Some(Attribute {
                attribute: AttributeType::Agility,
                quantity: Some(1),
                ..Default::default()
            }),
            effect: Effect::Change {
                attribute_type: AttributeType::Default,
                value: 4,
            },
            encounters: vec![Encounter::Combat, Encounter::Boss],
        }
    };
    Combat {
        name: String::from("Fire Elemental"),
        special_ability: Effect::Flames,
        challenges: vec![
            ChallengeBox {
                dice_type: Some(AttributeType::Magic),
                total_value: 3,
                single_dice: false,
                priority: false,
                consequences: vec![Attribute {
                    attribute: AttributeType::Time,
                    quantity: Some(1),
                    ..Default::default()
                }],
            },
            ChallengeBox {
                dice_type: Some(AttributeType::Agility),
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
            },
            ChallengeBox {
                dice_type: Some(AttributeType::Magic),
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
                        quantity: Some(1),
                        ..Default::default()
                    },
                ],
            },
            ChallengeBox {
                dice_type: Some(AttributeType::Agility),
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
            },
        ],
        xp_reward: 4,
        item_reward,
        skill_reward,
    }
}

fn get_wraith(is_option_one: bool) -> Combat {
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
    Combat {
        name: String::from("Wraith"),
        special_ability: Effect::Drain,
        challenges: vec![
            ChallengeBox {
                dice_type: Some(AttributeType::Magic),
                total_value: 9,
                single_dice: false,
                priority: true,
                consequences: Vec::new(),
            },
            ChallengeBox {
                dice_type: Some(AttributeType::Strength),
                total_value: 5,
                single_dice: true,
                priority: true,
                consequences: Vec::new(),
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
            ChallengeBox {
                dice_type: Some(AttributeType::Strength),
                total_value: 5,
                single_dice: true,
                priority: false,
                consequences: vec![Attribute {
                    attribute: AttributeType::Health,
                    quantity: Some(2),
                    ..Default::default()
                }],
            },
            ChallengeBox {
                dice_type: Some(AttributeType::Magic),
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
            },
        ],
        xp_reward: 3,
        item_reward: vec![Attribute {
            attribute: AttributeType::Strength,
            quantity: Some(1),
            ..Default::default()
        }],
        skill_reward,
    }
}
