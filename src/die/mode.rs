pub enum RollMode {
    KeepHighest,
    KeepLowest,
    DropHighest,
    DropLowest,
    Reroll,
    Exploding,
    Compounding,
    Penetrating,
    CountSuccesses,
    CountFailures,
}

pub enum ComparisonMode {
    GreaterThan,
    LessThan,
    Equal,
}
