use tavern_die;

#[test]
fn normal_equal_test() {
    let mut die = tavern_die::die::Die::default();
    let mut engine = tavern_die::rng_engine::test_engine::TestEngine::new();
    engine.rolls = vec![69];
    die.engine = Box::new(engine);
    assert_eq!(die.roll().dice_rolls[0], 69);
}

#[test]
fn normal_greater_than_test() {
    let mut die = tavern_die::die::Die::default();
    let mut engine = tavern_die::rng_engine::test_engine::TestEngine::new();
    engine.rolls = vec![69];
    die.comparison_mode = tavern_die::die::mode::ComparisonMode::GreaterThan(68);
    die.engine = Box::new(engine);
    assert_eq!(die.roll().dice_rolls[0], 69);
}

#[test]
fn normal_less_than_test() {
    let mut die = tavern_die::die::Die::default();
    let mut engine = tavern_die::rng_engine::test_engine::TestEngine::new();
    engine.rolls = vec![67];
    die.comparison_mode = tavern_die::die::mode::ComparisonMode::LessThan(68);
    die.engine = Box::new(engine);
    assert_eq!(die.roll().dice_rolls[0], 67);
}

#[test]
fn reroll_equal_test() {
    let mut die = tavern_die::die::Die::default();
    let mut engine = tavern_die::rng_engine::test_engine::TestEngine::new();
    engine.rolls = vec![3, 69];
    die.roll_mode = tavern_die::die::mode::RollMode::Reroll;
    die.comparison_mode = tavern_die::die::mode::ComparisonMode::Equal(3);
    die.engine = Box::new(engine);
    assert_eq!(die.roll().dice_rolls[0], 69);
}

#[test]
fn reroll_greater_than_test() {
    let mut die = tavern_die::die::Die::default();
    let mut engine = tavern_die::rng_engine::test_engine::TestEngine::new();
    engine.rolls = vec![69, 3];
    die.roll_mode = tavern_die::die::mode::RollMode::Reroll;
    die.comparison_mode = tavern_die::die::mode::ComparisonMode::GreaterThan(20);
    die.engine = Box::new(engine);
    assert_eq!(die.roll().dice_rolls[0], 3);
}

#[test]
fn reroll_less_than_test() {
    let mut die = tavern_die::die::Die::default();
    let mut engine = tavern_die::rng_engine::test_engine::TestEngine::new();
    engine.rolls = vec![3, 69];
    die.roll_mode = tavern_die::die::mode::RollMode::Reroll;
    die.comparison_mode = tavern_die::die::mode::ComparisonMode::LessThan(20);
    die.engine = Box::new(engine);
    assert_eq!(die.roll().dice_rolls[0], 69);
}

#[test]
fn exploding_equal_test() {
    let mut die = tavern_die::die::Die::default();
    let mut engine = tavern_die::rng_engine::test_engine::TestEngine::new();
    die.roll_mode = tavern_die::die::mode::RollMode::Exploding;
    die.comparison_mode = tavern_die::die::mode::ComparisonMode::Equal(5);
    engine.rolls = vec![5, 5, 59];
    die.engine = Box::new(engine);
    assert_eq!(die.roll().sum(), 69);
}

#[test]
fn exploding_greater_than_test() {
    let mut die = tavern_die::die::Die::default();
    let mut engine = tavern_die::rng_engine::test_engine::TestEngine::new();
    die.roll_mode = tavern_die::die::mode::RollMode::Exploding;
    die.comparison_mode = tavern_die::die::mode::ComparisonMode::GreaterThan(4);
    engine.rolls = vec![5, 5, 55, 4];
    die.engine = Box::new(engine);
    assert_eq!(die.roll().sum(), 69);
}

#[test]
fn exploding_less_than_test() {
    let mut die = tavern_die::die::Die::default();
    let mut engine = tavern_die::rng_engine::test_engine::TestEngine::new();
    die.roll_mode = tavern_die::die::mode::RollMode::Exploding;
    die.comparison_mode = tavern_die::die::mode::ComparisonMode::LessThan(4);
    engine.rolls = vec![3, 3, 3, 60];
    die.engine = Box::new(engine);
    assert_eq!(die.roll().sum(), 69);
}

#[test]
fn compounding_equal_test() {
    let mut die = tavern_die::die::Die::default();
    let mut engine = tavern_die::rng_engine::test_engine::TestEngine::new();
    die.roll_mode = tavern_die::die::mode::RollMode::Compounding;
    die.comparison_mode = tavern_die::die::mode::ComparisonMode::Equal(20);
    engine.rolls = vec![20, 20, 20, 9];
    die.engine = Box::new(engine);
    assert_eq!(die.roll().dice_rolls[0], 69);
}

#[test]
fn compounding_greater_than_test() {
    let mut die = tavern_die::die::Die::default();
    let mut engine = tavern_die::rng_engine::test_engine::TestEngine::new();
    die.roll_mode = tavern_die::die::mode::RollMode::Compounding;
    die.comparison_mode = tavern_die::die::mode::ComparisonMode::GreaterThan(19);
    engine.rolls = vec![20, 20, 20, 9];
    die.engine = Box::new(engine);
    assert_eq!(die.roll().dice_rolls[0], 69);
}

#[test]
fn compounding_less_than_test() {
    let mut die = tavern_die::die::Die::default();
    let mut engine = tavern_die::rng_engine::test_engine::TestEngine::new();
    die.roll_mode = tavern_die::die::mode::RollMode::Compounding;
    die.comparison_mode = tavern_die::die::mode::ComparisonMode::LessThan(21);
    engine.rolls = vec![20, 20, 29];
    die.engine = Box::new(engine);
    assert_eq!(die.roll().dice_rolls[0], 69);
}

#[test]
fn penetrating_equal_test() {
    let mut die = tavern_die::die::Die::default();
    let mut engine = tavern_die::rng_engine::test_engine::TestEngine::new();
    die.roll_mode = tavern_die::die::mode::RollMode::Penetrating;
    die.comparison_mode = tavern_die::die::mode::ComparisonMode::Equal(21);
    engine.rolls = vec![21, 21, 29];
    die.engine = Box::new(engine);
    assert_eq!(die.roll().sum(), 69);
}

#[test]
fn penetrating_greater_than_test() {
    let mut die = tavern_die::die::Die::default();
    let mut engine = tavern_die::rng_engine::test_engine::TestEngine::new();
    die.roll_mode = tavern_die::die::mode::RollMode::Penetrating;
    die.comparison_mode = tavern_die::die::mode::ComparisonMode::GreaterThan(20);
    engine.rolls = vec![21, 21, 21, 9];
    die.engine = Box::new(engine);
    assert_eq!(die.roll().sum(), 69);
}

#[test]
fn penetrating_less_than_test() {
    let mut die = tavern_die::die::Die::default();
    let mut engine = tavern_die::rng_engine::test_engine::TestEngine::new();
    die.roll_mode = tavern_die::die::mode::RollMode::Penetrating;
    die.comparison_mode = tavern_die::die::mode::ComparisonMode::LessThan(22);
    engine.rolls = vec![21, 21, 29];
    die.engine = Box::new(engine);
    assert_eq!(die.roll().sum(), 69);
}
