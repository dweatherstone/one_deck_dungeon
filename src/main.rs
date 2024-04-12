use dungeon::Dungeon;
use hero::Hero;

pub mod boss;
pub mod dungeon;
pub mod encounter;
pub mod game;
pub mod game_setup;
pub mod hero;
pub mod print_helper;

use print_helper::{print_dungeon, print_hero};

/*
fn main() {
    let mut hero = Hero::get_mage();
    print_hero(&hero);
    // let caliana = Hero::get_caliana();
    // print_hero(&caliana);
    // let phoenix_dungeon = Dungeon::get_phoenix_den();
    // print_dungeon(&phoenix_dungeon);
    let dragons_dungeon = Dungeon::get_dragons_cave();
    print_dungeon(&dragons_dungeon);

    // println!("Getting the number of items and skills for level 1. Should be 1 items and 2 skills");
    // let items = hero
    //     .levels
    //     .get(&hero.current_level)
    //     .expect("Couldn't find the current level")
    //     .get("Items")
    //     .expect("Couldn't find the items");
    // let skills = hero
    //     .levels
    //     .get(&hero.current_level)
    //     .expect("Couldn't find the current level")
    //     .get("Skills")
    //     .expect("Couldn't find the skills");
    // println!("{} items and {} skills", items, skills);

    if hero
        .change_attribute_quantity(hero::AttributeType::Strength, 1)
        .is_err()
    {
        println!("Error changing attribute quantity");
        return;
    };
    print_hero(&hero);
    // if hero
    //     .change_attribute_quantity(hero::AttributeType::Strength, -5)
    //     .is_err()
    // {
    //     println!("Error changing attribute quantity");
    //     return;
    // }
    // print_hero(&hero);
    println!("Current level: {}", hero.current_level);
    for _ in 0..3 {
        match hero.descend_level() {
            Err(e) => {
                println!("Cannot descend further. Error: {}", e);
                return;
            }
            Ok(level) => println!("Current level: {}", level),
        }
    }

    print_hero(&hero);
}
*/
fn main() {
    let perils = game_setup::get_all_perils();

    for peril in perils {
        println!("{}: {}", peril.name, peril.skill_reward.name);
    }

    let combats = game_setup::get_all_combats();

    for combat in combats {
        println!("{}: {}", combat.name, combat.special_ability);
    }
}
