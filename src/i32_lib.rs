use crate::EucRes;

pub struct I32;

impl I32 {
    pub fn euc_ext(mut d1: i32, mut d2: i32) -> EucRes<i32> {
        let (mut spp, mut sp) = (1, 0);
        let (mut tpp, mut tp) = (0, 1);
        let mut q = d1 / d2;

        loop {
            (d1, d2) = (d2, d1 % d2);
            if d2 == 0 {
                break;
            }
            (spp, sp) = (sp, spp - (q * sp));
            (tpp, tp) = (tp, tpp - (q * tp));
            q = d1 / d2;
        }

        return EucRes {
            d: d1,
            s: sp,
            t: tp,
        };
    }

    pub fn euc(mut d1: i32, mut d2: i32) -> i32 {
        while d2 != 0 {
            (d1, d2) = (d2, d1 % d2);
        }
        return d1;
    }

    pub fn euc_recursive(mut d1: i32, mut d2: i32) -> i32 {
        if d1 > d2 {
            (d1, d2) = (d2, d1)
        }
        if d1 % d2 == 0 {
            return d2;
        } else {
            Self::euc_recursive(d2, d1 % d2)
        }
    }

    pub fn euc_from_vec(mut d: Vec<i32>) -> Result<i32, String> {
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

    pub fn lcm(d1: i32, d2: i32) -> i32 {
        i32::abs(d1 * d2) / Self::euc(d1, d2)
    }

    pub fn lcm_recursive(d1: i32, d2: i32) -> i32 {
        i32::abs(d1 * d2) / Self::euc_recursive(d1, d2)
    }

    pub fn lcm_from_vec(mut d: Vec<i32>) -> Result<i32, String> {
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

    pub fn congruence(mut a: i32, mut b: i32, n: i32) -> Result<i32, String> {
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