use crate::{config::Config, optolith_attributes::OptolithAttributes, optolith_heroes::optolith::OptolithHeroes, optolith_skills::OptolithSkills};
#[derive(Debug, Clone)]
pub struct Context {
    pub config: Config,
    pub heroes: OptolithHeroes,
    pub attributes: OptolithAttributes,
    pub skills: OptolithSkills,
}
