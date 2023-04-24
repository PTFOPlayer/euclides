use crate::EucRes;
extern crate num;
use num::{ One, Zero, Signed};

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

    fn euc_from_vec(mut d: Vec<T>) -> Result<T, String> {
        if d.len() == 2 {
            return Ok(Self::euc(d[0], d[1]));
        } else if d.len() < 2 {
            return Err(
                "critical error occured, length of vector smaller than 2, unable to calculate euc"
                    .to_owned(),
            );
        }
        Ok(Self::euc(
            d.pop().expect("error occured calculating euc"),
            match Self::euc_from_vec(d.clone()) {
                Ok(res) => res,
                Err(err) => return Err(err),
            },
        ))
    }

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

pub trait Lcm<T>: self::Euc<T> + self::Euc<T>
where
    T: std::ops::Mul<Output = T> + std::ops::Rem<Output = T> + Zero + std::cmp::Ord + Copy + Signed,
{
    fn lcm(d1: T, d2: T) -> T {
        T::abs(&(d1 * d2)) / Self::euc(d1, d2)
    }

    fn lcm_recursive(d1: T, d2: T) -> T {
        T::abs(&(d1 * d2)) / Self::euc_recursive(d1, d2)
    }

    fn lcm_from_vec(mut d: Vec<T>) -> Result<T, String> {
        if d.len() == 2 {
            return Ok(Self::lcm(d[0], d[1]));
        } else if d.len() < 2 {
            return Err(
                "critical error occured, length of vector smaller than 2, unable to calculate lcm"
                    .to_owned(),
            );
        }
        Ok(Self::lcm(
            d.pop().expect("error occured calculating lcm"),
            match Self::lcm_from_vec(d.clone()) {
                Ok(res) => res,
                Err(err) => return Err(err),
            },
        ))
    }
}

pub trait Congruence<T>: self::EucExt<T>
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
    fn congruence(mut a: T, mut b: T, n: T) -> Result<T, String> {
        a = a % n;
        b = b % n;
        let ecr = Self::euc_ext(a, n);

        if b % ecr.d != T::zero() {
            return Err("Error occured b%euc_ext(a,n) != 0".to_owned());
        }
        let x = (ecr.s * (b / ecr.d)) % n;
        if x > T::zero() {
            return Ok(x);
        }
        return Ok(x + n);
    }
}
