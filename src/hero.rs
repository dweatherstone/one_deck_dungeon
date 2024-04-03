use std::{
    collections::HashMap,
    fmt::{Display, Error},
};

use crate::encounter::Encounter;

pub struct HeroicFeat {
    pub name: String,
    pub description: String,
    pub encounters: Vec<Encounter>,
}

#[derive(Debug)]
pub enum AttributeType {
    Strength,
    Agility,
    Magic,
    Heroic,
    Health,
    Time,
    Door,
    Potion,
    Value(Vec<i8>),
    Default,
}

impl Display for AttributeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Attribute {
    pub attribute: AttributeType,
    pub quantity: Option<usize>,
    pub value: Option<i8>,
}

impl Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.quantity.is_some() && self.value.is_none() {
            write!(f, "{} x {}", self.quantity.unwrap(), self.attribute)
        } else if self.quantity.is_none() && self.value.is_some() {
            write!(f, "{} value {}", self.attribute, self.value.unwrap())
        } else {
            Err(Error)
        }
    }
}

impl Default for Attribute {
    fn default() -> Self {
        Attribute {
            attribute: AttributeType::Default,
            quantity: None,
            value: None,
        }
    }
}

pub struct Skill {
    pub name: String,
    pub description: Option<String>,
    pub requirements: Option<Attribute>,
    pub effect: Effect,
    pub encounters: Vec<Encounter>,
}

pub enum Effect {
    Gain(Vec<Attribute>),
    Roll(Vec<Attribute>),
    Increase(usize),
    Reroll(AttributeType),
    Change {
        attribute_type: AttributeType,
        value: usize,
    },
    Prevent(AttributeType),
    Discard(AttributeType),
    Heal(usize),
}

impl Display for Effect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Effect::Gain(attributes) => {
                let mut output = String::from("Gain: ");
                let attribute_string = attributes
                    .iter()
                    .map(|at| format!("{}", at).to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                output.push_str(&attribute_string);
                write!(f, "{}", output)
            }
            Effect::Roll(attributes) => {
                let mut output = String::from("Add to pool: ");
                let attribute_string = attributes
                    .iter()
                    .map(|at| format!("{}", at).to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                output.push_str(&attribute_string);
                write!(f, "{}", output)
            }
            Effect::Increase(val) => write!(f, "Increase one dice by {}", val),
            Effect::Reroll(at) => write!(f, "Reroll one {} dice", at),
            Effect::Change {
                attribute_type,
                value,
            } => write!(f, "Set one {} dice to {}", attribute_type, value),
            Effect::Prevent(at) => write!(f, "Prevent: {}", at),
            Effect::Discard(at) => write!(f, "Discard one {} dice", at),
            Effect::Heal(value) => write!(f, "Heal {} damage", value),
        }
    }
}

pub struct Hero {
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub heroic_feat: HeroicFeat,
    pub skill: Vec<Skill>,
    pub levels: HashMap<i8, HashMap<String, i8>>,
    pub current_level: i8,
    pub potions: i8,
    pub encounter_bonus: i8,
}

impl Hero {
    pub fn get_mage() -> Hero {
        Hero {
            name: String::from("Mage"),
            attributes: vec![
                Attribute {
                    attribute: AttributeType::Strength,
                    quantity: Some(1),
                    ..Default::default()
                },
                Attribute{
                    attribute: AttributeType::Agility,
                    quantity: Some(2),
                    ..Default::default()
                },
                Attribute{
                    attribute: AttributeType::Magic,
                    quantity: Some(4),
                    ..Default::default()
                },
                Attribute{
                    attribute: AttributeType::Health,
                    quantity: Some(5),
                    ..Default::default()
                },
            ],
            heroic_feat: HeroicFeat {
                name: String::from("MANA CHARGE"),
                description: String::from("Roll any or all of your dice stored here.\nStore a HEROIC DICE here when you explore or flee. You may store up to two dice at a time."),
                encounters: vec![
                    Encounter::Combat, Encounter::Peril
                ]
            },
            skill: vec![Skill {
                name: String::from("SHIELD AURA"),
                description: Some(String::from("Prevent HEALTH.")),
                requirements: None,
                effect: Effect::Prevent(AttributeType::Health),
                encounters: vec![Encounter::Peril]
            },],
            levels: Self::get_default_levels(),
            current_level: 1,
            potions: 1,
            encounter_bonus: 0,
        }
    }

    pub fn get_caliana() -> Hero {
        Hero {
            name: String::from("Caliana"),
            attributes: vec![
                Attribute {
                    attribute: AttributeType::Strength,
                    quantity: Some(1),
                    ..Default::default()
                },
                Attribute{
                    attribute: AttributeType::Agility,
                    quantity: Some(1),
                    ..Default::default()
                },
                Attribute{
                    attribute: AttributeType::Magic,
                    quantity: Some(5),
                    ..Default::default()
                },
                Attribute{
                    attribute: AttributeType::Health,
                    quantity: Some(0),
                    ..Default::default()
                },
            ],
            heroic_feat: HeroicFeat {
                name: String::from("WHIMSICALITY"),
                description: String::from("Convert 3 damage to time each turn (prevent 4 per boss round). If Caliana would take damage, the game ends."),
                encounters: vec![
                    Encounter::Combat, Encounter::Peril, Encounter::Boss,
                ]
            },
            skill: vec![Skill {
                name: String::from("FAERIE FIRE"),
                description: Some(String::from("Add X x STRENGTH and X x AGILITY.")),
                requirements: Some(Attribute { attribute: AttributeType::Magic, quantity: Some(1), ..Default::default() }),
                effect: Effect::Gain(vec![Attribute{attribute: AttributeType::Strength, quantity: Some(1),
                    ..Default::default()}, Attribute{attribute: AttributeType::Agility, quantity: Some(1),
                        ..Default::default()}]),
                encounters: vec![Encounter::Combat],
            }],
            levels: Self::get_default_levels(),
            current_level: 1,
            potions: 1,
            encounter_bonus: 0,
        }
    }

    pub fn get_paladin() -> Hero {
        Hero {
            name: String::from("Paladin"),
            attributes: vec![
                Attribute {attribute: AttributeType::Strength, quantity: Some(3),
                    ..Default::default()},
                Attribute {attribute: AttributeType::Agility, quantity: Some(1),
                    ..Default::default()},
                Attribute {attribute: AttributeType::Magic, quantity: Some(3),
                    ..Default::default()},
                Attribute {attribute: AttributeType::Health, quantity: Some(5),
                    ..Default::default()},
            ],
            heroic_feat: HeroicFeat {
                name: String::from("VALIANT"),
                description: String::from("Roll any or all of your dice stored here.\nStore a HEROIC DICE here when you open a door with 4+ XP. You may store up to two dice at a time."),
                encounters: vec![Encounter::Combat, Encounter::Peril],
            },
            skill: vec![
                Skill {
                    name: String::from("ARMOR"),
                    description: Some(String::from("For every 2 x HEALTH you would lose, prevent 1 x HEALTH. You cannot prevent damage otherwise.")),
                    requirements: None,
                    effect: Effect::Prevent(AttributeType::Health),
                    encounters: vec![Encounter::Combat, Encounter::Peril],
                }
            ],
            levels: Self::get_default_levels(),
            current_level: 1,
            potions: 1,
            encounter_bonus: 0,
        }
    }

    pub fn get_warrior() -> Hero {
        Hero {
            name: String::from("Warrior"),
            attributes: vec![
                Attribute {attribute: AttributeType::Strength, quantity: Some(4),
                    ..Default::default()},
                Attribute {attribute: AttributeType::Agility, quantity: Some(2),
                    ..Default::default()},
                Attribute {attribute: AttributeType::Magic, quantity: Some(1),
                    ..Default::default()},
                Attribute {attribute: AttributeType::Health, quantity: Some(6),
                    ..Default::default()},
            ],
            heroic_feat: HeroicFeat {
                name: String::from("FRENZY"),
                description: String::from("Roll any or all of your dice stored here.\nStore a HEROIC DICE here for each damage you take. You may store up to two dice at a time."),
                encounters: vec![Encounter::Combat, Encounter::Peril],
            },
            skill: vec![
                Skill {
                    name: String::from("SECOND WIND"),
                    description: Some(String::from("When you descend, heal two damage.")),
                    requirements: None,
                    effect: Effect::Heal(2),
                    encounters: vec![Encounter::Combat, Encounter::Peril, Encounter::Boss],
                }
            ],
            levels: Self::get_default_levels(),
            current_level: 1,
            potions: 1,
            encounter_bonus: 0,
        }
    }

    pub fn get_rogue() -> Hero {
        Hero {
            name: String::from("Rogue"),
            attributes: vec![
                Attribute {attribute: AttributeType::Strength, quantity: Some(1),
                    ..Default::default()},
                Attribute {attribute: AttributeType::Agility, quantity: Some(4),
                    ..Default::default()},
                Attribute {attribute: AttributeType::Magic, quantity: Some(2),
                    ..Default::default()},
                Attribute {attribute: AttributeType::Health, quantity: Some(5),
                    ..Default::default()},
            ],
            heroic_feat: HeroicFeat {
                name: String::from("DARING GAMBLE"),
                description: String::from("Roll one or two MAGIC DICE. If either is a 1, lose 1 x HEALTH and 3 x TIME. Do this before checking any other effects."),
                encounters: vec![Encounter::Combat, Encounter::Peril],
            },
            skill: vec![
                Skill {
                    name: String::from("STEALTH"),
                    description: Some(String::from("When you flee you may add one door to the dungeon, if under the door limit.")),
                    requirements: None,
                    effect: Effect::Gain(vec![Attribute {attribute: AttributeType::Door, quantity: Some(1),
                        ..Default::default()}]),
                    encounters: vec![Encounter::Combat, Encounter::Peril],
                }
            ],
            levels: Self::get_default_levels(),
            current_level: 1,
            potions: 1,
            encounter_bonus: 0,
        }
    }

    pub fn get_archer() -> Hero {
        Hero {
            name: String::from("Archer"),
            attributes: vec![
                Attribute {attribute: AttributeType::Strength, quantity: Some(2),
                    ..Default::default()},
                Attribute {attribute: AttributeType::Agility, quantity: Some(3),
                    ..Default::default()},
                Attribute {attribute: AttributeType::Magic, quantity: Some(2),
                    ..Default::default()},
                Attribute {attribute: AttributeType::Health, quantity: Some(5),
                    ..Default::default()},
            ],
            heroic_feat: HeroicFeat {
                name: String::from("EAGLE EYE"),
                description: String::from("Spend 2 x TIME to roll 2 x HEROIC DICE or 4 x TIME to roll 3 x HEROIC DICE. Before checking any other effects, discard one of the dice rolled."),
                encounters: vec![Encounter::Combat, Encounter::Peril],
            },
            skill: vec![
                Skill {
                    name: String::from("KITING"),
                    description: Some(String::from("If you would lose only one HEALTH, spend TIME instead. Prevent one HEALTH in each boss round")),
                    requirements: None,
                    effect: Effect::Prevent(AttributeType::Health),
                    encounters: vec![Encounter::Combat, Encounter::Boss],
                }
            ],
            levels: Self::get_default_levels(),
            current_level: 1,
            potions: 1,
            encounter_bonus: 0,
        }
    }

    fn get_default_levels() -> HashMap<i8, HashMap<String, i8>> {
        HashMap::from([
            (
                1,
                HashMap::from([
                    (String::from("Items"), 1),
                    (String::from("Skills"), 2),
                    (String::from("Potions"), 1),
                    (String::from("Encounter Bonus"), 0),
                    (String::from("XP to level up"), 6),
                ]),
            ),
            (
                2,
                HashMap::from([
                    (String::from("Items"), 3),
                    (String::from("Skills"), 3),
                    (String::from("Potions"), 1),
                    (String::from("Encounter Bonus"), 1),
                    (String::from("XP to level up"), 8),
                ]),
            ),
            (
                3,
                HashMap::from([
                    (String::from("Items"), 5),
                    (String::from("Skills"), 4),
                    (String::from("Potions"), 1),
                    (String::from("Encounter Bonus"), 1),
                    (String::from("XP to level up"), 10),
                ]),
            ),
            (
                4,
                HashMap::from([
                    (String::from("Items"), 7),
                    (String::from("Skills"), 5),
                    (String::from("Potions"), 1),
                    (String::from("Encounter Bonus"), 2),
                    (String::from("XP to level up"), 5),
                ]),
            ),
        ])
    }
}
