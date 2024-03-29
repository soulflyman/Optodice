use std::{collections::HashMap, fs};
use json::JsonValue;

#[derive(Debug, Default, Clone)]
pub struct OptolithSkills {
    by_id: HashMap<String, Skill>,
    pub by_group: HashMap<String, Vec<Skill>>,
    group_order: Vec<String>,
}

#[derive(Debug, Default, Clone)]
pub struct Skill {    
    pub id: String,
    pub group_id: String,
    pub name: String,
    pub check: Vec<String>,
    pub check_display: Vec<String>,
}

#[derive(Debug, Default, Clone)]
struct SkillGroup {
    id: String,
    name: String,
    check: Vec<String>,
}

impl OptolithSkills {
    pub fn new() -> OptolithSkills {
        let path = "./optolith-data/custom/skills.json";
        let json_data = fs::read_to_string(path).expect("Unable to read file");
        let skills_json: JsonValue = json::parse(&json_data).expect("Error: Parsing of json data failed.");

        let mut skills = OptolithSkills::default();

        for (group, tmp_skills) in skills_json.entries() {
            let skill_group = SkillGroup {
                id: group.to_string(),
                name: group.to_string(),
                ..SkillGroup::default()
            };
            
            let mut grouped_skills: Vec<Skill> = Vec::new();
            for (skill_id, skill_values) in tmp_skills.entries() {
                let mut check: Vec<String> = vec!();
                for check_id in skill_values["test"].members() {
                    check.push(check_id.to_string());
                }

                let skill = Skill{
                    id: skill_id.to_string(),
                    group_id: skill_group.id.clone(),
                    name: skill_values["name"].to_string(),
                    check_display: check.clone(),
                    check,
                };

                skills.by_id.insert(skill_id.to_string(), skill.clone());    
                grouped_skills.push(skill);
            }

            skills.by_group.insert(skill_group.id.clone(), grouped_skills);
            skills.group_order.push(skill_group.id.clone());
        }

        return skills;
    }
  
    pub fn by_id(&self, skill_id: &String) -> Skill {
        return self.by_id.get(skill_id).unwrap().clone();
    }

    /// Get a reference to the optolith skills's group order.
    pub fn group_order(&self) -> Vec<String> {
        self.group_order.clone()
    }
}

impl Skill {
    pub fn check_list(&self) -> Vec<String> {
        return self.check.clone();
    }

    pub fn name(&self) -> String {
        return self.name.clone();
    }
}