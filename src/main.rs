use hero::Hero;

pub mod boss;
pub mod dungeon;
pub mod encounter;
pub mod game;
pub mod hero;
pub mod level;
pub mod print_helper;

fn main() {
    let hero = Hero::get_mage();
    print_helper::print_hero(&hero);
}
