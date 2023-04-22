use std::fmt;

#[derive(Debug)]
pub struct EucRes<T> {
    pub d: T,
    pub s: T,
    pub t: T,
}

impl fmt::Display for EucRes<i32> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NWD = {}, S = {}, T = {}", self.d, self.s, self.t)
    }
}

impl fmt::Display for EucRes<i64> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NWD = {}, S = {}, T = {}", self.d, self.s, self.t)
    }
}

impl fmt::Display for EucRes<i128> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NWD = {}, S = {}, T = {}", self.d, self.s, self.t)
    }
}

mod traits;
pub use traits::*;

mod i128_lib;
mod i32_lib;
mod i64_lib;
pub use i128_lib::*;
pub use i32_lib::*;
pub use i64_lib::*;
