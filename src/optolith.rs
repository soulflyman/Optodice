pub mod optolith {

    use json::JsonValue;
    use std::{fs, path::Path};
    use std::path::PathBuf;
    use std::env::{var};
    
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
                    .expect("ERROR: HOME environment variable not set.");
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
    }

    #[derive(Debug, Clone, Default)]
    pub struct SimpleHero {
        pub id: String,
        pub name: String,
    }
}