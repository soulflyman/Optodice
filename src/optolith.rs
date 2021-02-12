pub mod optolith {

    use json::JsonValue;
    use std::{fs, path::Path};
    use std::path::PathBuf;
    use std::env::var;
    
    
    #[derive(Debug, Clone)]
    pub struct OptolithHeroes {
        heroes: JsonValue,
    }

    impl OptolithHeroes {
        pub fn new() -> OptolithHeroes {
            //let heroes_path = current_dir().map(|path| path.join("heroes.json")).expect("Ups something unusual happened.");
            //Heroes::copy_heroes(&heroes_path);

            let heroes_path = OptolithHeroes::get_heroes_path();
            let heroes_json = fs::read_to_string(heroes_path.as_os_str()).expect("Unable to read file");
            OptolithHeroes {
                heroes: json::parse(heroes_json.as_str()).expect("Error: Failed to parse json data")
            }
        }

        fn copy_heroes(heroes_path: &PathBuf) {
            //todo rewrite, maybe use match
            if cfg!(unix) {                
                let heroes_source_file_path = var("HOME").map(|home|format!("{}/.config/Optolith/heroes.json", home))
                    .expect("Error: HOME environment variable not set.");
                println!("copy {} -> {}", heroes_source_file_path, heroes_path.display());
                match fs::copy(heroes_source_file_path, heroes_path.to_path_buf()) {
                    Err(e) => {
                        println!("Could not copy heroes.json from Optolith.\n{}", e.to_string())
                    }
                    _ => return
                };
            } else if cfg!(windows) {

            } else {

            };
        }

        fn get_heroes_path() -> PathBuf {
            //todo rewrite, maybe use match
            if cfg!(unix) { 
                let hero_path =  var("HOME").map(|home|format!("{}/.config/Optolith/heroes.json", home))
               .expect("ERROR: HOME environment variable not set.");               
               return Path::new(&hero_path).to_path_buf();
            } else if cfg!(windows) {

            } else if cfg!(osx) {

            };
            panic!("Error: Could not determin heroes.json path.");
        }

        pub fn get_simple_hero_list(&self) -> Vec<SimpleHero> {
            let mut hero_list: Vec<SimpleHero> = vec!();
            for (key, hero) in self.heroes.entries() {
                hero_list.push(SimpleHero {
                    id: key.to_string(),
                    name: hero["name"].to_string(),
                });
            }
            return hero_list;
        }

        pub fn get_skill_value(&self, hero_id: &String, skill_id: &String) -> i32 {
           
            if skill_id.starts_with("TAL_") {
                return self.get_talent_value(hero_id, skill_id);
            }

            if skill_id.starts_with("ATTR_") {
                return self.get_attribute_value(hero_id, skill_id);
            }

            0
        }

        fn get_talent_value(&self, hero_id: &String, talent_id: &String) -> i32 {
            let hero: &JsonValue = self.get_hero_by_id_as_ref(hero_id);
            if !hero.has_key("talents") {
                return 0;
            }

            if !hero["talents"].has_key(talent_id.as_str()) {
                return 0;
            }

            return hero["talents"][talent_id].as_i32().unwrap_or(0);
        }

        fn get_attribute_value(&self, hero_id: &String, attribute_id: &String) -> i32 {
            let hero: &JsonValue = self.get_hero_by_id_as_ref(hero_id);
            if !hero.has_key("attr") {
                return 0;
            }

            if !hero["attr"].has_key("values"){
                return 0;
            }

            for attr in hero["attr"]["values"].members() {
                if attr["id"].to_string() == attribute_id.to_owned() {
                    return attr["value"].as_i32().unwrap_or(0);
                }
            }
            return 0;
        }

        fn get_hero_by_id_as_ref(&self, hero_id: &String) -> &JsonValue {
            if !self.heroes.has_key(hero_id.as_str()) {
                panic!("Error: Hero not found in heroes.json");                
            }

            return &self.heroes[hero_id]
        } 

        pub fn get_hero_name_by_id(&self, hero_id: String) -> String {
            let hero = self.get_hero_by_id_as_ref(&hero_id);
            if !hero.has_key("name") {
                return String::default();
            }

            return hero["name"].to_string();
        }
    }

    #[derive(Debug, Clone, Default)]
    pub struct SimpleHero {
        pub id: String,
        pub name: String,
    }
}