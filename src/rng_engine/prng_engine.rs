use crate::rng_engine::traits::engine::Engine;
use rand::prelude::*;

pub struct PrngEngine {}
impl Engine for PrngEngine {
    fn random(&self, min: i64, max: i64) -> i64 {
        return rand::thread_rng().gen_range(min..max);
    }
}
