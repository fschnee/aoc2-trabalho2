pub mod full;
pub mod regular;
pub mod cache;

pub trait TryPowerOfTwo {
    fn try_power_of_two(&self) -> Result<Self, Self>
        where Self: Copy + std::convert::Into<usize>
    {
        let converted: usize = (*self).into();
        match (converted != 0) && ((converted & (converted - 1)) == 0) {
            true => {Ok(*self)},
            false => {Err(*self)}
        }
    }
}

impl TryPowerOfTwo for usize {}
