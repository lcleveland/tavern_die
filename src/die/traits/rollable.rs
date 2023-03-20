use crate::die::roll_result::RollResult;

pub trait Rollable {
    pub fn roll(&self) -> RollResult;
}
