use std::{collections::HashMap, fs};
use json::JsonValue;

#[derive(Debug, Default, Clone)]
pub struct OptolithSkills {
    by_id: HashMap<String, Skill>
}

#[derive(Debug, Default, Clone)]
pub struct Skill {    
    id: String,
    group_id: String,
    name: String,
    test: Vec<String>,
    test_display: Vec<String>,
}

#[derive(Debug, Default, Clone)]
struct SkillGroup {
    id: String,
    name: String,
    test: Vec<String>,
}

impl OptolithSkills {
    pub fn new() -> OptolithSkills {
        let path = "./skills.json";
        let json_data = fs::read_to_string(path).expect("Unable to read file");
        let skills_json: JsonValue = json::parse(&json_data).expect("Error: Parsing of json data failed.");

        let mut skills = OptolithSkills::default();

        for (group, tmp_skills) in skills_json.entries() {
            let skill_group = SkillGroup {
                id: group.to_string(),
                name: group.to_string(),
                ..SkillGroup::default()
            };
            
            for (skill_id, skill_values) in tmp_skills.entries() {
                let mut test: Vec<String> = vec!();
                for test_id in skill_values["test"].members() {
                    test.push(test_id.to_string());
                }

                skills.by_id.insert(skill_id.to_string(), Skill{
                    id: skill_id.to_string(),
                    group_id: skill_group.id.clone(),
                    name: skill_values["name"].to_string(),
                    test_display: test.clone(),
                    test: test,
                });
            }
        }

        return skills;        
    }
  
    pub fn by_id(&self, skill_id: &String) -> Skill {
        return self.by_id.get(skill_id).unwrap().clone();
    }
}

impl Skill {
    pub fn get_check(&self) -> Vec<String> {
        return self.test.clone();
    }

    pub fn get_name(&self) -> String {
        return self.name.clone();
    }
}