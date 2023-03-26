/// Represents the results returned when a die is rolled
pub struct RollResult {
    /// Holds all the rolls of a die, based on mode
    pub results: Vec<i64>,
}

impl RollResult {
    /// Creates a new roll result
    pub fn new() -> RollResult {
        RollResult {
            results: Vec::new(),
        }
    }
}

impl Default for RollResult {
    fn default() -> RollResult {
        RollResult::new()
    }
}
