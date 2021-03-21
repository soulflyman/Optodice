use json::JsonValue;

#[derive(Debug, Clone, Default)]
pub struct OptolithWeapon {
    id: String,
    name: String,
    gr: i32,    
    amount: i32,
    at: i32,
    damage_dice_number: i32,
    damage_flat: i32,
    length: i32,
    pa: i32,
    combat_technique: String,
    damage_dice_sides: i32,
    reach: i32,
    template: String,
    primary_threshold: i32,
}

impl OptolithWeapon {
    pub fn new_from_json(json_data: &JsonValue) -> OptolithWeapon {
        let mut weapon = OptolithWeapon::default();
        
        for (key, value) in json_data.entries() {
            match key.to_string().as_str() {                
                "id" => weapon.id = value.to_string(),
                "name" => weapon.name = value.to_string(),
                "gr" => weapon.gr = value.as_i32().unwrap(),
                "amount" => weapon.amount = value.as_i32().unwrap(),
                "at" => weapon.at = value.as_i32().unwrap(),
                "damageDiceNumber" => weapon.damage_dice_number = value.as_i32().unwrap(),
                "damageFlat" => weapon.damage_flat = value.as_i32().unwrap(),
                "length" => weapon.length = value.as_i32().unwrap(),
                "pa" => weapon.pa = value.as_i32().unwrap(),
                "combatTechnique" => weapon.combat_technique = value.to_string(),
                "damageDiceSides" => weapon.damage_dice_sides = value.as_i32().unwrap(),
                "reach" => weapon.reach = value.as_i32().unwrap(),
                "template" => weapon.template = value.to_string(),
                "primaryThreshold" => {
                    if value.has_key("threshold") {
                        weapon.primary_threshold = value["threshold"].as_i32().unwrap();
                    }
                },                
                _ => continue,
            }
        }

        /*
        if json_data.has_key("") {
            weapon. = json_data[""].as_i32();
        }
        */

        return weapon;
    }

    /// Get a reference to the optolith weapon's name.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Get a reference to the optolith weapon's id.
    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn can_parry(&self) -> bool {
        let range_techniques = vec!("CT_1".to_string(), "CT_2".to_string(), "CT_11".to_string(), "CT_14".to_string(), "CT_18".to_string(), "CT_19".to_string());

        return !range_techniques.contains(&self.combat_technique);
    }

    /// Get a reference to the optolith weapon's combat technique.
    pub fn combat_technique(&self) -> &String {
        &self.combat_technique
    }
}