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
        where Self: Copy + std::convert::Into<u64>
    {
        let converted: u64 = (*self).into();
        match (converted != 0) && ((converted & (converted - 1)) == 0) {
            true => {Ok(*self)},
            false => {Err(*self)}
        }
    }
}

impl TryPowerOfTwo for u32{}
impl TryPowerOfTwo for u64{}
