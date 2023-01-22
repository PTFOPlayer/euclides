#[derive(Debug)]
pub struct EucRes {
    pub d: i32,
    pub s: i32,
    pub t: i32,
}

pub fn euc_ext(d1: i32, d2: i32) -> EucRes {
    let mut d = vec![d1, d2];
    let mut s = vec![1, 0];
    let mut t = vec![0, 1];
    let mut qp = d1 / d2;

    let mut i = 2;
    while d2 != 0 {
        d.append(&mut vec![d[i - 2] % d[i - 1]]);
        if d[i] == 0 {
            break;
        }
        s.append(&mut vec![s[i - 2] - (qp * s[i - 1])]);
        t.append(&mut vec![t[i - 2] - (qp * t[i - 1])]);
        let q = d[i - 1] / d[i];
        qp = q;
        i += 1;
    }

    return EucRes {
        d: d[i - 1],
        s: s[i - 1],
        t: t[i - 1],
    };
}

pub fn euc(mut d1: i32, mut d2: i32) -> i32 {
    while d2 != 0 {
        let t = d2;
        d2 = d1 % d2;
        d1 = t.clone();
    }
    return d1;
}

pub fn euc_recursive(d1: i32, d2: i32) -> i32 {
    if d1 % d2 == 0 {
        return d2;
    } else {
        return euc_recursive(d2, d1 % d2);
    }
}

pub fn lcm(d1:i32, d2:i32) -> i32 {
    let mut temp = d1 * d2;
    if temp < 0 {
        temp = temp * -1;
    }
    return temp/euc(d1, d2);
}