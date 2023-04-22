use crate::EucRes;
extern crate num;
use num::{One, Zero};
pub trait EucExt<T>
where
    T: One
        + Zero
        + Copy
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + std::ops::Rem<Output = T>
        + std::ops::Sub<Output = T>
        + std::cmp::Ord,
{
    fn euc_ext(mut d1: T, mut d2: T) -> EucRes<T> {
        let (mut spp, mut sp) = (T::one(), T::zero());
        let (mut tpp, mut tp) = (T::zero(), T::one());
        let mut q = d1 / d2;

        loop {
            (d1, d2) = (d2, d1 % d2);
            if d2 == T::zero() {
                break;
            }
            (spp, sp, tpp, tp) = (sp, spp - (q * sp), tp, tpp - (q * tp));
            q = d1 / d2;
        }

        return EucRes {
            d: d1,
            s: sp,
            t: tp,
        };
    }
}

pub trait Euc<T>
where
    T: std::ops::Rem<Output = T> + Zero + std::cmp::Ord + Copy,
{
    fn euc(mut d1: T, mut d2: T) -> T {
        while d2 != T::zero() {
            (d1, d2) = (d2, d1 % d2);
        }
        return d1;
    }
}

pub trait EucRecursive<T>
where
    T: Zero + Copy + std::ops::Rem<Output = T> + std::cmp::Ord,
{
    fn euc_recursive(mut d1: T, mut d2: T) -> T {
        if d1 > d2 {
            (d1, d2) = (d2, d1)
        }
        if d1 % d2 == T::zero() {
            return d2;
        } else {
            Self::euc_recursive(d2, d1 % d2)
        }
    }
}
