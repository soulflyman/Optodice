use crate::check_result::{CheckResult, CheckResultStatus};


#[derive(Default)]
pub struct BattleCheckResult {
    pub action_name: String,
    pub action_name_abbr: String,
    pub action_value: i32,
    pub dice_value: i32,
    pub difficulty: i32,    
    pub success: bool,
    pub critical: bool,
}

impl BattleCheckResult {    
    pub fn get_formated(&self) -> String {
        let mut difficulty_str :String = String::default();
        if self.difficulty > 0 {
            difficulty_str.push_str("+");
            difficulty_str.push_str(self.difficulty.to_string().as_str());
        } else if self.difficulty < 0 {
            difficulty_str = self.difficulty.to_string();
        }

        let check_results = format!("`{} {:>2} {:>2} = {:>2}\t[{:>2}]`\n", 
                                                                self.action_name_abbr, 
                                                                self.action_value, 
                                                                difficulty_str, 
                                                                (self.action_value + self.difficulty), 
                                                                self.dice_value.to_string().as_str());

        let res = format!("**{action_name}** {difficulty}\n \
                                    {action_name}wert {action_value}\n \
                                    {check_results}\n",
                                    action_name=self.action_name.as_str(), 
                                    difficulty=difficulty_str, 
                                    action_value=self.action_value.to_string(), 
                                    check_results=check_results);

        return res;
    }        

    pub fn is_success(&self) -> bool {
        return self.success;
    }

    pub fn to_check_result(&self) -> CheckResult {
        let status = match self.success {
            true => CheckResultStatus::Success,
            false => CheckResultStatus::Failure,
        };

        CheckResult {
            message: self.get_formated(),    
            status: status,
            critical: self.critical,
        }
    }
}




