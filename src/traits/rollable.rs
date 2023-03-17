use crate::die::roll_result::RollResult;

pub trait rollable {
    fn roll(&self) -> RollResult;
}
