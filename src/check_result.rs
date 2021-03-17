
#[derive(Default)]
pub struct CheckResult {
    pub skill_name: String,
    pub skill_points: i32,
    pub attribute_names: Vec<String>,
    pub attribute_values: Vec<i32>,
    pub dice_values: Vec<i32>,
    pub difficulty: i32,
    pub quality: i32,
    pub success: bool,
}

impl CheckResult {    
    pub fn get_formated(&self) -> String {
        let mut res = String::default();
        res.push_str("**");
        res.push_str(self.skill_name.as_str());
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
        res.push_str(self.skill_points.to_string().as_str());
        res.push_str("\n");

        res.push_str("```\n");
        for i in 0..self.attribute_names.len() {
            res.push_str(format!("{} {:>2} {:>2} = {:>2}\t[{:>2}]\n", self.attribute_names[i], self.attribute_values[i], difficulty_str, (self.attribute_values[i] as i32 + difficulty_str.parse::<i32>().unwrap()), self.dice_values[i]).as_str());
        }
        res.push_str("```\n");

        if self.attribute_names.len() > 1 {
            res.push_str("QS: ");
            res.push_str(self.quality.to_string().as_str());
        }
        return res;
    }        

    pub fn is_success(&self) -> bool {
        return self.success;
    }
}




