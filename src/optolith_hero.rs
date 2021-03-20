use crc::crc32;
use json::JsonValue;

use crate::optolith_weapon::OptolithWeapon;

#[derive(Debug, Clone)]
pub struct OptolithHero {
    pub(crate) hero: JsonValue,
}

impl OptolithHero {
    pub fn name(&self) -> String {
        self.hero["name"].to_string()
    }

    pub fn id(&self) -> String {
        self.hero["id"].to_string()
    }

    pub fn skill_points(&self, skill_id: &String) -> i32 {
        
        if !self.hero.has_key("talents") {
            return 0;
        }

        if !self.hero["talents"].has_key(skill_id.as_str()) {
            return 0;
        }

        return self.hero["talents"][skill_id].as_i32().unwrap_or(0);
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
        //dbg!(res);
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

    pub fn weapons(&self) -> Vec<OptolithWeapon> {
        let mut weapons: Vec<OptolithWeapon> = vec![];
        if !self.hero.has_key("belongings") || !self.hero["belongings"].has_key("items") {
            return weapons;
        }

        for (_,b) in self.hero["belongings"]["items"].entries() {
            if !b.has_key("combatTechnique") {
                continue;
            }

            let weapon = OptolithWeapon::new_from_json(b);
            weapons.push(weapon);
        }
        
        return weapons;
    }
}