
#[derive(Default)]
pub struct TestResult {
    pub ability_name: String,
    pub ability_score: i32,
    pub skill_names: Vec<String>,
    pub skill_values: Vec<i32>,
    pub dice_values: Vec<i32>,
    pub difficulty: i32,
    pub quality: i32,
    pub success: bool,
}

impl TestResult {    
    pub fn get_formated(&self) -> String {
        let mut res = String::default();
        res.push_str("**");
        res.push_str(self.ability_name.as_str());
        res.push_str("-Probe**\n");

        let mut difficulty_str :String = String::default();
        if self.difficulty > 0 {
            difficulty_str.push_str("+");
            difficulty_str.push_str(self.difficulty.to_string().as_str());
        }else{
            difficulty_str = self.difficulty.to_string();
        }

        res.push_str("**Modifikation: ");
        res.push_str(difficulty_str.as_str());
        res.push_str("**\n");        
                
        res.push_str("Skillwert ");
        res.push_str(self.ability_score.to_string().as_str());
        res.push_str("\n");

        res.push_str("```\n");
        for i in 0..self.skill_names.len() {
            res.push_str(format!("{} {:>2} {:>2} = {:>2}\t[{:>2}]\n", self.skill_names[i], self.skill_values[i], difficulty_str, (self.skill_values[i] as i32 + difficulty_str.parse::<i32>().unwrap()), self.dice_values[i]).as_str());
        }
        res.push_str("```\n");

        if self.skill_names.len() > 1 {
            res.push_str("QS: ");
            res.push_str(self.quality.to_string().as_str());
        }
        return res;
    }        

    pub fn is_success(&self) -> bool {
        return self.success;
    }
}




