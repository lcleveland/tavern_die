use crate::rng_engine::traits::engine::Engine;

pub struct TestEngine {
    rolls: Vec<i64>,
}

impl TestEngine {
    pub fn new() -> TestEngine {
        TestEngine { rolls: Vec::new() }
    }
}

impl Engine for TestEngine {
    fn random(&self, min: i64, max: i64) -> i64 {
        *self.rolls.first().unwrap_or(&0)
    }
}
