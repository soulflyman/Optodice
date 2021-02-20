mod optolith;


#[derive(Debug)]
pub struct AbilityCheckFactory {
    heroes :optolith::OptolithHeroes,
    talent_mapping :JsonValue,
    abilites :HashMap<String, AbilityCheck>,
}

impl AbilityCheckFactory {
    pub fn new(heroes :optolith::OptolithHeroes) -> AbilityCheckFactory {

        let path = "./src/talents.json";
        let json_data = fs::read_to_string(path).expect("Unable to read file");
        let talents: JsonValue = json::parse(&json_data).expect("Error: Parsing of json data failed.");

        AbilityCheckFactory{
            heroes =  heroes,
            talent_mapping = talents,
            abilites = HashMap::new(),
        }
    }

    pub fn get_ability_check(&self, hero_id :String, skill_id :String) -> AbilityCheck {
        let ability_check = self.abilites.entry(skill_id);
        if !ability_check.contains_key(skill_id) {
            let new_abilit_check = AbilityCheck::new(self.heroes, hero_id, self.talent_mapping, skill_id);
            self.abilites.insert(skill_id, new_abilit_check.clone());
            return new_abilit_check;
        }else{
            return ability_check.entry(skill_id);
        }
        
    }
}