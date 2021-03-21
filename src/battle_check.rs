use crate::{context::Context, optolith_weapon::OptolithWeapon};
use crate::battle_check_result::BattleCheckResult;
use rand::prelude::*;
#[derive(Debug, Default, Clone)]
pub struct BattleCheck;

impl BattleCheck { 
    pub fn dodge(context: &Context, difficulty: i32) -> BattleCheckResult {        
        
        let dodge_value = context.heroes.active_hero().dodge_value() as i32;

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
        let ct_value = context.heroes.active_hero().combat_technique_value(weapon.combat_technique());
        
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

    fn parry(context: &Context) {

    }
}