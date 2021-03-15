pub mod optolith {
    use crc::crc32;
    use json::JsonValue;
    use std::{collections::HashMap, env::var};
    use std::path::PathBuf;
    use std::{fs, path::Path};

    #[derive(Debug, Clone)]
    pub struct OptolithHeroes {
        heroes: HashMap<String, OptolithHero>,
        active_hero_id: String,
    }

    impl OptolithHeroes {
        pub fn new() -> OptolithHeroes {
            //let heroes_path = current_dir().map(|path| path.join("heroes.json")).expect("Ups something unusual happened.");
            //Heroes::copy_heroes(&heroes_path);

            let heroes_path = OptolithHeroes::heroes_json_path();
            let heroes_json =
                fs::read_to_string(heroes_path.as_os_str()).expect(format!("Unable to read file: {}", heroes_path.to_str().unwrap_or("")).as_str());

            let heroes_parsed = json::parse(heroes_json.as_str()).expect("Error: Failed to parse json data");
            let mut heroes: HashMap<String, OptolithHero> = HashMap::new();
            for (hero_id, hero_json) in heroes_parsed.entries() {
                let hero = OptolithHero {
                    hero: hero_json.to_owned(),                    
                };
                heroes.insert(hero_id.to_string(), hero);
            }

            OptolithHeroes {
                heroes,
                active_hero_id: String::default(),
            }
        }
        
        fn heroes_json_path() -> PathBuf {
            //todo rewrite? maybe use match
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

        pub fn simple_hero_list(&self) -> Vec<SimpleHero> {
            let mut hero_list: Vec<SimpleHero> = vec![];
            for (hero_id, hero) in &self.heroes {
                hero_list.push(SimpleHero {
                    id: hero_id.to_string(),
                    name: hero.name(),
                });
            }
            return hero_list;
        }

        pub fn set_active_hero(&mut self, hero_id: String) {
            self.active_hero_id = hero_id;
        }
  
        pub fn active_hero(&self) -> &OptolithHero {
            &self.heroes.get(&self.active_hero_id).expect("Error: Hero not found.")
        }
    
        /// Get a reference to the optolith heroes's active hero id.
        pub fn active_hero_id(&self) -> &String {
            &self.active_hero_id
        }
    }

    #[derive(Debug, Clone, Default)]
    pub struct SimpleHero {
        pub id: String,
        pub name: String,
    }

    #[derive(Debug, Clone)]
    pub struct OptolithHero {
        hero: JsonValue,
    }

    impl OptolithHero {
        pub fn name(&self) -> String {
            self.hero["name"].to_string()
        }

        pub fn id(&self) -> String {
            self.hero["id"].to_string()
        }

        pub fn skill_value(&self, skill_id: &String) -> i32 {
            
            if !self.hero.has_key("skills") {
                return 0;
            }

            if !self.hero["skills"].has_key(skill_id.as_str()) {
                return 0;
            }

            return self.hero["skills"][skill_id].as_i32().unwrap_or(0);
        }

        pub fn attribute_value(&self, attribute_id: &String) -> i32 {
            if !self.hero.has_key("attr") {
                return 8;
            }

            if !self.hero["attr"].has_key("values") {
                return 8;
            }

            for attr in self.hero["attr"]["values"].members() {
                if attr["id"].to_string() == attribute_id.to_owned() {
                    return attr["value"].as_i32().unwrap_or(8);
                }
            }
            return 8;
        }

        pub fn avatar(&self) -> String {
            if self.hero.has_key("avatar") {
                return self.hero["avatar"].to_string();
            }

            return String::default();
        }

        pub fn upload_avatar(&self, uploader_url: String) {
            let params = [("hero_id", self.id()), ("image", self.avatar()), ("checksum", self.avater_checksum())];
            let client = reqwest::blocking::Client::new();
            let res = client.post(uploader_url.as_str())
                .form(&params)
                .send();
            dbg!(res);
        }

        pub fn get_avatar_file_name(&self) -> String {
            let checksum = self.avater_checksum();
            let mut file_name = self.id();
            file_name.push('_');
            file_name.push_str(checksum.as_str());
            file_name.push_str(".png");
            return file_name;
        }

        fn avater_checksum(&self) -> String {
            crc32::checksum_ieee(self.avatar().as_bytes()).to_string()
        }
    }
}