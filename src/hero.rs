use std::{
    cmp::Ordering,
    collections::{BTreeMap, HashMap},
    fmt::{Display, Error},
};

use crate::encounter::Encounter;

pub struct HeroicFeat {
    pub name: String,
    pub description: String,
    pub encounters: Vec<Encounter>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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

impl PartialOrd for AttributeType {
    fn partial_cmp(&self, other: &AttributeType) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AttributeType {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            return Ordering::Equal;
        }
        match self {
            AttributeType::Strength => Ordering::Less,
            AttributeType::Default => Ordering::Greater,
            AttributeType::Agility => {
                if other == &AttributeType::Strength {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
            AttributeType::Magic => {
                if other == &AttributeType::Strength || other == &AttributeType::Agility {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
            AttributeType::Health => {
                if other == &AttributeType::Strength
                    || other == &AttributeType::Agility
                    || other == &AttributeType::Magic
                {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
            _ => self.to_string().cmp(&other.to_string()),
        }
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
    Skip(Attribute),
    Value(i8, AttributeType),
    Survivor,
    Ethereal,
    Dodge,
    Fade,
    Swarm,
    Undying,
    Split,
    Frost,
    Flames,
    Drain,
    None,
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
            Effect::Skip(value) => write!(f, "Skip to the Claim Loot phase for {}", value),
            Effect::Value(x, attr) => write!(f, "X = {} per {}", x, attr),
            Effect::Survivor => write!(
                f,
                "Survivor: If any armor boxes are empty, discard this instead of looting."
            ),
            Effect::Ethereal => write!(f, "Ethereal: Immediately discard all 1s and 3s rolled."),
            Effect::Dodge => write!(
                f,
                "Dodge: Making an HEROIC dice usees three dice instead of two."
            ),
            Effect::Fade => write!(f, "Fade: Spend 1 x TIME for each skill you use."),
            Effect::Swarm => write!(f, "Swarm: X = 4 per open door, including this one."),
            Effect::Undying => write!(f, "Undying: If any boxes are empty, spend 2 x TIME."),
            Effect::Split => write!(f, "Split: Spend 1 x TIME for each 1 rolled."),
            Effect::Frost => write!(f, "Frost: Before the encounter, spend 3 x TIME."),
            Effect::Flames => write!(
                f,
                "Flames: Before the encounter, place 1 x HEALTH on a hero."
            ),
            Effect::Drain => write!(f, "Drain: Before the encounter, convert one item to XP."),
            Effect::None => write!(f, "No effect"),
        }
    }
}

type Result<T> = std::result::Result<T, HeroError>;

#[derive(Debug, Clone)]
pub enum HeroError {
    AttributeNotFound,
    QuantityNegative,
    ValueNotFound,
    DuplicateSkill,
    LevelTooHigh,
}

impl Display for HeroError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error updating the Hero stats")
    }
}

// #[derive(Debug, Clone)]
// pub struct AttributeNotFoundError;
// #[derive(Debug, Clone)]
// pub struct QuantityNegativeError;
// #[derive(Debug, Clone)]
// pub struct ValueNotFoundError;

pub struct Hero {
    pub name: String,
    pub attributes: BTreeMap<AttributeType, Attribute>,
    pub heroic_feat: HeroicFeat,
    pub skills: Vec<Skill>,
    pub levels: HashMap<i8, HashMap<String, i8>>,
    pub current_level: i8,
    pub potions: i8,
    pub encounter_bonus: i8,
}

impl Hero {
    pub fn change_attribute_quantity(
        &mut self,
        attribute: AttributeType,
        change_by: i8,
    ) -> Result<usize> {
        let attr = self
            .attributes
            .get(&attribute)
            .ok_or(HeroError::AttributeNotFound)?;
        let quantity = attr.quantity.unwrap_or(0) as i8 + change_by;
        if quantity < 0 {
            return Err(HeroError::QuantityNegative);
        }
        let quantity = quantity as usize;
        let new_attr = Attribute {
            attribute: attribute.clone(),
            quantity: Some(quantity),
            value: attr.value,
        };
        self.attributes
            .insert(attribute.clone(), new_attr)
            .ok_or(HeroError::AttributeNotFound)?;

        Ok(quantity)
    }

    pub fn add_skill(&mut self, new_skill: Skill) -> Result<()> {
        for existing_skill in self.skills.iter() {
            if existing_skill.name == new_skill.name {
                return Err(HeroError::DuplicateSkill);
            }
        }
        self.skills.push(new_skill);
        Ok(())
    }

    pub fn descend_level(&mut self) -> Result<i8> {
        if self.current_level + 1 > *self.levels.keys().max().unwrap_or(&1) {
            return Err(HeroError::LevelTooHigh);
        } else {
            self.current_level += 1;
            self.potions += 1;
            self.encounter_bonus = *self
                .levels
                .get(&self.current_level)
                .ok_or(HeroError::AttributeNotFound)?
                .get("Encounter Bonus")
                .ok_or(HeroError::AttributeNotFound)?;
        }
        Ok(self.current_level)
    }

    pub fn get_mage() -> Hero {
        Hero {
            name: String::from("Mage"),
            attributes: BTreeMap::from([
                (AttributeType::Strength, Attribute {
                    attribute: AttributeType::Strength,
                    quantity: Some(1),
                    ..Default::default()
                }),
                (AttributeType::Agility, Attribute{
                    attribute: AttributeType::Agility,
                    quantity: Some(2),
                    ..Default::default()
                }),
                (AttributeType::Magic, Attribute{
                    attribute: AttributeType::Magic,
                    quantity: Some(4),
                    ..Default::default()
                }),
                (AttributeType::Health, Attribute{
                    attribute: AttributeType::Health,
                    quantity: Some(5),
                    ..Default::default()
                }),
            ]),
            heroic_feat: HeroicFeat {
                name: String::from("MANA CHARGE"),
                description: String::from("Roll any or all of your dice stored here.\nStore a HEROIC DICE here when you explore or flee. You may store up to two dice at a time."),
                encounters: vec![
                    Encounter::Combat, Encounter::Peril
                ]
            },
            skills: vec![Skill {
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
            attributes: BTreeMap::from([
                (AttributeType::Strength, Attribute {
                    attribute: AttributeType::Strength,
                    quantity: Some(1),
                    ..Default::default()
                }),
                (AttributeType::Agility, Attribute{
                    attribute: AttributeType::Agility,
                    quantity: Some(1),
                    ..Default::default()
                }),
                (AttributeType::Magic, Attribute{
                    attribute: AttributeType::Magic,
                    quantity: Some(5),
                    ..Default::default()
                }),
                (AttributeType::Health, Attribute{
                    attribute: AttributeType::Health,
                    quantity: Some(0),
                    ..Default::default()
                }),
            ]),
            heroic_feat: HeroicFeat {
                name: String::from("WHIMSICALITY"),
                description: String::from("Convert 3 damage to time each turn (prevent 4 per boss round). If Caliana would take damage, the game ends."),
                encounters: vec![
                    Encounter::Combat, Encounter::Peril, Encounter::Boss,
                ]
            },
            skills: vec![Skill {
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
            attributes: BTreeMap::from([
                (AttributeType::Strength, Attribute {attribute: AttributeType::Strength, quantity: Some(3),
                    ..Default::default()}),
                (AttributeType::Agility, Attribute {attribute: AttributeType::Agility, quantity: Some(1),
                    ..Default::default()}),
                (AttributeType::Magic, Attribute {attribute: AttributeType::Magic, quantity: Some(3),
                    ..Default::default()}),
                (AttributeType::Health, Attribute {attribute: AttributeType::Health, quantity: Some(5),
                    ..Default::default()}),
            ]),
            heroic_feat: HeroicFeat {
                name: String::from("VALIANT"),
                description: String::from("Roll any or all of your dice stored here.\nStore a HEROIC DICE here when you open a door with 4+ XP. You may store up to two dice at a time."),
                encounters: vec![Encounter::Combat, Encounter::Peril],
            },
            skills: vec![
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
            attributes: BTreeMap::from([
                (AttributeType::Strength, Attribute {attribute: AttributeType::Strength, quantity: Some(4),
                    ..Default::default()}),
                (AttributeType::Agility, Attribute {attribute: AttributeType::Agility, quantity: Some(2),
                    ..Default::default()}),
                (AttributeType::Magic, Attribute {attribute: AttributeType::Magic, quantity: Some(1),
                    ..Default::default()}),
                (AttributeType::Health, Attribute {attribute: AttributeType::Health, quantity: Some(6),
                    ..Default::default()}),
            ]),
            heroic_feat: HeroicFeat {
                name: String::from("FRENZY"),
                description: String::from("Roll any or all of your dice stored here.\nStore a HEROIC DICE here for each damage you take. You may store up to two dice at a time."),
                encounters: vec![Encounter::Combat, Encounter::Peril],
            },
            skills: vec![
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
            attributes: BTreeMap::from([
                (AttributeType::Strength, Attribute {attribute: AttributeType::Strength, quantity: Some(1),
                    ..Default::default()}),
                (AttributeType::Agility, Attribute {attribute: AttributeType::Agility, quantity: Some(4),
                    ..Default::default()}),
                (AttributeType::Magic, Attribute {attribute: AttributeType::Magic, quantity: Some(2),
                    ..Default::default()}),
                (AttributeType::Health, Attribute {attribute: AttributeType::Health, quantity: Some(5),
                    ..Default::default()}),
            ]),
            heroic_feat: HeroicFeat {
                name: String::from("DARING GAMBLE"),
                description: String::from("Roll one or two MAGIC DICE. If either is a 1, lose 1 x HEALTH and 3 x TIME. Do this before checking any other effects."),
                encounters: vec![Encounter::Combat, Encounter::Peril],
            },
            skills: vec![
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
            attributes: BTreeMap::from([
                (AttributeType::Strength, Attribute {attribute: AttributeType::Strength, quantity: Some(2),
                    ..Default::default()}),
                (AttributeType::Agility, Attribute {attribute: AttributeType::Agility, quantity: Some(3),
                    ..Default::default()}),
                (AttributeType::Magic, Attribute {attribute: AttributeType::Magic, quantity: Some(2),
                    ..Default::default()}),
                (AttributeType::Health, Attribute {attribute: AttributeType::Health, quantity: Some(5),
                    ..Default::default()}),
            ]),
            heroic_feat: HeroicFeat {
                name: String::from("EAGLE EYE"),
                description: String::from("Spend 2 x TIME to roll 2 x HEROIC DICE or 4 x TIME to roll 3 x HEROIC DICE. Before checking any other effects, discard one of the dice rolled."),
                encounters: vec![Encounter::Combat, Encounter::Peril],
            },
            skills: vec![
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
