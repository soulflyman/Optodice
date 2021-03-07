use std::{fs, path::{Path, PathBuf}};
use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Config {
    webhook_url: Option<String>,
    last_used_hero_id: Option<String>,
    avatar_proxy_url: Option<String>,
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
        let mut optodice_path = std::env::current_exe().expect("Error: Unable to define Application path.");
        optodice_path.pop();
        let mut config_toml_path = Path::new(&optodice_path).to_path_buf();
        config_toml_path.push("config.toml");
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
        self.webhook_url.is_some() && self.webhook_url.as_ref().unwrap().len() > 0
    }

    pub fn get_last_used_hero_id(&self) -> String {
        return self.last_used_hero_id.clone().unwrap_or_default();
    }

    pub fn set_avatar_proxy_url(&mut self, avatar_proxy_url: String) {
        self.avatar_proxy_url = Some(avatar_proxy_url);
        self.save();
    }

    pub fn get_avatar_proxy_url(&self) -> String {
        return self.avatar_proxy_url.clone().unwrap_or_default();        
    }

    pub fn set_avatar_base_url(&mut self, avater_base_url: String) {
        self.avater_base_url = Some(avater_base_url);
        self.save();
    }

    pub fn get_avatar_base_url(&self) -> String {
        return self.avater_base_url.clone().unwrap_or_default();        
    }
}