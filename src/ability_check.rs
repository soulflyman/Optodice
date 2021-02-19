

#[derive(Debug)]
pub struct AbilityCheck {
    ability_score: Option<u32>,
    skill_names : Vec<String>,
    skill_values: Vec<u32>,
    dice_values: Vec<u32>,
    quality : u32,
}

impl AbilityCheck {
    pub fn new() -> AbilityCheck {

    }

}