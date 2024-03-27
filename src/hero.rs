use std::fmt::Display;

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
    Health,
}

impl Display for AttributeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Attribute {
    pub attribute: AttributeType,
    pub quantity: usize,
}

impl Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.attribute, self.quantity)
    }
}

pub struct Skill {
    pub name: String,
    pub requirements: Option<Attribute>,
    pub effect: Effect,
    pub encounters: Vec<Encounter>,
}

pub enum Effect {
    Gain {
        attribute_type: AttributeType,
        value: usize,
    },
    Roll(AttributeType),
    Increase(usize),
    Reroll(AttributeType),
    Change {
        attribute_type: AttributeType,
        value: usize,
    },
    Prevent(AttributeType),
    Discard(AttributeType),
}

impl Display for Effect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Effect::Gain {
                attribute_type,
                value,
            } => write!(f, "Gain: {} ({})", attribute_type, value),
            Effect::Roll(at) => write!(f, "Add to pool 1 x {}", at),
            Effect::Increase(val) => write!(f, "Increase one dice by {}", val),
            Effect::Reroll(at) => write!(f, "Reroll one {} dice", at),
            Effect::Change {
                attribute_type,
                value,
            } => write!(f, "Set one {} dice to {}", attribute_type, value),
            Effect::Prevent(at) => write!(f, "Prevent: {}", at),
            Effect::Discard(at) => write!(f, "Discard one {} dice", at),
        }
    }
}

pub struct Hero {
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub heroic_feat: HeroicFeat,
    pub skill: Skill,
}

impl Hero {
    pub fn get_mage() -> Hero {
        Hero {
            name: String::from("Mage"),
            attributes: vec![
                Attribute {
                    attribute: AttributeType::Strength,
                    quantity: 1,
                },
                Attribute{
                    attribute: AttributeType::Agility,
                    quantity: 2,
                },
                Attribute{
                    attribute: AttributeType::Magic,
                    quantity: 4,
                },
                Attribute{
                    attribute: AttributeType::Health,
                    quantity: 5,
                },
            ],
            heroic_feat: HeroicFeat {
                name: String::from("MANA CHARGE"),
                description: String::from("Roll any or all of your dice stored here.\n Store a HEROIC DICE here when you explore or flee. You may store up to two dice at a time."),
                encounters: vec![
                    Encounter::Combat, Encounter::Peril
                ]
            },
            skill: Skill {
                name: String::from("SHIELD AURA"),
                requirements: None,
                effect: Effect::Prevent(AttributeType::Health),
                encounters: vec![Encounter::Peril]
            }
        }
    }
}
