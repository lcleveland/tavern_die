pub trait Engine {
    fn random(&self, min: i64, max: i64) -> i64;
}
