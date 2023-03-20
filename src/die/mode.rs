pub enum RollMode {
    Normal,
    Reroll(i64),
    Exploding(i64),
    Compounding(i64),
    Penetrating(i64),
    Success(i64),
    Failure(i64),
}

pub enum ComparisonMode {
    GreaterThan,
    LessThan,
    Equal,
}
