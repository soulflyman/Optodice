pub mod optolith {

    use lazy_static::lazy_static;
    use std::fs;
    use std::path::PathBuf;
    use std::env::{var, current_dir};
    use std::process::exit;
    use json::JsonValue;
    use json::object::Iter;

    pub struct Heroes {
        heroes_path: PathBuf,
        heroes: JsonValue,
    }

    impl Heroes {
        pub fn new() -> Heroes {
            let heroes_path = current_dir().map(|path| path.join("heroes.json")).expect("Ups something unusual happened.");
            Heroes::copy_heroes(&heroes_path);
            let heroes_json = fs::read_to_string(heroes_path.as_os_str()).expect("Unable to read file");
            //TODO replace json with serde_json
            Heroes {
                heroes_path,
                heroes: json::parse(heroes_json.as_str()).unwrap(),
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

        pub fn get_simple_hero_list(&self) -> Vec<String> {
            let mut hero_list: Vec<String> = vec!();
            for (key, hero) in self.heroes.entries() {
                hero_list.push(hero["name"].to_string());
            }
            return hero_list;
        }
    }


}