/// Trait used to build custom RNG engines for die
pub trait Engine {
    /// Generate a random number
    fn random(&mut self, min: i64, max: i64) -> i64;
}
