pub struct TestResult {
    pub ability_name: String,
    pub ability_score: Option<i32>,
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
                
        if self.ability_score.is_some() {
            res.push_str("Talentwert ");
            res.push_str(self.ability_score.unwrap_or(0).to_string().as_str());
            res.push_str("\n");
        }
        
        res.push_str("```\n");
        for i in 0..self.skill_names.len() {
            res.push_str(format!("{} {:>2}\t[{:>2}]\n", self.skill_names[i], self.skill_values[i], self.dice_values[i]).as_str());
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



