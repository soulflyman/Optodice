use crate::check_result::CheckResult;


#[derive(Default)]
pub struct AttributeCheckResult {
    pub attribute_name: String,
    pub attribute_name_abbr: String,
    pub attribute_value: i32,
    pub dice_value: i32,
    pub difficulty: i32,    
    pub success: bool,
    pub critical: bool,
}

impl AttributeCheckResult {    
    pub fn get_formated(&self) -> String {
        let mut difficulty_str :String = String::default();
        if self.difficulty > 0 {
            difficulty_str.push_str("+");
            difficulty_str.push_str(self.difficulty.to_string().as_str());
        } else if self.difficulty < 0 {
            difficulty_str = self.difficulty.to_string();
        }

        let check_results = format!("{} {:>2} {:>2} = {:>2}\t[{:>2}]\n", 
                                                                self.attribute_name_abbr, 
                                                                self.attribute_value, 
                                                                difficulty_str, 
                                                                (self.attribute_value + self.difficulty), 
                                                                self.dice_value.to_string().as_str());

        let res = format!("**{attribute_name}-Probe** {difficulty}\n \
                                    Eigenschaftswert {attribute_value}\n \
                                    ```\n\
                                    {check_results}\
                                    ```",
                                    attribute_name=self.attribute_name.as_str(), 
                                    difficulty=difficulty_str, 
                                    attribute_value=self.attribute_value.to_string(), 
                                    check_results=check_results);

        return res;
    }        

    pub fn is_success(&self) -> bool {
        return self.success;
    }

    pub fn to_check_result(&self) -> CheckResult {
        CheckResult {
            message: self.get_formated(),    
            success: self.success,
            critical: self.critical,
        }
    }
}




