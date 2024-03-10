#![recursion_limit = "256"]

use const_bitset::{div32::Div32, Addition, ConstBitset, Occupations, Succ, Zero};

type Add8<T> = Succ<Succ<Succ<Succ<Succ<Succ<Succ<Succ<T>>>>>>>>;
type Add32<T> = Add8<Add8<Add8<Add8<T>>>>;

struct Chain<A, B>(A, B);

impl<A, B> Occupations for Chain<A, B>
where
    A: Occupations,
    A::Count: Addition,
    <A::Count as Addition>::Add<B::Count>: Div32,
    B: Occupations,
{
    type Count = <A::Count as Addition>::Add<B::Count>;
}

struct A24;

impl Occupations for A24 {
    type Count = Add8<Add8<Add8<Zero>>>;
}

struct A32;

impl Occupations for A32 {
    type Count = Add32<Zero>;
}

fn main() {
    type Chained = Chain<A24, Chain<A32, A24>>; // Count = 24 + (32 + 24) = 80
    let _: fn([u32; 3]) = drop::<<Chained as ConstBitset>::Bitset>; // ceil(80 / 32) = 3
}
