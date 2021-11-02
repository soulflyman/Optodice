use std::fs;
use json::JsonValue;

#[derive(Debug, Default, Clone)]
pub struct OptolithAttributes {
    attributes: Vec<Attribute>
}

#[derive(Debug, Default, Clone)]
pub struct Attribute {    
    pub id: String,
    pub name: String,
    pub name_abbr: String,
}

impl OptolithAttributes {
    pub fn new() -> OptolithAttributes {
        let path = "./optolith-data/custom/attributes.json";
        let json_data = fs::read_to_string(path).expect(format!("Unable to read file {}", path).as_str());
        let attributes_json: JsonValue = json::parse(&json_data).expect(format!("Error: Parsing of json data failed {}", path).as_str());

        let mut attributes = OptolithAttributes::default();

        for (attribute_id, attribute_values) in attributes_json.entries() {
            attributes.attributes.push( Attribute{
              id: attribute_id.to_string(),
              name: attribute_values["name"].to_string(),
              name_abbr: attribute_values["nameAbbr"].to_string(),
            });
        }        

        return attributes;        
    }
  
    pub fn by_id(&self, attribute_id: &String) -> Attribute {
        for attribute in &self.attributes {
            if attribute.id == attribute_id.to_owned() {
                return attribute.clone();
            }
        }

        Attribute::default()
    }

    pub fn name_abbrs(&self, attribute_ids: Vec<String>) -> Vec<String> {
        let mut name_abbrs: Vec<String> = vec!();
        for attribute_id in attribute_ids {
            name_abbrs.push(self.by_id(&attribute_id).name_abbr.clone());
        }
        return name_abbrs;
    }    

    pub fn all(&self) -> &Vec<Attribute> {
        &self.attributes
    }
}

impl Attribute {
    pub fn name(&self) -> String {
        return self.name.clone();
    }

    pub fn abbr(&self) -> String {
        return self.name_abbr.clone();
    }
}
