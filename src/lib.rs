use std::fmt;

#[derive(Debug)]
pub struct EucRes<T> {
    pub d: T,
    pub s: T,
    pub t: T,
}

impl<T> fmt::Display for EucRes<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NWD = {}, S = {}, T = {}", self.d, self.s, self.t)
    }
}

mod i128_lib;
mod i32_lib;
mod i64_lib;
mod traits;

pub use i128_lib::*;
pub use i32_lib::*;
pub use i64_lib::*;
pub use traits::*;
