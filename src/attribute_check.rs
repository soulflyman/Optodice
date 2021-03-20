use crate::context::Context;
use crate::attribute_check_result::AttributeCheckResult;
use rand::prelude::*;
#[derive(Debug, Default, Clone)]
pub struct AttributeCheck {
    attribute_name: String,
    attribute_id: String,
    attribute_name_abbr: String,
    attribute_value: i32,
    dice_value: i32,
    quality: i32,
}

impl AttributeCheck {
    pub fn new(
        context: &Context,   
        attribute_id: String,
    ) -> AttributeCheck {
        let mut attribute_check = AttributeCheck {
            attribute_name: context.attributes.by_id(&attribute_id).get_name(),
            attribute_id: attribute_id.clone(),
            attribute_name_abbr: context.attributes.by_id(&attribute_id).get_abbr(),
            attribute_value: context.heroes.active_hero().attribute_value(&attribute_id),
            ..AttributeCheck::default()
        };

        return attribute_check;
    }

    pub fn check(&mut self, difficulty :&i32) -> AttributeCheckResult {
        let mut check_result = AttributeCheckResult::default();       

        check_result.difficulty = difficulty.clone();
      
        check_result.attribute_value = self.attribute_value.clone();
        check_result.attribute_name = self.attribute_name.clone();

        let mut rng = rand::thread_rng();
        self.dice_value = rng.gen_range(1..21);
        
        check_result.dice_value = self.dice_value.clone();

        if self.check_critical_roll(20) {
            // Kritischer Patzer
            check_result.success = false;
            check_result.critical = true;
            return check_result;
        }

        if self.check_critical_roll(1) {
            // Kritischer Erfolg
            check_result.success = true;
            check_result.critical = true;
            return check_result;
        }

        check_result.success = self.dice_value <= self.attribute_value;
        return check_result;
    }

    fn check_critical_roll(&self, value: i32) -> bool {
        if self.dice_value == value {
            return true;
        }

        return false;
    } 
}