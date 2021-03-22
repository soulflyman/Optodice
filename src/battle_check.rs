use crate::{context::Context, optolith_weapon::OptolithWeapon};
use crate::battle_check_result::BattleCheckResult;
use rand::prelude::*;
#[derive(Debug, Default, Clone)]
pub struct BattleCheck;

impl BattleCheck { 
    pub fn dodge(context: &Context, difficulty: i32) -> BattleCheckResult {        
        
        let dodge_value = context.heroes.active_hero().dodge_value();

        let mut rng = rand::thread_rng();
        let dice_value = rng.gen_range(1..21);
        let mut success = dice_value <= dodge_value;
        let mut critical = false;
        
        
        if dice_value == 20 {
            // critical fail
            success = false;
            critical = true;
        }

        if dice_value == 1 {
            // critical success
            success = true;
            critical = true;
        }       

        BattleCheckResult {
            action_name: "Ausweichen".to_string(),
            action_name_abbr: "DO".to_string(),
            action_value: dodge_value,
            dice_value,
            difficulty,
            success,
            critical,
        }
    }

    pub fn attack(context: &Context, weapon: &OptolithWeapon, difficulty: i32) -> BattleCheckResult{
        let ct_value = context.heroes.active_hero().attack_value(&weapon);
        
        let mut rng = rand::thread_rng();
        let dice_value = rng.gen_range(1..21);
        let mut success = dice_value <= ct_value;
        let mut critical = false;        
        
        if dice_value == 20 {
            // critical fail
            success = false;
            critical = true;
        }

        if dice_value == 1 {
            // critical success
            success = true;
            critical = true;
        }       

        BattleCheckResult {
            action_name: format!("{} Attacke", weapon.name()),
            action_name_abbr: "AT".to_string(),
            action_value: ct_value,
            dice_value,
            difficulty,
            success,
            critical,
        }
    }

    pub fn parry(context: &Context, weapon: &OptolithWeapon, difficulty: i32) -> BattleCheckResult {    
        let ct_primary_attributes = context.combat_techniques.primary_attributes(weapon.combat_technique());
        let parry_value = context.heroes.active_hero().parry_value(&weapon, ct_primary_attributes);
                  
        let mut rng = rand::thread_rng();
        let dice_value = rng.gen_range(1..21);
        let mut success = dice_value <= parry_value;
        let mut critical = false;        
        
        if dice_value == 20 {
            // critical fail
            success = false;
            critical = true;
        }

        if dice_value == 1 {
            // critical success
            success = true;
            critical = true;
        }       

        BattleCheckResult {
            action_name: format!("{} Parade", weapon.name()),
            action_name_abbr: "AT".to_string(),
            action_value: parry_value,
            dice_value,
            difficulty,
            success,
            critical,
        }
    }
}