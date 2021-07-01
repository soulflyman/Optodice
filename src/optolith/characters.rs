use std::{collections::HashMap, env::var};
use std::path::PathBuf;
use std::{fs, path::Path};

use crate::optolith::character::OptolithCharacter;

#[derive(Debug, Clone)]
pub struct OptolithCharacters {
    characters: HashMap<String, OptolithCharacter>,
    active_character_id: String,
}

impl OptolithCharacters {
    pub fn new() -> OptolithCharacters {
        let heroes_json_path = OptolithCharacters::heroes_json_path();
        let heroes_json_raw =
            fs::read_to_string(heroes_json_path.as_os_str()).expect(format!("Unable to read file: {}", heroes_json_path.to_str().unwrap_or("")).as_str());

        let heroes_json = json::parse(heroes_json_raw.as_str()).expect("Error: Failed to parse json data");
        let mut characters: HashMap<String, OptolithCharacter> = HashMap::new();
        for (character_id, character_json) in heroes_json.entries() {
            let character = OptolithCharacter::new(character_json);                
            characters.insert(character_id.to_string(), character);
        }

        OptolithCharacters {
            characters,
            active_character_id: String::default(),
        }
    }
    
    fn heroes_json_path() -> PathBuf {
        //todo rewrite? maybe use match
        if cfg!(unix) {
            let heroes_json_path_str = var("HOME").expect("Error: Unable to find AppData directory.");
            let mut heroes_json_path  = Path::new(&heroes_json_path_str).to_path_buf();
            heroes_json_path.push(".config/Optolith/heroes.json");
            return heroes_json_path;
        } else if cfg!(windows) {
            //let hero_path = "C:/Users/micro/AppData/Roaming/Optolith/heroes.json";
            let heroes_path_str = var("appdata").expect("Error: Unable to find AppData directory.");
            let mut heroes_json_path  = Path::new(&heroes_path_str).to_path_buf();
            heroes_json_path.push("Optolith/heroes.json");
            return heroes_json_path;
        } else if cfg!(macos) {
            let heroes_json_path_str = var("HOME").expect("Error: Unable to find AppData directory.");
            let mut heroes_json_path  = Path::new(&heroes_json_path_str).to_path_buf();
            heroes_json_path.push("Library/Application Support/Optolith/heroes.json");
        };
        panic!("Error: Unknown platform. Could not determin heroes.json path.");
    }

    pub fn simple_character_list(&self) -> Vec<SimpleCharacter> {
        let mut character_count: HashMap<String, i32> = HashMap::new();

        let mut character_list: Vec<SimpleCharacter> = vec![];
        for (character_id, character) in &self.characters {
            let mut character_name = character.name();
            *character_count.entry(character_name.clone()).or_insert(0) += 1;
            if character_count.get(&character_name).unwrap().to_owned() > 1 {
                character_name = format!("{} ({})", character_name, character_count.get(&character_name).unwrap());
            }
            
            character_list.push(SimpleCharacter {
                id: character_id.to_string(),
                name: character_name,
            });
        }
        return character_list;
    }

    pub fn set_active_character(&mut self, character_id: String) {
        self.active_character_id = character_id;            
    }

    pub fn active(&mut self) -> &mut OptolithCharacter {
        self.characters.get_mut(&self.active_character_id).expect("Error: Character not found.")
    }

    pub fn active_character_id(&self) -> &String {
        &self.active_character_id
    }
}

#[derive(Debug, Clone, Default)]
pub struct SimpleCharacter {
    pub id: String,
    pub name: String,
}
