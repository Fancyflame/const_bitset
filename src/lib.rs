use std::borrow::{Borrow, BorrowMut};

use div32::Div32;

pub mod div32;
pub mod ext_arr;

#[derive(Default)]
pub struct Zero;

#[derive(Default)]
pub struct Succ<T>(T);

pub trait Array: Borrow<[u32]> + BorrowMut<[u32]> + Sized {
    const LENGTH: usize;
}

impl<const N: usize> Array for [u32; N] {
    const LENGTH: usize = N;
}

// implements addition (+)

pub trait Addition {
    type Add<Rhs>;
}

impl Addition for Zero {
    type Add<Rhs> = Rhs;
}

impl<T> Addition for Succ<T>
where
    T: Addition,
{
    type Add<Rhs> = Succ<T::Add<Rhs>>;
}

pub trait Occupations {
    type Count: Div32;
}

pub trait ConstBitset {
    type Bitset: Array;
}

impl<T> ConstBitset for T
where
    T: Occupations,
{
    type Bitset = <T::Count as Div32>::Grouped;
}
