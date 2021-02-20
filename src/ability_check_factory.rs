use json::JsonValue;
use crate::optolith::optolith::*;
use std::{collections::HashMap, fs};
use crate::ability_check::AbilityCheck;

#[derive(Debug)]
pub struct AbilityCheckFactory {
    heroes :OptolithHeroes,
    talent_mapping :JsonValue,
    abilites :HashMap<String, AbilityCheck>,
}

impl AbilityCheckFactory {
    pub fn new(heroes :OptolithHeroes) -> AbilityCheckFactory {
        let test = AbilityCheck::default();

        let path = "./src/talents.json";
        let json_data = fs::read_to_string(path).expect("Unable to read file");
        let talents: JsonValue = json::parse(&json_data).expect("Error: Parsing of json data failed.");

        AbilityCheckFactory{
            heroes: heroes,
            talent_mapping: talents,
            abilites: HashMap::new(),
        }
    }
 
    pub fn get_ability_check(&mut self, hero_id :String, skill_id :String) -> AbilityCheck {
        if !self.abilites.contains_key(&skill_id) {
            let new_abilit_check = AbilityCheck::new(self.heroes.clone(), hero_id, self.talent_mapping.clone(), skill_id.clone());
            self.abilites.insert(skill_id.clone(), new_abilit_check.clone());
            return new_abilit_check;
        }else{
            return self.abilites.get(&skill_id).expect("kanns net sagt manu").to_owned();
        }
    }
}