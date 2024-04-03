use strum::IntoEnumIterator;

use crate::{dungeon::Dungeon, encounter::Encounter, hero::Hero};

const CARD_WIDTH: usize = 50;

pub fn print_hero(hero: &Hero) {
    let mut output = String::new();
    output.push_str(&get_card_top_or_bottom());
    // Name
    output.push_str(&surround_with_edge(&hero.name.to_uppercase()));

    // Attributes
    for attribute in &hero.attributes {
        output.push_str(&surround_with_edge(&format!("{}", attribute)));
    }
    output.push_str(&surround_with_edge(""));

    // Heroic Feat
    output.push_str(&surround_with_edge(&format!(
        "Heroic Feat: {}",
        hero.heroic_feat.name
    )));
    let mut feat_description = String::new();
    let mut col_counter = 0;
    for mut word in hero.heroic_feat.description.split(' ') {
        if col_counter + word.len() >= CARD_WIDTH {
            split_line_long_description(&mut output, &mut feat_description, &mut col_counter);
        }
        if let Some(pos) = word.find('\n') {
            feat_description += &word[..pos];
            split_line_long_description(&mut output, &mut feat_description, &mut col_counter);
            word = &word[pos + 1..];
        }
        feat_description.push_str(word);
        feat_description.push(' ');
        col_counter += word.len() + 1;
    }
    output.push_str(&surround_with_edge(&feat_description));
    let heroic_feat_encounters = Encounter::iter()
        .map(|encounter| {
            if hero.heroic_feat.encounters.contains(&encounter) {
                encounter.to_string()
            } else {
                format!("Not {}", encounter)
            }
        })
        .collect::<Vec<String>>()
        .join(", ");
    output.push_str(&surround_with_edge(&heroic_feat_encounters));
    output.push_str(&surround_with_edge(""));

    // Skill
    for skill in hero.skill.iter() {
        output.push_str(&surround_with_edge(&format!("Skill: {}", skill.name)));
        if skill.description.is_some() {
            output.push_str(&surround_with_edge(skill.description.as_ref().unwrap()));
        }

        if skill.requirements.is_some() {
            output.push_str(&surround_with_edge(&format!(
                "Requirements: {}",
                skill.requirements.as_ref().unwrap()
            )));
        } else {
            output.push_str(&surround_with_edge("Requirements: Free skill"));
        }
        output.push_str(&surround_with_edge(&format!("{}", skill.effect)));
        let skill_encounters = skill
            .encounters
            .iter()
            .map(|encounter| encounter.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        output.push_str(&surround_with_edge(&skill_encounters));
    }
    output.push_str(&get_card_top_or_bottom());
    println!("{}", output);
}

pub fn print_dungeon(dungeon: &Dungeon) {
    let mut output = String::new();
    output.push_str(&get_card_top_or_bottom());
    // Name
    output.push_str(&surround_with_edge(&dungeon.name.to_uppercase()));
    // Difficulty
    output.push_str(&surround_with_edge(&format!(
        "Difficulty: {}",
        dungeon.difficulty
    )));
    output.push_str(&surround_with_edge(""));
    // Challenge Boxes for Levels
    for i in 1i8..=3i8 {
        output.push_str(&surround_with_edge(&format!("Level {}", i)));
        output.push_str(&surround_with_edge("Peril:"));
        for peril in dungeon.peril_challenges.get(&i).unwrap() {
            let challenge_string = peril.to_string();
            output.push_str(&surround_with_edge(&challenge_string));
        }
        output.push_str(&surround_with_edge("Combat:"));
        for combat in dungeon.combat_challenges.get(&i).unwrap() {
            let challenge_string = combat.to_string();
            output.push_str(&surround_with_edge(&challenge_string));
        }
        if i < 3 {
            output.push_str(&surround_with_edge(""));
        }
    }

    output.push_str(&get_card_top_or_bottom());
    println!("{}", output);
}

fn split_line_long_description(
    output: &mut String,
    description: &mut String,
    col_counter: &mut usize,
) {
    output.push_str(&surround_with_edge(&*description));
    description.clear();
    *col_counter = 0;
}

fn get_card_top_or_bottom() -> String {
    let mut result = String::from("|");
    result.push_str(&"-".repeat(CARD_WIDTH + 2));
    result.push('|');
    result.push('\n');
    result
}

fn surround_with_edge(contents: &str) -> String {
    let mut result = String::from("| ");
    result.push_str(&format!("{: <CARD_WIDTH$}", contents));
    result.push_str(" |\n");
    result
}
