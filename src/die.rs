pub mod mode;
pub mod roll_result;
pub mod traits;

use crate::die::mode::{ComparisonMode, RollMode};
use crate::die::roll_result::RollResult;
use crate::die::traits::rollable::Rollable;
use crate::rng_engine::prng_engine::PrngEngine;
use crate::rng_engine::traits::engine::Engine;

pub struct Die {
    pub roll_mode: RollMode,
    pub comparison_mode: ComparisonMode,
    pub sides: i64,
    pub engine: Box<dyn Engine>,
}

impl Die {
    pub fn new(
        roll_mode: RollMode,
        comparison_mode: ComparisonMode,
        engine: Box<dyn Engine>,
        sides: i64,
    ) -> Die {
        Die {
            roll_mode,
            comparison_mode,
            engine,
            sides,
        }
    }

    fn normal_roll(&self) -> RollResult {
        let mut result = RollResult::new();
        result.results.push(self.engine.random(1, self.sides));
        result
    }

    fn reroll(&self) -> RollResult {
        let mut result = RollResult::new();
        match self.comparison_mode {
            ComparisonMode::Equal => {
                let die_roll: i64 = self.engine.random(1, self.sides);
                while let RollMode::Reroll(die_roll) = self.roll_mode {
                    die_roll = self.engine.random(1, self.sides);
                }
                result.results.push(die_roll);
            }
            ComparisonMode::LessThan => {}
            ComparisonMode::GreaterThan => {}
        };
        result
    }
}

impl Default for Die {
    fn default() -> Die {
        Die::new(
            RollMode::Normal,
            ComparisonMode::Equal,
            Box::new(PrngEngine::new()),
            20,
        )
    }
}

impl Rollable for Die {
    fn roll(&self) -> RollResult {
        match self.roll_mode {
            RollMode::Normal => self.normal_roll(),
            RollMode::Reroll(_) => self.normal_roll(),
            RollMode::Exploding(_) => self.normal_roll(),
            RollMode::KeepLowest(_) => self.normal_roll(),
            RollMode::DropLowest(_) => self.normal_roll(),
            RollMode::KeepHighest(_) => self.normal_roll(),
            RollMode::DropHighest(_) => self.normal_roll(),
            RollMode::Compounding(_) => self.normal_roll(),
            RollMode::Penetrating(_) => self.normal_roll(),
            RollMode::CountFailures(_) => self.normal_roll(),
            RollMode::CountSuccesses(_) => self.normal_roll(),
        }
    }
}
