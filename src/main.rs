mod optolith;
mod test_ui;
mod ui;

use std::fs;
use std::process::exit;
use crate::optolith::optolith::Heroes;
use iced::{Sandbox, Settings};

fn main() {
    let path = "./src/talents.json";
    let json_data = fs::read_to_string(path).expect("Unable to read file");
    let talents = json::parse(&json_data).unwrap();
    println!("{:#}", talents["talents"]["TAL_1"]);


    let heroes = Heroes::new();
    let hero_list = heroes.get_simple_hero_list();
    for hero_name in &hero_list {
        println!("hero found: {}", hero_name);
    }
 
    //let bla = test_ui::test_ui::Counter::run(Settings::default());
    let blub = ui::ui_hero_select::ui_hero_select::HeroSelect::run(Settings::default());
}