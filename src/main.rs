use dungeon::Dungeon;
use hero::Hero;

pub mod boss;
pub mod dungeon;
pub mod encounter;
pub mod game;
pub mod hero;
pub mod level;
pub mod print_helper;

use print_helper::{print_dungeon, print_hero};

fn main() {
    let hero = Hero::get_mage();
    print_hero(&hero);
    let caliana = Hero::get_caliana();
    print_hero(&caliana);
    let phoenix_dungeon = Dungeon::get_phoenix_den();
    print_dungeon(&phoenix_dungeon);
    let dragons_dungeon = Dungeon::get_dragons_cave();
    print_dungeon(&dragons_dungeon);

    println!("Getting the number of items and skills for level 1. Should be 1 items and 2 skills");
    let items = hero
        .levels
        .get(&hero.current_level)
        .expect("Couldn't find the current level")
        .get("Items")
        .expect("Couldn't find the items");
    let skills = hero
        .levels
        .get(&hero.current_level)
        .expect("Couldn't find the current level")
        .get("Skills")
        .expect("Couldn't find the skills");
    println!("{} items and {} skills", items, skills);
}
