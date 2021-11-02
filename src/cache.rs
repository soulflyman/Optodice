use std::{fs::{self, File, OpenOptions}, io::BufReader, path::PathBuf};
use serde::{Serialize, Deserialize};

use crate::APP_NAME;

#[derive(Default, Serialize, Deserialize)]
pub struct CachedAppData {
    last_used_hero_id: String,
    window_pos_x: i32,
    window_pos_y: i32,
    window_width: i32,
    window_height: i32,
    windows_is_maximized: bool,
}

pub struct Cache;

impl Cache {

    pub fn cache_dir_path() -> PathBuf {
        let mut app_cache_dir_path = dirs_next::cache_dir().expect("Error: Unable to find cache directory.");
        app_cache_dir_path.push(APP_NAME);
        
        if !app_cache_dir_path.exists() {
            fs::create_dir(&app_cache_dir_path).expect("Error: Failed to create the cache directory.");
        }
        
        return app_cache_dir_path;
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