use crate::die::roll_result::RollResult;

pub trait Rollable {
    fn roll(&self) -> RollResult;
}
