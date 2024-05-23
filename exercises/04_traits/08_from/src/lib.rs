// TODO: Implement the `From` trait for the `u32` type to make `example` compile.

pub struct WrappingU32 {
    value: u32,
}

impl From<u32> for WrappingU32 {
    fn from(val: u32) -> WrappingU32 {
        WrappingU32{ value: val}
    }
}

fn example() {
    let wrapping: WrappingU32 = 42.into();
    let wrapping = WrappingU32::from(42);
}
