pub mod optolith {
    use std::{collections::HashMap, env::var};
    use std::path::PathBuf;
    use std::{fs, path::Path};

    use crate::optolith_hero::OptolithHero;

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
                let mut hero = OptolithHero {
                    hero: hero_json.to_owned(),
                    health: 0,
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
                hero_path.push(".config/Optolith/heroes.json");
                //hero_path.push("Optolith");
                //hero_path.push("heroes.json");
                return hero_path;
            } else if cfg!(windows) {
                //let hero_path = "C:/Users/micro/AppData/Roaming/Optolith/heroes.json";
                let hero_path_str = var("appdata").expect("Error: Unable to find AppData directory.");
                let mut hero_path  = Path::new(&hero_path_str).to_path_buf();
                hero_path.push("Optolith/heroes.json");
                return hero_path;
            } else if cfg!(macos) {
                let hero_path_str = var("HOME").expect("Error: Unable to find AppData directory.");
                let mut hero_path  = Path::new(&hero_path_str).to_path_buf();
                hero_path.push("Library/Application Support/Optolith/heroes.json");
            };
            panic!("Error: Could not determin heroes.json path.");
        }

        pub fn simple_hero_list(&self) -> Vec<SimpleHero> {
            let mut hero_count: HashMap<String, i32> = HashMap::new();

            let mut hero_list: Vec<SimpleHero> = vec![];
            for (hero_id, hero) in &self.heroes {
                let mut hero_name = hero.name();
                *hero_count.entry(hero_name.clone()).or_insert(0) += 1;
                if hero_count.get(&hero_name).unwrap().to_owned() > 1 {
                    hero_name = format!("{} ({})", hero_name, hero_count.get(&hero_name).unwrap());
                }
                
                hero_list.push(SimpleHero {
                    id: hero_id.to_string(),
                    name: hero_name,
                });
            }
            return hero_list;
        }

        pub fn set_active_hero(&mut self, hero_id: String) {
            self.active_hero_id = hero_id;            
        }
  
        pub fn active_hero(&mut self) -> &mut OptolithHero {
            self.heroes.get_mut(&self.active_hero_id).expect("Error: Hero not found.")
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


}