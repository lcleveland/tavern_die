use tavern_die;

#[test]
fn normal_equal_test() {
    let mut die = tavern_die::die::Die::default();
    let mut engine = tavern_die::rng_engine::test_engine::TestEngine::new();
    engine.rolls = vec![69];
    die.engine = Box::new(engine);
    assert_eq!(die.roll().dice_rolls[0], 69);
}
