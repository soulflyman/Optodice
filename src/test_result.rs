pub struct TestResult {
    pub test_name: String,
    pub talent_value: Option<i32>,
    pub skills: Vec<String>,
    pub values: Vec<i32>,
    pub results: Vec<i32>,
    pub difficulty: i32,
    pub qs: i32,
    pub success: bool,
}

impl TestResult {    
    pub fn get_formated(&self) -> String {
        let mut res = String::default();
        res.push_str("**");
        res.push_str(self.test_name.as_str());
        res.push_str("-Probe**\n");
                
        if self.talent_value.is_some() {
            res.push_str("Talentwert ");
            res.push_str(self.talent_value.unwrap_or(0).to_string().as_str());
            res.push_str("\n");
        }
        
        res.push_str("```\n");
        for i in 0..self.skills.len() {
            res.push_str(format!("{} {:>2}\t[{:>2}]\n", self.skills[i], self.values[i], self.results[i]).as_str());
        }
        res.push_str("```\n");

        if self.skills.len() > 1 {
            res.push_str("QS: ");
            res.push_str(self.qs.to_string().as_str());
        }
        return res;
    }        

    pub fn is_success(&self) -> bool {
        return self.success;
    }
}



