#[derive(PartialEq, Eq)]
pub enum CheckResultStatus {
    Success,
    Failure,
    Information,
    None,
}

impl Default for CheckResultStatus {
    fn default() -> CheckResultStatus {
        CheckResultStatus::None
    }
}

#[derive(Default)]
pub struct CheckResult {
    pub message: String,
    pub critical: bool,
    pub status: CheckResultStatus,
}
