

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, PartialOrd, Eq, Ord)]
#[serde(rename_all = "camelCase")]
pub struct Spell {
    name: String, //"Adlerauge"
    id: String, //SPELL_1    
    effect: String, // Das Talent *Sinnesschärfe* wird während der Wirkungsdauer um QS +3 des Zaubers erhöht.
    casting_time: String, //"2 Aktionen"
    casting_time_short: String, //"2 Akt"
    ae_cost: String, //"4 AsP (Aktivierung des Zaubers) + 2 AsP pro 5 Minuten"
    ae_cost_short: String, //"4 AsP + 2 AsP pro 5 Min"
    range: String, //"selbst"
    range_short: String, //"selbst"
    duration: String, //"aufrechterhaltend"
    duration_short: String, //"(A)"
    target:String, // "Wesen"
    #[serde(default)]
    points: i32,
    #[serde(default)]
    check: Vec<String>,
}

impl Spell {
      /// Get a reference to the spell's id.
    pub fn id(&self) -> String {
        self.id.clone()
    }

    /// Get a reference to the spell's name.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Set the spell's points.
    pub fn set_points(&mut self, points: i32) {
        self.points = points;
    }

    /// Set the spell's check.
    pub fn set_check(&mut self, check: Vec<String>) {
        self.check = check;
    }

    /// Get a reference to the spell's points.
    pub fn points(&self) -> i32 {
        self.points.clone()
    }

    /// Get a reference to the spell's check.
    pub fn check(&self) -> &[String] {
        self.check.as_slice()
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpellUniv {
    pub id: String,
    pub check1: String,
    pub check2: String,
    pub check3: String,
    ic: String,
    casting_time_no_mod: bool,
    ae_cost_no_mod: bool,
    range_no_mod: bool,
    duration_no_mod: bool,
    traditions: Vec<i32>,
    property: i32,
    gr: i32,
}