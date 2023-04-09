/// Represents the dice_rolls returned when a die is rolled
pub struct DieResult {
    /// Holds all the rolls of a die, based on mode
    pub dice_rolls: Vec<i64>,
}

impl DieResult {
    /// Creates a new roll result
    pub fn new() -> DieResult {
        DieResult {
            dice_rolls: Vec::new(),
        }
    }

    /// Add all dice together
    pub fn sum(&self) -> i64 {
        self.dice_rolls.iter().sum()
    }
}

impl Default for DieResult {
    fn default() -> DieResult {
        DieResult::new()
    }
}
