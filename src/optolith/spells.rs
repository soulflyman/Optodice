use std::collections::HashMap;
use serde_yaml;
use super::spell::{Spell, SpellUniv};

#[derive(Debug, Default, Clone)]
pub struct Spells {
    list: Vec<Spell>,
    by_id: HashMap<String, Spell>,
}



impl Spells {
    pub fn new() -> Spells {
        let spells_univ_yaml = std::fs::File::open("./optolith-data/Data/univ/Spells.yaml").unwrap();
        let spells_univ: Vec<SpellUniv> = serde_yaml::from_reader(spells_univ_yaml).unwrap();
        let mut spells_univ_map : HashMap<String, SpellUniv> = HashMap::new();
        for spell in spells_univ {
            spells_univ_map.insert(spell.id.clone(), spell);
        }

        let spells_yaml = std::fs::File::open("./optolith-data/Data/de-DE/Spells.yaml").unwrap();
        let mut spells: Vec<Spell> = serde_yaml::from_reader(spells_yaml).unwrap();
        let mut by_id: HashMap<String, Spell> = HashMap::new();

        for mut spell in spells.clone() {
            let mut check: Vec<String> = vec![];
            let check1 = spells_univ_map.get(&spell.id()).unwrap_or(&SpellUniv::default()).check1.clone();
            let check2 = spells_univ_map.get(&spell.id()).unwrap_or(&SpellUniv::default()).check2.clone();
            let check3 = spells_univ_map.get(&spell.id()).unwrap_or(&SpellUniv::default()).check3.clone();
            check.push(check1);
            check.push(check2);
            check.push(check3);
            spell.set_check(check);
            by_id.insert(spell.id(), spell);
        }

        spells.sort();

        Spells {
            list: spells,
            by_id,
        }
    }
  
    pub fn by_id(&self, spell_id: &str) -> Spell {
        return self.by_id.get(spell_id).unwrap().clone();
    }

    pub fn all(&self) -> Vec<Spell> {
        return self.list.clone();
    }
}

