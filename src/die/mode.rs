pub enum RollMode {
    Normal,
    Reroll,
    Exploding,
    Compounding,
    Penetrating,
    Success,
    Failure,
}

pub enum ComparisonMode {
    GreaterThan(i64),
    LessThan(i64),
    Equal(i64),
}
