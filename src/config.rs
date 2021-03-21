use std::{fs, path::{Path, PathBuf}};
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
        //TODO use os depending path for config file:
        // linux: $XDG_CONFIG_HOME/Optodice
        // windows: %appdata%/Optodice
        // macos: $HOME/Library/Application Support/Optodice
        let mut optodice_path = std::env::current_exe().expect("Error: Unable to define Application path.");
        optodice_path.pop();
        let mut config_toml_path = Path::new(&optodice_path).to_path_buf();
        config_toml_path.push("config.toml");
        dbg!(&config_toml_path);
        return config_toml_path;
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
        if !avatar_uploader_url.is_empty() {
            self.avatar_uploader_url = Some(avatar_uploader_url.clone());
        }        
        self.save();

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