use crate::rng_engine::traits::engine::Engine;

pub struct TestEngine {
    pub rolls: Vec<i64>,
}

impl TestEngine {
    pub fn new() -> TestEngine {
        TestEngine { rolls: Vec::new() }
    }
}

impl Engine for TestEngine {
    fn random(&mut self, _min: i64, _max: i64) -> i64 {
        self.rolls.remove(0)
    }
}
