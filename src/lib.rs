pub mod cards;
pub mod game;
pub enum Compliancy {
    Strict,
    Minimum(f64),
    Maximum(f64),
}
impl Compliancy {
    fn passed(self, accuracy: f64) -> bool {
        match self {
            Compliancy::Strict => accuracy == 100 as f64,
            Compliancy::Maximum(max) => accuracy < max,
            Compliancy::Minimum(min) => accuracy > min,
        }
    }
}
