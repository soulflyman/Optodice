use crate::context::Context;
use crate::check_result::CheckResult;
use rand::prelude::*;
#[derive(Debug, Default, Clone)]
pub struct SkillCheck {
    skill_name: String,
    skill_points: i32,
    attribute_names: Vec<String>,
    attribute_keys: Vec<String>,
    attribute_values: Vec<i32>,
    dice_values: Vec<i32>,
    quality: i32,
}

impl SkillCheck {
    pub fn new(
        context: &Context,   
        skill_id: String,
    ) -> SkillCheck {
        let mut skill_check = SkillCheck::default();

        skill_check.skill_name = context.skills.by_id(&skill_id).get_name();
        skill_check.attribute_keys = context.skills.by_id(&skill_id).get_check();
        

        for attrribute_id in skill_check.attribute_keys.iter() {
            skill_check
                .attribute_names
                .push(context.attributes.by_id(attrribute_id).name_abbr);
            skill_check
                .attribute_values
                .push(context.heroes.active_hero().attribute_value(&attrribute_id));
        }

        skill_check.skill_points = context.heroes.active_hero().skill_points(&skill_id);

        return skill_check;
    }

    pub fn check_ability(&mut self, difficulty :&i32) -> CheckResult {
        let mut check_result = CheckResult::default();
        let mut running_ability_score = self.skill_points;

        check_result.difficulty = difficulty.clone();
        check_result.skill_points = self.skill_points;
        check_result.skill_name = self.skill_name.clone();
        check_result.attribute_values = self.attribute_values.clone();
        check_result.attribute_names = self.attribute_names.clone();

        let mut rng = rand::thread_rng();
        self.dice_values.push(rng.gen_range(1..21));
        self.dice_values.push(rng.gen_range(1..21));
        self.dice_values.push(rng.gen_range(1..21));

        check_result.dice_values = self.dice_values.clone();

        if self.check_critical_roll(20) {
            // Kritischer Patzer
            check_result.success = false;
            return check_result;
        }

        if self.check_critical_roll(1) {
            // Kritischer Erfolg
            check_result.quality = self.calc_quality(&running_ability_score);
            check_result.success = true;
            return check_result;
        }

        for i in 0..3 {
            running_ability_score = self.check_skill(
                self.dice_values[i],
                self.attribute_values[i],
                running_ability_score,
                difficulty.clone(),
            );

            if running_ability_score < 0 {
                check_result.success = false;
                return check_result;
            }
        }

        check_result.quality = self.calc_quality(&running_ability_score);
        check_result.success = true;
        return check_result;
    }

    fn check_critical_roll(&self, value: i32) -> bool {
        if (self.dice_values[0] == value && self.dice_values[1] == value)
            || (self.dice_values[0] == value && self.dice_values[2] == value)
            || (self.dice_values[1] == value && self.dice_values[2] == value)
        {
            return true;
        }

        return false;
    }

    fn check_skill(&self,
        dice_value: i32,
        skill_value: i32,
        running_ability_score: i32,
        difficulty: i32,
    ) -> i32 {
        let mut run_ability_score = running_ability_score;
        if dice_value > (skill_value + difficulty) {
            run_ability_score =
                run_ability_score - (dice_value - (skill_value + difficulty));    
        }

        println!("Calc Score: {} - {} - {}",running_ability_score,dice_value,skill_value);

        return run_ability_score;
    }

    fn calc_quality(&self, running_ability_score: &i32) -> i32 {
        if running_ability_score.to_owned() == 0 {
            return 1;
        } else if running_ability_score.to_owned() > 16 {
            return 6;
        } else {
            let quality = running_ability_score.to_owned() as f64 / 3.0;
            println!("Running Score: {}",running_ability_score);
            println!("Quality: {}",quality);
            return quality.ceil() as i32;
        }
    }
}