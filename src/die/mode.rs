pub enum RollMode {
    Normal,
    KeepHighest(i64),
    KeepLowest(i64),
    DropHighest(i64),
    DropLowest(i64),
    Reroll(i64),
    Exploding(i64),
    Compounding(i64),
    Penetrating(i64),
    CountSuccesses(i64),
    CountFailures(i64),
}

pub enum ComparisonMode {
    GreaterThan,
    LessThan,
    Equal,
}
