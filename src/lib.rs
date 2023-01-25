#[derive(Debug)]
pub struct EucRes {
    pub d: i32,
    pub s: i32,
    pub t: i32,
}

pub fn euc_ext(mut d1: i32, mut d2: i32) -> EucRes {
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
        (d1,d2) =(d2,d1) 
    }
    if d1 % d2 == 0 {
        return d2;
    } else {
        return euc_recursive(d2, d1 % d2);
    }
}

pub fn euc_from_vec(mut d: Vec<i32>) -> Result<i32, String> {
    if d.len() == 2 {
        return Ok(euc(d[0], d[1]));
    } else if d.len() < 2{
        return Err("critical error occured, length of vector smaller than 2, unable to calculate euc".to_owned());
    }
    Ok(euc(d.pop().expect("error occured calculating euc"), euc_from_vec(d.clone()).unwrap()))
}

pub fn lcm(d1: i32, d2: i32) -> i32 {
    return i32::abs(d1 * d2) / euc(d1, d2);
}

pub fn lcm_recursive(d1: i32, d2: i32) -> i32 {
    return i32::abs(d1 * d2) / euc_recursive(d1, d2);
}

pub fn lcm_from_vec(mut d: Vec<i32>) -> Result<i32, String> {
    if d.len() == 2 {
        return Ok(lcm(d[0], d[1]));
    } else if d.len() < 2{
        return Err("critical error occured, length of vector smaller than 2, unable to calculate lcm".to_owned());
    }
    Ok(lcm(d.pop().expect("error occured calculating lcm"), lcm_from_vec(d.clone()).unwrap()))
}
