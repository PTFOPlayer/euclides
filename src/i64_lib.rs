use crate::*;

pub struct I64;

impl EucExt<i64> for I64{}
impl Euc<i64> for I64 {}
impl EucRecursive<i64> for I64 {}

impl I64 {
    pub fn euc_from_vec(mut d: Vec<i64>) -> Result<i64, String> {
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

    pub fn lcm(d1: i64, d2: i64) -> i64 {
        i64::abs(d1 * d2) / Self::euc(d1, d2)
    }

    pub fn lcm_recursive(d1: i64, d2: i64) -> i64 {
        i64::abs(d1 * d2) / Self::euc_recursive(d1, d2)
    }

    pub fn lcm_from_vec(mut d: Vec<i64>) -> Result<i64, String> {
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

    pub fn congruence(mut a: i64, mut b: i64, n: i64) -> Result<i64, String> {
        a = a % n;
        b = b % n;
        let ecr = Self::euc_ext(a, n);

        if b % ecr.d != 0 {
            return Err("Error occured b%euc_ext(a,n) != 0".to_owned());
        }
        let x = (ecr.s * (b / ecr.d)) % n;
        if x > 0 {
            return Ok(x);
        }
        return Ok(x + n);
    }
}