/// Inductive or coinductive?
#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash)]
pub enum Ductive {
    In,
    Coin,
}
