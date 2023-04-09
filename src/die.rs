pub mod mode;
pub mod result;

use crate::die::mode::{ComparisonMode, RollMode};
use crate::die::result::DieResult;
use crate::rng_engine::prng_engine::PrngEngine;
use crate::rng_engine::traits::engine::Engine;

/// A struct for rolling dice, with advanced features
pub struct Die {
    /// Controls the logic used when rolling the die
    pub roll_mode: RollMode,

    /// Controls how comparisons are made when rolling dice
    pub comparison_mode: ComparisonMode,

    /// How many sides the die has
    pub sides: i64,

    /// Controls how random numbers are generated
    pub engine: Box<dyn Engine>,
}

impl Die {
    /// Create a new die
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

    /// Roll a die with minimal logic
    fn normal(&mut self) -> DieResult {
        let mut result = DieResult::new();
        let die_roll: i64 = self.engine_roll();
        match self.comparison_mode {
            ComparisonMode::Equal(_) => {
                result.dice_rolls.push(die_roll);
            }
            ComparisonMode::LessThan(target) => {
                if die_roll < target {
                    result.dice_rolls.push(die_roll);
                }
            }
            ComparisonMode::GreaterThan(target) => {
                if die_roll > target {
                    result.dice_rolls.push(die_roll);
                }
            }
        }
        result
    }

    /// Roll a die and re-roll if matching criteria
    fn reroll(&mut self) -> DieResult {
        let mut result = DieResult::new();
        let mut die_roll: i64 = self.engine_roll();
        match self.comparison_mode {
            ComparisonMode::Equal(target) => loop {
                if die_roll != target {
                    result.dice_rolls.push(die_roll);
                    break;
                }

                println!("{die_roll}");
                die_roll = self.engine_roll();
            },
            ComparisonMode::LessThan(target) => loop {
                if die_roll >= target {
                    result.dice_rolls.push(die_roll);
                    break;
                }
                die_roll = self.engine_roll();
            },
            ComparisonMode::GreaterThan(target) => loop {
                if die_roll <= target {
                    result.dice_rolls.push(die_roll);
                    break;
                }
                die_roll = self.engine_roll();
            },
        };
        result
    }

    /// Roll a die and add more dice based on criteria
    fn exploding(&mut self) -> DieResult {
        let mut result = DieResult::new();
        let mut die_roll = self.engine_roll();
        match self.comparison_mode {
            ComparisonMode::Equal(target) => loop {
                if die_roll != target {
                    result.dice_rolls.push(die_roll);
                    break;
                }
                result.dice_rolls.push(die_roll);
                die_roll = self.engine_roll();
            },
            ComparisonMode::LessThan(target) => loop {
                if die_roll >= target {
                    result.dice_rolls.push(die_roll);
                    break;
                }
                result.dice_rolls.push(die_roll);
                die_roll = self.engine_roll();
            },
            ComparisonMode::GreaterThan(target) => loop {
                if die_roll <= target {
                    result.dice_rolls.push(die_roll);
                    break;
                }
                result.dice_rolls.push(die_roll);
                die_roll = self.engine_roll();
            },
        }
        result
    }

    /// Roll a die and add to the total based on criteria
    fn compounding(&mut self) -> DieResult {
        let mut result = DieResult::new();
        let mut die_roll = self.engine_roll();
        let mut compounding_total = 0;
        match self.comparison_mode {
            ComparisonMode::Equal(target) => loop {
                if die_roll != target {
                    compounding_total += die_roll;
                    result.dice_rolls.push(compounding_total);
                    break;
                }
                compounding_total += die_roll;
                die_roll = self.engine_roll();
            },
            ComparisonMode::LessThan(target) => loop {
                if die_roll >= target {
                    compounding_total += die_roll;
                    result.dice_rolls.push(compounding_total);
                    break;
                }
                compounding_total += die_roll;
                die_roll = self.engine_roll();
            },
            ComparisonMode::GreaterThan(target) => loop {
                if die_roll <= target {
                    compounding_total += die_roll;
                    result.dice_rolls.push(compounding_total);
                    break;
                }
                compounding_total += die_roll;
                die_roll = self.engine_roll();
            },
        }
        result
    }

    /// Roll a die and add another die subtracting 1 based on criteria
    fn penetrating(&mut self) -> DieResult {
        let mut result = DieResult::new();
        let mut die_roll = self.engine_roll();
        let mut penetrated = false;
        match self.comparison_mode {
            ComparisonMode::Equal(target) => loop {
                if die_roll != target {
                    if penetrated {
                        die_roll -= 1
                    }
                    result.dice_rolls.push(die_roll);
                    break;
                }
                if penetrated {
                    die_roll -= 1;
                }
                penetrated = true;
                result.dice_rolls.push(die_roll);
                die_roll = self.engine_roll();
            },
            ComparisonMode::LessThan(target) => loop {
                if die_roll >= target {
                    if penetrated {
                        die_roll -= 1;
                    }
                    result.dice_rolls.push(die_roll);
                    break;
                }
                if penetrated {
                    die_roll -= 1;
                }
                penetrated = true;
                result.dice_rolls.push(die_roll);
                die_roll = self.engine_roll();
            },
            ComparisonMode::GreaterThan(target) => loop {
                if die_roll <= target {
                    if penetrated {
                        die_roll -= 1;
                    }
                    result.dice_rolls.push(die_roll);
                    break;
                }
                if penetrated {
                    die_roll -= 1;
                }
                penetrated = true;
                result.dice_rolls.push(die_roll);
                die_roll = self.engine_roll();
            },
        }
        result
    }

    /// Roll a die, if the die fails criteria return one; otherwise leave dice_rolls empty
    fn failure(&mut self) -> DieResult {
        let mut result = DieResult::new();
        let die_roll = self.engine_roll();
        match self.comparison_mode {
            ComparisonMode::Equal(target) => {
                if die_roll != target {
                    result.dice_rolls.push(1);
                }
            }
            ComparisonMode::LessThan(target) => {
                if die_roll >= target {
                    result.dice_rolls.push(1);
                }
            }
            ComparisonMode::GreaterThan(target) => {
                if die_roll <= target {
                    result.dice_rolls.push(1);
                }
            }
        }
        result
    }

    /// Roll a die, if the die matches criteria return one; otherwise leave dice_rolls empty
    fn success(&mut self) -> DieResult {
        let mut result = DieResult::new();
        let die_roll = self.engine_roll();
        match self.comparison_mode {
            ComparisonMode::Equal(target) => {
                if die_roll == target {
                    result.dice_rolls.push(1);
                }
            }
            ComparisonMode::LessThan(target) => {
                if die_roll < target {
                    result.dice_rolls.push(1);
                }
            }
            ComparisonMode::GreaterThan(target) => {
                if die_roll > target {
                    result.dice_rolls.push(1);
                }
            }
        }
        result
    }

    /// Makes a call to the rng engine to generate a number
    fn engine_roll(&mut self) -> i64 {
        self.engine.random(1, self.sides)
    }

    pub fn roll(&mut self) -> DieResult {
        match self.roll_mode {
            RollMode::Normal => self.normal(),
            RollMode::Reroll => self.reroll(),
            RollMode::Exploding => self.exploding(),
            RollMode::Compounding => self.compounding(),
            RollMode::Penetrating => self.penetrating(),
            RollMode::Failure => self.failure(),
            RollMode::Success => self.success(),
        }
    }
}

impl Default for Die {
    /// Creates a default die with 20 sides
    fn default() -> Die {
        Die::new(
            RollMode::Normal,
            ComparisonMode::Equal(20),
            Box::new(PrngEngine::new()),
            20,
        )
    }
}
