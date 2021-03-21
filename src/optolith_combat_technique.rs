#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptolithCombatTechnique {
  id: String,  
  primary: Vec<String>,
  gr: i32,
}

impl OptolithCombatTechnique {
    /// Get a reference to the optolith combat technique's id.
    pub fn id(&self) -> String {
        self.id.clone()
    }

    /// Get a reference to the optolith combat technique's primary.
    pub fn primary(&self) -> Vec<String> {
        self.primary.clone()
    }

    pub fn group(&self) -> i32 {
        self.gr.clone()
    }
}