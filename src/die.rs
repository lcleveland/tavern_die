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

    fn normal(&self) -> RollResult {
        let mut result = RollResult::new();
        result.results.push(self.engine_roll());
        result
    }

    fn reroll(&self) -> RollResult {
        let mut result = RollResult::new();
        let mut die_roll: i64 = self.engine_roll();
        match self.comparison_mode {
            ComparisonMode::Equal => {
                while let RollMode::Reroll(target) = self.roll_mode {
                    if die_roll != target {
                        break;
                    }
                    die_roll = self.engine_roll();
                }
                result.results.push(die_roll);
            }
            ComparisonMode::LessThan => {
                while let RollMode::Reroll(target) = self.roll_mode {
                    if die_roll > target {
                        break;
                    }
                    die_roll = self.engine_roll();
                }
                result.results.push(die_roll);
            }
            ComparisonMode::GreaterThan => {
                while let RollMode::Reroll(target) = self.roll_mode {
                    if die_roll < target {
                        break;
                    }
                    die_roll = self.engine_roll();
                }
                result.results.push(die_roll);
            }
        };
        result
    }

    fn exploding(&self) -> RollResult {
        let mut result = RollResult::new();
        match self.comparison_mode {
            ComparisonMode::Equal => {}
            ComparisonMode::LessThan => {}
            ComparisonMode::GreaterThan => {}
        }
        result
    }

    fn keep_lowest(&self) -> RollResult {
        let mut result = RollResult::new();
        match self.comparison_mode {
            ComparisonMode::Equal => {}
            ComparisonMode::LessThan => {}
            ComparisonMode::GreaterThan => {}
        }
        result
    }

    fn keep_highest(&self) -> RollResult {
        let mut result = RollResult::new();
        match self.comparison_mode {
            ComparisonMode::Equal => {}
            ComparisonMode::LessThan => {}
            ComparisonMode::GreaterThan => {}
        }
        result
    }
    fn drop_highest(&self) -> RollResult {
        let mut result = RollResult::new();
        match self.comparison_mode {
            ComparisonMode::Equal => {}
            ComparisonMode::LessThan => {}
            ComparisonMode::GreaterThan => {}
        }
        result
    }

    fn drop_lowest(&self) -> RollResult {
        let mut result = RollResult::new();
        match self.comparison_mode {
            ComparisonMode::Equal => {}
            ComparisonMode::LessThan => {}
            ComparisonMode::GreaterThan => {}
        }
        result
    }

    fn compounding(&self) -> RollResult {
        let mut result = RollResult::new();
        match self.comparison_mode {
            ComparisonMode::Equal => {}
            ComparisonMode::LessThan => {}
            ComparisonMode::GreaterThan => {}
        }
        result
    }

    fn penetrating(&self) -> RollResult {
        let mut result = RollResult::new();
        match self.comparison_mode {
            ComparisonMode::Equal => {}
            ComparisonMode::LessThan => {}
            ComparisonMode::GreaterThan => {}
        }
        result
    }

    fn count_failures(&self) -> RollResult {
        let mut result = RollResult::new();
        match self.comparison_mode {
            ComparisonMode::Equal => {}
            ComparisonMode::LessThan => {}
            ComparisonMode::GreaterThan => {}
        }
        result
    }
    fn count_successes(&self) -> RollResult {
        let mut result = RollResult::new();
        match self.comparison_mode {
            ComparisonMode::Equal => {}
            ComparisonMode::LessThan => {}
            ComparisonMode::GreaterThan => {}
        }
        result
    }

    fn engine_roll(&self) -> i64 {
        self.engine.random(1, self.sides)
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
            RollMode::Normal => self.normal(),
            RollMode::Reroll(_) => self.reroll(),
            RollMode::Exploding(_) => self.exploding(),
            RollMode::KeepLowest(_) => self.keep_lowest(),
            RollMode::DropLowest(_) => self.drop_lowest(),
            RollMode::KeepHighest(_) => self.keep_highest(),
            RollMode::DropHighest(_) => self.drop_highest(),
            RollMode::Compounding(_) => self.compounding(),
            RollMode::Penetrating(_) => self.penetrating(),
            RollMode::CountFailures(_) => self.count_failures(),
            RollMode::CountSuccesses(_) => self.count_successes(),
        }
    }
}
