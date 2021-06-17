use crc::{CRC_32_ISO_HDLC, Crc};
use json::JsonValue;

use crate::{display_error, optolith::{spell::Spell, spells::Spells, weapon::OptolithWeapon}};

#[derive(Debug, Clone)]
pub struct OptolithHero {
    hero: JsonValue,
    health: i32,
    pain_level: i32,
    astral_points: i32,
}

const CRC: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);   

impl OptolithHero {
    pub fn new(hero_json: &JsonValue) -> OptolithHero {
        OptolithHero {
            hero: hero_json.to_owned(),
            health: 0,
            pain_level: 0,
            astral_points: 0,
        }
    }

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

        if res.is_err() {            
            display_error("Avatar upload failed!", &res.err().unwrap().to_string());
        }
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
        let mut digest = CRC.digest();
        digest.update(self.avatar().as_bytes());
        return digest.finalize().to_string();
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

    pub fn spells(&self) -> Vec<Spell> {        
        let mut spells: Vec<Spell> = vec![];

        if !self.hero.has_key("spells")  {
            return spells;
        }
        
        let all_spells = Spells::new();
        for (spell_id,spell_points) in self.hero["spells"].entries() {
            let mut spell = all_spells.by_id(&spell_id.to_string());
            // todo remove let spell_points_i = spell_points.as_i32().unwrap_or_default();
            spell.set_points(spell_points.as_i32().unwrap_or_default());
            spells.push(spell);
        }
        
        return spells;
    }

    pub fn dodge_value(&self) -> i32 {
        let dodge_value = f64::from(self.attribute_value(&"ATTR_6".to_string())) / 2.0;
        dodge_value.round() as i32
    }

    fn combat_technique_base_value(&self, combat_technique_id: &String) -> i32 {
        if !self.hero.has_key("ct") {
            return 6;
        }

        if !self.hero["ct"].has_key(combat_technique_id) {
            return 6;
        }

        return self.hero["ct"][combat_technique_id].as_i32().unwrap_or(6);
    }

    pub fn attack_value(&self, weapon :&OptolithWeapon) -> i32 {
        let combat_technique_bonus: i32;
        if self.is_ranged_combat_technique(weapon.combat_technique()) {
            let dexterity = self.attribute_value(&"ATTR_5".to_string());
            combat_technique_bonus = ((dexterity - 8) as f64 / 3.0) as i32;
        } else {
            let courage = self.attribute_value(&"ATTR_1".to_string());
            combat_technique_bonus = ((courage - 8) as f64 / 3.0) as i32;
        }

        self.combat_technique_base_value(weapon.combat_technique()) + combat_technique_bonus + weapon.at()
    }

    pub fn is_ranged_combat_technique(&self, combat_technique_id: &String) -> bool {
        let range_techniques = vec!("CT_1", "CT_2", "CT_11", "CT_14", "CT_17", "CT_18", "CT_19");

        range_techniques.contains(&combat_technique_id.as_str())
    }

    pub fn parry_value(&self, weapon :&OptolithWeapon, primary_attributes: Vec<String>) -> i32 {
        let mut primary_attribute_value = 0;
        for attribute in primary_attributes {
            let attribute_value = self.attribute_value(&attribute);
            if attribute_value > primary_attribute_value {
                primary_attribute_value = attribute_value;
            }
        }

        let ct_base_value = self.combat_technique_base_value(weapon.combat_technique());
        let ct_bonus = ((primary_attribute_value - 8) as f64 / 3.0) as i32 ;

        let ct_base_value_half = (ct_base_value as f64 / 2.0).ceil() as i32;

        if weapon.combat_technique() == "CT_10"
        {
            ct_base_value_half + ct_bonus + weapon.pa() * 2
        }else
        {
            ct_base_value_half + ct_bonus + weapon.pa()
        }
        
    }

    /// Get a reference to the optolith hero's health.
    pub fn health(&self) -> &i32 {
        &self.health
    }

    /// Set the optolith hero's health.
    pub fn set_health(&mut self, health: i32) {
        self.health = health;
    }

    pub fn ini(&mut self) -> i32 {
        let mu = self.attribute_value(&"ATTR_1".to_string());
        let ge = self.attribute_value(&"ATTR_6".to_string());

        return ((mu + ge) as f64 / 2.0).ceil() as i32;
    }

    pub fn astral_points(&self) -> &i32 {
        &self.astral_points
    }

    pub fn set_astral_points(&mut self, astral_points: i32) {
        self.astral_points = astral_points;
    }

    pub fn set_pain_level(&mut self, pain_level: i32) {
        self.pain_level = pain_level;
    }

    pub fn pain_level(&self) -> &i32 {
        &self.pain_level
    }

    pub fn is_mage(&self) -> bool {
        self.hero["activatable"].has_key("ADV_50")
    }
}