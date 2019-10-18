pub mod full;
pub mod regular;

#[derive(Debug, PartialEq)]
pub enum ReplacementPolicy{
    Lru,
    Fifo,
    Random
}

pub trait TryPowerOfTwo {
    fn try_power_of_two(&self) -> Result<Self, Self>
        where Self: std::marker::Sized;
}

impl TryPowerOfTwo for u32 {
    fn try_power_of_two(&self) -> Result<Self, Self> {
        match (*self != 0) && ((*self & (*self - 1)) == 0) {
            true => {Ok(*self)},
            false => {Err(*self)}
        }
    }
}
