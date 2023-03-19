pub enum RollMode {
    Normal,
    KeepHighest(i64),
    KeepLowest(i64),
    DropHighest(i64),
    DropLowest(i64),
    Reroll(ComparisonMode, i64),
    Exploding(ComparisonMode, i64),
    Compounding(ComparisonMode, i64),
    Penetrating(ComparisonMode, i64),
    CountSuccesses(ComparisonMode, i64),
    CountFailures(ComparisonMode, i64),
}

pub enum ComparisonMode {
    GreaterThan,
    LessThan,
    Equal,
}
