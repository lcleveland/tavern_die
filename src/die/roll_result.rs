/// Represents the results returned when a die is rolled
pub struct RollResult {
    pub results: Vec<i64>,
}

impl RollResult {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_result_test() {
        let mut roll_result = RollResult::new();
        roll_result.results.push(1);
        assert_eq!(roll_result.results[0], 1);
    }
}
