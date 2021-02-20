mod optolith;
use optolith;
use rand::prelude::*;

#[derive(Debug, Default)]
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
        let ability_check = AbilityCheck::default();

        for map in mapping.entries() {
            if map.contains(skill_id) {
                ability_check.ability_name = map[skill_id][name];
                for skill in map[skill_id][test] {
                    ability_check.skill_keys.insert(skill);
                }
                break;
            }
        }

        for skill_key in ability_check.skill_keys {
            ability_check
                .skill_names
                .insert(mapping[Attribute][skill_key]["token"]);
            ability_check
                .skill_value
                .insert(heroes.get_skill_value(&hero_id, &skill_key));
        }

        ability_check.ability_score = heroes.get_skill_value(&hero_id, &skill_id);

        return ability_check;
    }

    pub fn check_ability(&self, &difficulty: i32) -> TestResult::TestResult {
        let mut test_result = TestResult::default();
        let mut running_ability_score = self.ability_score;

        test_result.difficulty = difficulty;
        test_result.ability_score = self.ability_score;
        test_result.skill_values = self.skill_values;
        test_result.skill_names = self.skill_names;

        let mut rng = rand::thread_rng();
        self.dice_values.insert(rng.gen_range(1..21));
        self.dice_values.insert(rng.gen_range(1..21));
        self.dice_values.insert(rng.gen_range(1..21));

        test_result.dice_values = self.dice_values;

        if check_critical_roll(20) {
            // Kritischer Patzer
            test_result.success = false;
            return test_result;
        }

        if check_critical_roll(1) {
            // Kritischer Erfolg
            test_result.quality = calc_quality(&running_ability_score);
            test_result.success = true;
            return test_result;
        }

        for i in (0..2) {
            running_ability_score = check_skill(
                self.dice_values[i],
                self.skill_values[i],
                running_ability_score,
                self.difficulty,
            );

            if running_ability_score < 0 {
                test_result.success = false;
                return test_result;
            }
        }

        test_result.quality = calc_quality(running_ability_score);
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
    }

    fn check_skill(
        dice_value: i32,
        skill_value: i32,
        running_ability_score: i32,
        difficulty: i32,
    ) -> i32 {
        if dice_value > (skill_value + difficulty) {
            running_ability_score =
                running_ability_score - (dice_value - (skill_value + difficulty));
        }

        return running_ability_score;
    }

    fn calc_quality(running_ability_score: i32) -> i32 {
        if running_ability_score == 0 {
            return 1;
        } else if running_ability_score > 16 {
            return 6;
        } else {
            let mut quality = running_ability_score / 3;
            return quality.ceil();
        }
    }
}
