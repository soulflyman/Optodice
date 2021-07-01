use crate::check_result::{CheckResult, CheckResultStatus};


#[derive(Default)]
pub struct SpellCheckResult {
    pub spell_name: String,
    pub spell_points: i32,
    pub attribute_names: Vec<String>,
    pub attribute_values: Vec<i32>,
    pub dice_values: Vec<i32>,
    pub difficulty: i32,
    pub quality: i32,
    pub success: bool,
    pub critical: bool,
}

impl SpellCheckResult {    
    pub fn format(&self) -> String {
        let mut difficulty_str :String = String::default();
        if self.difficulty > 0 {
            difficulty_str.push_str("+");
            difficulty_str.push_str(self.difficulty.to_string().as_str());
        } else if self.difficulty < 0 {
            difficulty_str = self.difficulty.to_string();
        }

        let mut check_results = String::default();
        for i in 0..self.attribute_names.len() {
            check_results.push_str(format!("`{} {:>2} {:>2} = {:>2}\t[{:>2}]`\n", self.attribute_names[i], self.attribute_values[i], difficulty_str, (self.attribute_values[i] as i32 + difficulty_str.parse::<i32>().unwrap_or(0)), self.dice_values[i]).as_str());
        }

        let res = format!("**{spell_name}** {difficulty}\n \
                                    Fertigkeitswert {spell_points}\n \
                                    {check_results}\n \
                                    QS: {quality_level}",
                                    spell_name=self.spell_name.as_str(), 
                                    difficulty=difficulty_str, 
                                    spell_points=self.spell_points.to_string(), 
                                    check_results=check_results, 
                                    quality_level=self.quality.to_string());

        return res;
    }        

    pub fn to_check_result(&self) -> CheckResult {
        let status = match self.success {
            true => CheckResultStatus::Success,
            false => CheckResultStatus::Failure,
        };

        CheckResult {
            message: self.format(),    
            status: status,
            critical: self.critical,
        }
    }
}




