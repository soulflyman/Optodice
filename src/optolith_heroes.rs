pub mod optolith {
    //TODO move everything hero relatet into own struct OptolithHero and let OptolithHeroes just return this struct. Just like in OptolithSkills
    use json::JsonValue;
    use std::env::var;
    use std::path::PathBuf;
    use std::{fs, path::Path};

    #[derive(Debug, Clone)]
    pub struct OptolithHeroes {
        heroes: JsonValue,
    }

    impl OptolithHeroes {
        pub fn new() -> OptolithHeroes {
            //let heroes_path = current_dir().map(|path| path.join("heroes.json")).expect("Ups something unusual happened.");
            //Heroes::copy_heroes(&heroes_path);

            let heroes_path = OptolithHeroes::get_heroes_path();
            let heroes_json =
                fs::read_to_string(heroes_path.as_os_str()).expect(format!("Unable to read file: {}", heroes_path.to_str().unwrap_or("")).as_str());
            OptolithHeroes {
                heroes: json::parse(heroes_json.as_str())
                    .expect("Error: Failed to parse json data"),
            }
        }
        
        fn get_heroes_path() -> PathBuf {
            //todo rewrite, maybe use match
            if cfg!(unix) {
                let hero_path_str = var("HOME").expect("Error: Unable to find AppData directory.");
                let mut hero_path  = Path::new(&hero_path_str).to_path_buf();
                hero_path.push(".config");
                hero_path.push("Optolith");
                hero_path.push("heroes.json");
                return hero_path;
            } else if cfg!(windows) {
                //let hero_path = "C:/Users/micro/AppData/Roaming/Optolith/heroes.json";
                let hero_path_str = var("appdata").expect("Error: Unable to find AppData directory.");
                let mut hero_path  = Path::new(&hero_path_str).to_path_buf();
                hero_path.push("Optolith");
                hero_path.push("heroes.json");

                return hero_path;
            } else if cfg!(macos) {
                //check if this is the same as unix https://stackoverflow.com/questions/43292357/how-can-one-detect-the-os-type-using-rust
            };
            panic!("Error: Could not determin heroes.json path.");
        }

        pub fn get_simple_hero_list(&self) -> Vec<SimpleHero> {
            let mut hero_list: Vec<SimpleHero> = vec![];
            for (key, hero) in self.heroes.entries() {
                hero_list.push(SimpleHero {
                    id: key.to_string(),
                    name: hero["name"].to_string(),
                });
            }
            return hero_list;
        }

        pub fn get_skill_value(&self, hero_id: &String, skill_id: &String) -> i32 {
            let hero: &JsonValue = self.get_hero_by_id_as_ref(hero_id);
            if !hero.has_key("skills") {
                return 0;
            }

            if !hero["skills"].has_key(skill_id.as_str()) {
                return 0;
            }

            return hero["skills"][skill_id].as_i32().unwrap_or(0);
        }

        pub fn get_attribute_value(&self, hero_id: &String, attribute_id: &String) -> i32 {
            let hero: &JsonValue = self.get_hero_by_id_as_ref(hero_id);
            if !hero.has_key("attr") {
                return 8;
            }

            if !hero["attr"].has_key("values") {
                return 8;
            }

            for attr in hero["attr"]["values"].members() {
                if attr["id"].to_string() == attribute_id.to_owned() {
                    return attr["value"].as_i32().unwrap_or(8);
                }
            }
            return 8;
        }

        fn get_hero_by_id_as_ref(&self, hero_id: &String) -> &JsonValue {
            if !self.heroes.has_key(hero_id.as_str()) {
                panic!("Error: Hero not found in heroes.json");
            }

            return &self.heroes[hero_id];
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