use crate::Array;

pub trait ExtArr: Array {
    type Extended: Array;
}

macro_rules! ext_arr {
    ()=>{};
    (+ $($tt:tt)*)=>{
        impl ExtArr for [u32; 0 $($tt 1)*] {
            type Extended = [u32; 1 $($tt 1)*];
        }
        ext_arr!($($tt)*);
    }
}

ext_arr! [
    ++++++++++++++++++++++++++++++++
    ++++++++++++++++++++++++++++++++
    ++++++++++++++++++++++++++++++++
    +++++++++++++++++++++++++++++++
];
