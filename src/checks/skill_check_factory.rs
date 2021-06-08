
use crate::Context;
use std::collections::HashMap;
use crate::checks::skill_check::SkillCheck;

#[derive(Debug, Clone)]
pub struct SkillCheckFactory {
    skill_checks: HashMap<String, SkillCheck>,
    context: Context,
}

impl SkillCheckFactory {
    pub fn new(context: &mut Context) -> SkillCheckFactory {   
        SkillCheckFactory{
            skill_checks: HashMap::new(),
            context: context.clone(),
        }
    }
 
    pub fn get_skill_check(&mut self, skill_id :String) -> SkillCheck {
        if !self.skill_checks.contains_key(&skill_id) {            
            let new_abilit_check = SkillCheck::new(&mut self.context, skill_id.clone());
            self.skill_checks.insert(skill_id.clone(), new_abilit_check.clone());
            return new_abilit_check;
        }else{
            return self.skill_checks.get(&skill_id).expect("Error: This should not happen, case #3294").to_owned();
        }
    }
}