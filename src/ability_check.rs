use crate::optolith::optolith::*;
use crate::test_result::TestResult;
use json::JsonValue;
use rand::prelude::*;

#[derive(Debug, Default, Clone)]
pub struct AbilityCheck {
    ability_name: String,
    ability_score: i32,
    skill_names: Vec<String>,
    skill_keys: Vec<String>,
    skill_values: Vec<i32>,
    dice_values: Vec<i32>,
    quality: i32,
}


impl AbilityCheck {
    pub fn new(
        heroes: OptolithHeroes,
        hero_id: String,
        mapping: JsonValue,
        skill_id: String,
    ) -> AbilityCheck {
        let mut ability_check = AbilityCheck::default();

        for (_, map) in mapping.entries() {
            if map.has_key(skill_id.as_str()) {
                ability_check.ability_name = map[skill_id.clone()]["name"].to_string();
                for skill in map[skill_id.clone()]["test"].members() {
                    ability_check.skill_keys.push(skill.to_string());
                }
                break;
            }
        }

        for skill_key in ability_check.skill_keys.iter() {
            ability_check
                .skill_names
                .push(mapping["Attribute"][skill_key]["token"][0].to_string());
            ability_check
                .skill_values
                .push(heroes.get_skill_value(&hero_id, &skill_key));
        }

        ability_check.ability_score = heroes.get_skill_value(&hero_id, &skill_id);

        return ability_check;
    }

    pub fn check_ability(&mut self, difficulty :&i32) -> TestResult {
        let mut test_result = TestResult::default();
        let mut running_ability_score = self.ability_score;

        test_result.difficulty = difficulty.clone();
        test_result.ability_score = self.ability_score;
        test_result.ability_name = self.ability_name.clone();
        test_result.skill_values = self.skill_values.clone();
        test_result.skill_names = self.skill_names.clone();

        let mut rng = rand::thread_rng();
        self.dice_values.push(rng.gen_range(1..21));
        self.dice_values.push(rng.gen_range(1..21));
        self.dice_values.push(rng.gen_range(1..21));

        test_result.dice_values = self.dice_values.clone();

        if self.check_critical_roll(20) {
            // Kritischer Patzer
            test_result.success = false;
            return test_result;
        }

        if self.check_critical_roll(1) {
            // Kritischer Erfolg
            test_result.quality = self.calc_quality(&running_ability_score);
            test_result.success = true;
            return test_result;
        }

        for i in 0..3 {
            running_ability_score = self.check_skill(
                self.dice_values[i],
                self.skill_values[i],
                running_ability_score,
                difficulty.clone(),
            );

            if running_ability_score < 0 {
                test_result.success = false;
                return test_result;
            }
        }

        test_result.quality = self.calc_quality(&running_ability_score);
        test_result.success = true;
        return test_result;
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
            let mut quality = running_ability_score.to_owned() as f64 / 3.0;
            println!("Running Score: {}",running_ability_score);
            println!("Quality: {}",quality);
            return quality.ceil() as i32;
        }
    }
}