pub enum RollMode {
    /// This is a straight die roll, no comparison modes are used
    Normal,

    /// Reroll the die if comparison is met. Discards old die roll
    Reroll,

    /// If comparison is met adds die roll to results and rolls again
    Exploding,

    /// If comparison is met adds all die rolls that match together in results
    Compounding,

    /// If comparison is met adds die roll to results and rolls again, subtracting one from the
    /// penetrated roll
    Penetrating,

    /// If the comparison is met adds the die roll to results to show as a success
    Success,

    /// If the comparison is NOT met adds the die roll to results to show as a failure
    Failure,
}

pub enum ComparisonMode {
    /// Checks that the die roll is greater than the provided number
    GreaterThan(i64),

    /// Checks that the die roll is less than the provided number
    LessThan(i64),

    /// Checks that the die roll is equal to the provided number
    Equal(i64),
}
