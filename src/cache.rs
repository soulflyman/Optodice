use std::{env::var, fs::{self, File, OpenOptions}, io::BufReader, path::{Path, PathBuf}};

pub struct Cache;

impl Cache {

    pub fn cache_dir_path() -> PathBuf {
        let mut cache_dir_path: String;
        let macos_cache_dir_extras = "/Library/Caches";

        if cfg!(unix) {
            cache_dir_path = match var("XDG_CACHE_HOME"){
                Ok(path) => path,
                Err(_) => {
                    let mut home_dir = var("HOME").expect("Error: System variable $HOME ist not set.");
                    home_dir.push_str("/.cache");
                    home_dir
                }
            }
        } else if cfg!(windows) {
            cache_dir_path = var("localappdata").expect("Error: Unable to find AppData directory.");           
        } else if cfg!(macos) {
            cache_dir_path = var("HOME").expect("Error: System variable $HOME ist not set.");
            cache_dir_path.push_str(macos_cache_dir_extras);
        } else {
            panic!("Error: Unknow platform. Couldn't find config folder.");
        };

        if cache_dir_path.is_empty() || cache_dir_path == macos_cache_dir_extras.to_string() {
            panic!("Error: Ups, system variable $XDG_CACHE_HOME (Linux), %localappdata% (Windows) or $HOME (Linux or MacOS) are not set or the.");
        }

        let mut app_cache_dir_path = Path::new(cache_dir_path.as_str()).to_path_buf();
        app_cache_dir_path.push("optodice");
        
        if !app_cache_dir_path.exists() {
            fs::create_dir(&app_cache_dir_path).expect("Error: Failed to create the cache directory.");
        }
        
        return app_cache_dir_path;
    }   

    pub fn get_str(cache_id: &str) -> String {
        let mut cache_file: PathBuf = Self::cache_dir_path();
        cache_file.push(cache_id);
        if cache_file.exists() {
            return fs::read_to_string(cache_file).unwrap()
        }
        
        String::default()
    }

    pub fn read_object<'a, T>(cache_id: &'a str) -> Result<T, serde_json::Error> where T: for<'de> serde::Deserialize<'de> {
        let mut path = Self::cache_dir_path();
        path.push(cache_id);
        
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        serde_json::from_reader(reader)
    }

    pub fn save_object<T>(obj: T, cache_id: &str) -> Result<(), serde_json::Error> where T: serde::Serialize {
        let mut path = Self::cache_dir_path();
        path.push(cache_id);

        let file = OpenOptions::new()
                                            .create(true)
                                            .write(true)
                                            .truncate(true)
                                            .open(path).expect("Error: Failed to create cache file.");
        
        serde_json::to_writer(file, &obj)
    }

    pub fn exists(cache_id: &str) -> bool {
        let mut cache_file: PathBuf = Self::cache_dir_path();
        cache_file.push(cache_id);

        cache_file.exists()
    }
}