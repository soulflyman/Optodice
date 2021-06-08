use std::collections::HashMap;

use serde_yaml;

use crate::optolith::combat_technique::OptolithCombatTechnique;

#[derive(Debug, Clone)]
pub struct OptolithCombatTechniques {
    list: Vec<OptolithCombatTechnique>,
    by_id: HashMap<String, OptolithCombatTechnique>,
}

impl OptolithCombatTechniques {
    pub fn new() -> OptolithCombatTechniques {
        let f = std::fs::File::open("CombatTechniques.yaml").unwrap();
        let ct_list: Vec<OptolithCombatTechnique> = serde_yaml::from_reader(f).unwrap();
        let mut by_id: HashMap<String, OptolithCombatTechnique> = HashMap::new();

        for ct in ct_list.clone() {
            by_id.insert(ct.id(), ct);
        }

        OptolithCombatTechniques {
            list: ct_list,
            by_id,
        }
    }

    //todo move this int OptolithCombatTechnique and use OptolitComabatTechniques.get().primary() instead 
    pub fn primary_attributes(&self, combat_technique_id: &String) -> Vec<String> {
        self.by_id.get(combat_technique_id).unwrap().primary()
    }
}