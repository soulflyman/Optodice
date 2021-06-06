use std::{env::var, fs, path::{Path, PathBuf}, str::FromStr};
use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Config {
    webhook_url: Option<String>,
    last_used_hero_id: Option<String>,
    avatar_uploader_url: Option<String>,
    avater_base_url: Option<String>,
}

impl Config {
    pub fn load() -> Config {        
        let config_file_path = Config::get_config_file_path();
        if config_file_path.exists() {
            let config_toml_content = fs::read_to_string(config_file_path).expect("Error: Failed to read config.toml file.");
            let config: Config = toml::from_str(config_toml_content.as_str()).expect("Error: Parsing of config.toml failed.");
            return config;
        }
        
        let config = Config::default();
        config.save();
        return config;
    }

    fn save(&self) {
        let config_file_path = Config::get_config_file_path();
        let new_config_toml = toml::to_string_pretty(self).expect("Error: Failed to serealize config data.");
        fs::write(config_file_path, new_config_toml).expect("Error: Unable to write config.toml file.");
    }

    fn get_config_file_path() -> PathBuf {
        let mut config_dir_path = Config::get_config_dir_path();
        if !config_dir_path.exists() {
            fs::create_dir(&config_dir_path).expect("Error: Failed to create config dir.")
        }
        config_dir_path.push("config.toml");
        dbg!(&config_dir_path);
        return config_dir_path
    }

    fn get_config_dir_path() -> PathBuf {
        let mut system_config_dir_path: String;
        let macos_config_dir_extras = "/Library/Application Support";
        if cfg!(unix) {
            system_config_dir_path = match var("XDG_CONFIG_HOME"){
                Ok(path) => path,
                Err(_) => {
                    let mut home_dir = var("HOME").expect("Error: System variable $HOME ist not set.");
                    home_dir.push_str("/.config");
                    home_dir
                }
            }
        } else if cfg!(windows) {
            system_config_dir_path = var("appdata").expect("Error: Unable to find AppData directory.");           
        } else if cfg!(macos) {
            system_config_dir_path = var("HOME").expect("Error: System variable $HOME ist not set.");
            system_config_dir_path.push_str(macos_config_dir_extras);
        } else {
            panic!("Error: Unknow platform. Couldn't find config folder.");
        };

        if system_config_dir_path.is_empty() || system_config_dir_path == macos_config_dir_extras.to_string() {
            panic!("Error: Ups, system variable $XDG_CONFIG_HOME (Linux), %appdata% (Windows) or $HOME (Linux or MacOS) are not set or the.");
        }

        let mut config_path = Path::new(system_config_dir_path.as_str()).to_path_buf();
        config_path.push("Optodice");
        return config_path;
    }

    pub fn set_webhook_url(&mut self, webhook_url: String) {
        self.webhook_url = Some(webhook_url);
        self.save();
    }

    pub fn get_webhook_url(&self) -> String {
        return self.webhook_url.clone().unwrap_or_default();        
    }

    pub fn set_last_used_hero_id(&mut self, hero_id: String) {
        self.last_used_hero_id = Some(hero_id);
        self.save();
    }

    pub fn is_webhook_url_set(&self) -> bool {
        self.webhook_url.is_some() && !self.webhook_url.as_ref().unwrap().is_empty()
    }

    pub fn is_avatar_base_url_set(&self) -> bool {
        self.avater_base_url.is_some() && !self.avater_base_url.as_ref().unwrap().is_empty() 
    }

    pub fn get_last_used_hero_id(&self) -> String {
        return self.last_used_hero_id.clone().unwrap_or_default();
    }

    pub fn set_avatar_uploader_url(&mut self, avatar_uploader_url: String) {
        self.avatar_uploader_url = None;
        self.avatar_uploader_url = Some(avatar_uploader_url.clone());
        self.save();

        if avatar_uploader_url.is_empty() {
            return;
        }        

        let pos = avatar_uploader_url.rfind('/');
        if pos.is_none() {
            return;
        }
        let avatar_base_url = avatar_uploader_url.get(..pos.unwrap());
        if avatar_base_url.is_none() {
            return;
        }
        self.set_avatar_base_url(avatar_base_url.unwrap().to_string());
    }

    pub fn get_avatar_uploader_url(&self) -> String {
        return self.avatar_uploader_url.clone().unwrap_or_default();        
    }

    pub fn is_avatar_uploader_url_set(&self) -> bool {
        self.avatar_uploader_url.is_some() && !self.avatar_uploader_url.as_ref().unwrap().is_empty()
    }

    pub fn set_avatar_base_url(&mut self, avater_base_url: String) {
        self.avater_base_url = Some(avater_base_url);
        self.save();
    }

    pub fn get_avatar_base_url(&self) -> String {
        return self.avater_base_url.clone().unwrap_or_default();        
    }

    pub fn use_avatars(&self) -> bool {        
        self.is_avatar_uploader_url_set() && self.is_avatar_base_url_set()
    }
}