use std::{collections::HashMap, fs};
use json::JsonValue;

#[derive(Debug, Default, Clone)]
pub struct OptolithAttributes {
    by_id: HashMap<String, Attribute>
}

#[derive(Debug, Default, Clone)]
pub struct Attribute {    
    pub id: String,
    pub name: String,
    pub name_abbr: String,
}

impl OptolithAttributes {
    pub fn new() -> OptolithAttributes {
        let path = "./attributes.json";
        let json_data = fs::read_to_string(path).expect("Unable to read file");
        let attributes_json: JsonValue = json::parse(&json_data).expect("Error: Parsing of json data failed.");

        let mut attributes = OptolithAttributes::default();

        for (attribute_id, attribute_values) in attributes_json.entries() {
            attributes.by_id.insert(attribute_id.to_string(), Attribute{
              id: attribute_id.to_string(),
              name: attribute_values["name"].to_string(),
              name_abbr: attribute_values["nameAbbr"].to_string(),
            });
        }        

        return attributes;        
    }
  
    pub fn by_id(&self, attribute_id: &String) -> Attribute {
        return self.by_id.get(attribute_id).unwrap().clone();
    }

    pub fn get_name_abbrs(&self, attribute_ids: Vec<String>) -> Vec<String> {
        let mut name_abbrs: Vec<String> = vec!();
        for attribute_id in attribute_ids {
            name_abbrs.push(self.by_id(&attribute_id).name_abbr.clone());
        }
        return name_abbrs;
    }    

    pub fn all(&mut self) -> &HashMap<String, Attribute> {
        &self.by_id
    }
}
