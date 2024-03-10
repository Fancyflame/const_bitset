# Const Bitset

Automatic infer bitset size for type

```rust
type Add8<T> = Succ<Succ<Succ<Succ<Succ<Succ<Succ<Succ<T>>>>>>>>;
type Add32<T> = Add8<Add8<Add8<Add8<T>>>>;

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
```