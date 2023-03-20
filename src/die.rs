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
                        result.results.push(die_roll);
                        break;
                    }
                    die_roll = self.engine_roll();
                }
            }
            ComparisonMode::LessThan => {
                while let RollMode::Reroll(target) = self.roll_mode {
                    if die_roll > target {
                        result.results.push(die_roll);
                        break;
                    }
                    die_roll = self.engine_roll();
                }
            }
            ComparisonMode::GreaterThan => {
                while let RollMode::Reroll(target) = self.roll_mode {
                    if die_roll < target {
                        result.results.push(die_roll);
                        break;
                    }
                    die_roll = self.engine_roll();
                }
            }
        };
        result
    }

    fn exploding(&self) -> RollResult {
        let mut result = RollResult::new();
        let mut die_roll = self.engine_roll();
        match self.comparison_mode {
            ComparisonMode::Equal => {
                while let RollMode::Exploding(target) = self.roll_mode {
                    if die_roll != target {
                        result.results.push(die_roll);
                        break;
                    }
                    result.results.push(die_roll);
                    die_roll = self.engine_roll();
                }
            }
            ComparisonMode::LessThan => {
                while let RollMode::Exploding(target) = self.roll_mode {
                    if die_roll >= target {
                        result.results.push(die_roll);
                        break;
                    }
                    result.results.push(die_roll);
                    die_roll = self.engine_roll();
                }
            }
            ComparisonMode::GreaterThan => {
                while let RollMode::Exploding(target) = self.roll_mode {
                    if die_roll <= target {
                        result.results.push(die_roll);
                        break;
                    }
                    result.results.push(die_roll);
                    die_roll = self.engine_roll();
                }
            }
        }
        result
    }

    fn compounding(&self) -> RollResult {
        let mut result = RollResult::new();
        let mut die_roll = self.engine_roll();
        let mut compounding_total = 0;
        match self.comparison_mode {
            ComparisonMode::Equal => {
                while let RollMode::Compounding(target) = self.roll_mode {
                    if die_roll != target {
                        compounding_total += die_roll;
                        result.results.push(compounding_total);
                        break;
                    }
                    compounding_total += die_roll;
                    die_roll = self.engine_roll();
                }
            }
            ComparisonMode::LessThan => {
                while let RollMode::Compounding(target) = self.roll_mode {
                    if die_roll >= target {
                        compounding_total += die_roll;
                        result.results.push(compounding_total);
                        break;
                    }
                    compounding_total += die_roll;
                    die_roll = self.engine_roll();
                }
            }
            ComparisonMode::GreaterThan => {
                while let RollMode::Compounding(target) = self.roll_mode {
                    if die_roll <= target {
                        compounding_total += die_roll;
                        result.results.push(compounding_total);
                        break;
                    }
                    compounding_total += die_roll;
                    die_roll = self.engine_roll();
                }
            }
        }
        result
    }

    fn penetrating(&self) -> RollResult {
        let mut result = RollResult::new();
        let mut die_roll = self.engine_roll();
        let mut penetrated = false;
        match self.comparison_mode {
            ComparisonMode::Equal => {
                while let RollMode::Penetrating(target) = self.roll_mode {
                    if die_roll != target {
                        if penetrated {
                            die_roll -= 1
                        }
                        result.results.push(die_roll);
                        break;
                    }
                    if penetrated {
                        die_roll -= 1;
                    }
                    penetrated = true;
                    result.results.push(die_roll);
                    die_roll = self.engine_roll();
                }
            }
            ComparisonMode::LessThan => {
                while let RollMode::Penetrating(target) = self.roll_mode {
                    if die_roll >= target {
                        if penetrated {
                            die_roll -= 1;
                        }
                        result.results.push(die_roll);
                        break;
                    }
                    if penetrated {
                        die_roll -= 1;
                    }
                    penetrated = true;
                    result.results.push(die_roll);
                    die_roll = self.engine_roll();
                }
            }
            ComparisonMode::GreaterThan => {
                while let RollMode::Penetrating(target) = self.roll_mode {
                    if die_roll <= target {
                        if penetrated {
                            die_roll -= 1;
                        }
                        result.results.push(die_roll);
                        break;
                    }
                    penetrated = true;
                    result.results.push(die_roll);
                    die_roll = self.engine_roll();
                }
            }
        }
        result
    }

    fn failure(&self) -> RollResult {
        let mut result = RollResult::new();
        let die_roll = self.engine_roll();
        match self.comparison_mode {
            ComparisonMode::Equal => {
                if let RollMode::Failure(target) = self.roll_mode {
                    if die_roll != target {
                        result.results.push(die_roll);
                    }
                }
            }
            ComparisonMode::LessThan => {
                if let RollMode::Failure(target) = self.roll_mode {
                    if die_roll >= target {
                        result.results.push(die_roll);
                    }
                }
            }
            ComparisonMode::GreaterThan => {
                if let RollMode::Failure(target) = self.roll_mode {
                    if die_roll <= target {
                        result.results.push(die_roll);
                    }
                }
            }
        }
        result
    }
    fn success(&self) -> RollResult {
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
            RollMode::Compounding(_) => self.compounding(),
            RollMode::Penetrating(_) => self.penetrating(),
            RollMode::Failure(_) => self.failure(),
            RollMode::Success(_) => self.success(),
        }
    }
}
