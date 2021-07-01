use crate::context::Context;
use crate::optolith::spell::Spell;
use rand::prelude::*;

use super::results::spell_check_result::SpellCheckResult;
#[derive(Debug, Default, Clone)]
pub struct SpellCheck;

impl SpellCheck {
    pub fn check(context: &mut Context, spell: &Spell, difficulty :&i32) -> SpellCheckResult {
        let mut check_result = SpellCheckResult::default();
        let mut running_skill_score = spell.points();

        check_result.difficulty = difficulty.clone();
        check_result.spell_points = spell.points();
        check_result.spell_name = spell.name().to_string();

        for attribute_id in spell.check().to_owned() {
            let attribute_value = context.characters.active().attribute_value(&attribute_id);
            check_result.attribute_values.push(attribute_value);
            
            let attribute_name = context.attributes.by_id(&attribute_id).name_abbr;
            check_result.attribute_names.push(attribute_name);
        }

        let mut rng = rand::thread_rng();
        let mut dice_values: Vec<i32> = vec![];
        dice_values.push(rng.gen_range(1..21));
        dice_values.push(rng.gen_range(1..21));
        dice_values.push(rng.gen_range(1..21));

        check_result.dice_values = dice_values.clone();

        if Self::check_critical_roll(&dice_values, 20) {
            // Kritischer Patzer
            check_result.success = false;
            check_result.critical = true;
            return check_result;
        }

        if Self::check_critical_roll(&dice_values, 1) {
            // Kritischer Erfolg
            check_result.quality = Self::calc_quality(&running_skill_score);
            check_result.success = true;
            check_result.critical = true;
            return check_result;
        }

        for i in 0..3 {
            running_skill_score = Self::check_attribute(
                dice_values[i],
                check_result.attribute_values[i],
                running_skill_score,
                difficulty.clone(),
            );

            if running_skill_score < 0 {
                check_result.success = false;
                return check_result;
            }
        }

        check_result.quality = Self::calc_quality(&running_skill_score);
        check_result.success = true;
        return check_result;
    }

    fn check_critical_roll(dice_values: &Vec<i32>, value: i32) -> bool {
        if (dice_values[0] == value && dice_values[1] == value)
            || (dice_values[0] == value && dice_values[2] == value)
            || (dice_values[1] == value && dice_values[2] == value)
        {
            return true;
        }

        return false;
    }

    fn check_attribute(dice_value: i32,
        skill_value: i32,
        running_skill_score: i32,
        difficulty: i32,
    ) -> i32 {
        let mut run_skill_score = running_skill_score;
        if dice_value > (skill_value + difficulty) {
            run_skill_score =
                run_skill_score - (dice_value - (skill_value + difficulty));    
        }

        println!("Calc Score: {} - {} - {}",running_skill_score,dice_value,skill_value);

        return run_skill_score;
    }

    fn calc_quality(running_skill_score: &i32) -> i32 {
        if running_skill_score.to_owned() == 0 {
            return 1;
        } else if running_skill_score.to_owned() > 16 {
            return 6;
        } else {
            let quality = running_skill_score.to_owned() as f64 / 3.0;
            println!("Running Score: {}",running_skill_score);
            println!("Quality: {}",quality);
            return quality.ceil() as i32;
        }
    }
}