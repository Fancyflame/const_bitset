use crate::{ext_arr::ExtArr, Succ, Zero};

macro_rules! to_nested{
    ()=>{Zero};
    ($T:ident $($T2:ident)*)=>{
        $T<to_nested!($($T2)*)>
    }
}

macro_rules! impl_div32 {
    ()=>{};
    ($T:ident $($T2:ident)*)=>{
        impl Div32 for to_nested!($T $($T2)*) {
            type Grouped = [u32; 1];
        }

        impl_div32!($($T2)*);
    };
}

pub trait Div32 {
    type Grouped: ExtArr;
}

type Add8<T> = Succ<Succ<Succ<Succ<Succ<Succ<Succ<Succ<T>>>>>>>>;

// every 32 successors extends one time array
impl<T> Div32 for Add8<Add8<Add8<Add8<T>>>>
where
    T: Div32,
    <T::Grouped as ExtArr>::Extended: ExtArr,
{
    type Grouped = <T::Grouped as ExtArr>::Extended;
}

// 1~31 occupy 1 u32
impl_div32!(
    Succ Succ Succ Succ Succ Succ Succ Succ
    Succ Succ Succ Succ Succ Succ Succ Succ
    Succ Succ Succ Succ Succ Succ Succ Succ
    Succ Succ Succ Succ Succ Succ Succ
);

// 0 occupies nothing
impl Div32 for Zero {
    type Grouped = [u32; 0];
}
