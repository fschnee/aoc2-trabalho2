pub mod cache;
pub mod full;
pub mod regular;

pub trait TryPowerOfTwo {
    fn try_power_of_two(&self) -> Result<Self, Self>
    where
        Self: Copy + std::convert::Into<usize>,
    {
        let converted: usize = (*self).into();
        if (converted != 0) && ((converted & (converted - 1)) == 0) {
            Ok(*self)
        } else {
            Err(*self)
        }
    }
}

impl TryPowerOfTwo for usize {}

pub enum Either<T1, T2> {
    Left(T1),
    Right(T2),
}
