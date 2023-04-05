/// Represents the dice_rolls returned when a die is rolled
pub struct RollResult {
    /// Holds all the rolls of a die, based on mode
    pub dice_rolls: Vec<i64>,
}

impl RollResult {
    /// Creates a new roll result
    pub fn new() -> RollResult {
        RollResult {
            dice_rolls: Vec::new(),
        }
    }

    /// Add all dice together
    pub fn sum(&self) -> i64 {
        self.dice_rolls.iter().sum()
    }
}

impl Default for RollResult {
    fn default() -> RollResult {
        RollResult::new()
    }
}
