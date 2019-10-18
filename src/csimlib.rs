pub mod full;
pub mod regular;

#[derive(Debug, PartialEq)]
pub enum ReplacementPolicy{
    Lru,
    Fifo,
    Random
}
